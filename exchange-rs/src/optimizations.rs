use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::queue::ArrayQueue;
use crossbeam_utils::CachePadded;
use parking_lot::{Mutex, RwLock};
use rayon::ThreadPoolBuilder;

use crate::matching_engine::MatchingEngine;
use crate::order::Order;

#[allow(dead_code)]
const CACHE_LINE_SIZE: usize = 64;

pub struct OrderPool {
    free_list: Mutex<Vec<Arc<RwLock<Order>>>>,
    total_allocated: Mutex<usize>,
}

impl OrderPool {
    pub fn new(initial_capacity: usize) -> Self {
        let mut free_list = Vec::with_capacity(initial_capacity);

        for _ in 0..initial_capacity {
            let order = Arc::new(RwLock::new(Order::new(
                String::new(),
                crate::order::Side::Buy,
                crate::order::OrderType::Limit,
                0,
                0,
                0,
            )));
            free_list.push(order);
        }

        Self {
            free_list: Mutex::new(free_list),
            total_allocated: Mutex::new(initial_capacity),
        }
    }

    pub fn acquire(&self) -> Arc<RwLock<Order>> {
        let mut free_list = self.free_list.lock();

        if let Some(order) = free_list.pop() {
            let mut order_ref = order.write();
            *order_ref = Order::new(
                String::new(),
                crate::order::Side::Buy,
                crate::order::OrderType::Limit,
                0,
                0,
                0,
            );
            drop(order_ref);
            return order;
        }

        let mut total = self.total_allocated.lock();
        *total += 1;
        Arc::new(RwLock::new(Order::new(
            String::new(),
            crate::order::Side::Buy,
            crate::order::OrderType::Limit,
            0,
            0,
            0,
        )))
    }

    pub fn release(&self, order: Arc<RwLock<Order>>) {
        let mut guard = self.free_list.lock();
        guard.push(order);
    }

    pub fn get_total_allocated(&self) -> usize {
        *self.total_allocated.lock()
    }
}

pub struct SPSCQueue {
    queue: ArrayQueue<Order>,
}

impl SPSCQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: ArrayQueue::new(capacity),
        }
    }

    pub fn enqueue(&self, order: Order) -> Result<(), &'static str> {
        self.queue.push(order).map_err(|_| "Queue is full")
    }

    pub fn dequeue(&self) -> Option<Order> {
        self.queue.pop()
    }
}

pub struct CacheAlignedPriceLevel {
    price: CachePadded<u64>,
    total_volume: CachePadded<u64>,
    visible_volume: CachePadded<u64>,
    orders: Mutex<Vec<Arc<RwLock<Order>>>>,
}

impl CacheAlignedPriceLevel {
    pub fn new(price: u64) -> Self {
        Self {
            price: CachePadded::new(price),
            total_volume: CachePadded::new(0),
            visible_volume: CachePadded::new(0),
            orders: Mutex::new(Vec::new()),
        }
    }

    pub fn add_order(&mut self, order: Arc<RwLock<Order>>) {
        let order_ref = order.read();
        *self.total_volume += order_ref.remaining_quantity() as u64;
        *self.visible_volume += order_ref.visible_quantity() as u64;
        drop(order_ref);

        let mut orders = self.orders.lock();
        orders.push(order);
    }

    pub fn remove_order(&mut self, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        let mut orders = self.orders.lock();
        let position = orders.iter().position(|o| o.read().id == order_id)?;
        let order = orders.remove(position);

        let remaining_qty;
        let visible_qty;
        {
            let order_ref = order.read();
            remaining_qty = order_ref.remaining_quantity();
            visible_qty = order_ref.visible_quantity();
        }

        *self.total_volume -= remaining_qty as u64;
        *self.visible_volume -= visible_qty as u64;

        Some(order)
    }

    pub fn get_total_volume(&self) -> u64 {
        *self.total_volume
    }

    pub fn get_visible_volume(&self) -> u64 {
        *self.visible_volume
    }

    pub fn get_price(&self) -> u64 {
        *self.price
    }
}

pub struct OrderProcessorPool {
    workers: Vec<Worker>,
    next_worker: std::sync::atomic::AtomicUsize,
}

struct Worker {
    queue: Arc<SPSCQueue>,
    thread: Option<thread::JoinHandle<()>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl OrderProcessorPool {
    pub fn new(num_workers: usize, engine: Arc<Mutex<MatchingEngine>>) -> Self {
        let mut workers = Vec::with_capacity(num_workers);

        for _ in 0..num_workers {
            let queue = Arc::new(SPSCQueue::new(1024));
            let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));

            let worker_queue = Arc::clone(&queue);
            let worker_stop = Arc::clone(&stop);
            let worker_engine = Arc::clone(&engine);

            let thread = thread::spawn(move || {
                Self::worker_fn(worker_queue, worker_stop, worker_engine);
            });

            workers.push(Worker {
                queue,
                thread: Some(thread),
                stop,
            });
        }

        Self {
            workers,
            next_worker: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn worker_fn(
        queue: Arc<SPSCQueue>,
        stop: Arc<std::sync::atomic::AtomicBool>,
        engine: Arc<Mutex<MatchingEngine>>,
    ) {
        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            if let Some(order) = queue.dequeue() {
                let mut engine = engine.lock();
                if let Err(e) = engine.place_order(order) {
                    eprintln!("Error processing order: {}", e);
                }
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }

    pub fn submit_order(&self, order: Order) -> Result<(), &'static str> {
        let worker_idx = self
            .next_worker
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            % self.workers.len();

        self.workers[worker_idx].queue.enqueue(order)
    }
}

impl Drop for OrderProcessorPool {
    fn drop(&mut self) {
        for worker in &self.workers {
            worker
                .stop
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

pub struct ThreadPool {
    pool: rayon::ThreadPool,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<Self, rayon::ThreadPoolBuildError> {
        let pool = ThreadPoolBuilder::new().num_threads(num_threads).build()?;

        Ok(Self { pool })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pool.spawn(f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matching_engine::MatchingEngine;
    use crate::order::{Order, OrderType, Side};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_order_pool() {
        let pool = OrderPool::new(10);

        let mut orders = Vec::new();
        for i in 0..15 {
            let order = pool.acquire();
            {
                let mut order_ref = order.write();
                order_ref.id = i;
                order_ref.symbol = "AAPL".to_string();
                order_ref.price = 100;
                order_ref.quantity = 10;
            }
            orders.push(order);
        }

        for order in orders {
            pool.release(order);
        }
    }

    #[test]
    fn test_order_pool_reuse() {
        let pool = OrderPool::new(5);

        for _ in 0..3 {
            let mut orders = Vec::new();
            for i in 0..5 {
                let order = pool.acquire();
                {
                    let mut order_ref = order.write();
                    order_ref.id = i;
                    order_ref.symbol = "AAPL".to_string();
                    order_ref.price = 100;
                    order_ref.quantity = 10;
                }
                orders.push(order);
            }

            for order in orders {
                pool.release(order);
            }
        }

        assert_eq!(pool.get_total_allocated(), 5);
    }

    #[test]
    fn test_order_pool_concurrent() {
        let pool = Arc::new(OrderPool::new(100));
        let num_threads = 10;
        let orders_per_thread = 100;

        let total_orders = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for _ in 0..num_threads {
            let pool_clone = Arc::clone(&pool);
            let counter = Arc::clone(&total_orders);

            let handle = thread::spawn(move || {
                let mut thread_orders = Vec::new();

                for i in 0..orders_per_thread {
                    let order = pool_clone.acquire();
                    {
                        let mut order_ref = order.write();
                        order_ref.id = i as u64;
                        order_ref.symbol = "AAPL".to_string();
                        order_ref.price = 100;
                        order_ref.quantity = 10;
                    }
                    thread_orders.push(order);
                    counter.fetch_add(1, Ordering::SeqCst);
                }

                for order in thread_orders {
                    pool_clone.release(order);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(
            total_orders.load(Ordering::SeqCst),
            num_threads * orders_per_thread
        );
    }

    #[test]
    fn test_spsc_queue() {
        let queue = SPSCQueue::new(10);

        for i in 0..5 {
            let order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
            queue.enqueue(order).unwrap();
        }

        for _ in 0..5 {
            let order = queue.dequeue();
            assert!(order.is_some());
        }

        assert!(queue.dequeue().is_none());
    }

    #[test]
    fn test_spsc_queue_full() {
        let queue = SPSCQueue::new(2);

        for i in 0..2 {
            let order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, i);
            queue.enqueue(order).unwrap();
        }

        let order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 3);
        let result = queue.enqueue(order);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Queue is full");
    }

    #[test]
    fn test_spsc_queue_producer_consumer() {
        let queue = Arc::new(SPSCQueue::new(100));
        let num_orders = 100;

        let queue_clone = Arc::clone(&queue);
        let producer = thread::spawn(move || {
            for i in 0..num_orders {
                let order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, i);
                queue_clone.enqueue(order).unwrap();
            }
        });

        let queue_clone = Arc::clone(&queue);
        let consumer = thread::spawn(move || {
            let mut count = 0;
            while count < num_orders {
                if let Some(_) = queue_clone.dequeue() {
                    count += 1;
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            count
        });

        producer.join().unwrap();
        let processed = consumer.join().unwrap();

        assert_eq!(processed, num_orders);
    }

    #[test]
    fn test_cache_aligned_price_level() {
        let mut level = CacheAlignedPriceLevel::new(100);

        let mut order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
        order.id = 1;

        let order_arc = Arc::new(RwLock::new(order));

        level.add_order(Arc::clone(&order_arc));

        assert_eq!(level.get_total_volume(), 10);

        let removed = level.remove_order(1);
        assert!(removed.is_some());

        assert_eq!(level.get_total_volume(), 0);
    }

    #[test]
    fn test_cache_aligned_price_level_concurrent() {
        let mut level = CacheAlignedPriceLevel::new(100);
        let num_threads = 10;
        let orders_per_thread = 10;

        for t in 0..num_threads {
            for i in 0..orders_per_thread {
                let id = (t * orders_per_thread + i) as u64;
                let mut order =
                    Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, id);
                order.id = id;

                let order_arc = Arc::new(RwLock::new(order));
                level.add_order(Arc::clone(&order_arc));
            }
        }

        assert_eq!(
            level.get_total_volume(),
            (num_threads * orders_per_thread * 10) as u64
        );
    }

    #[test]
    fn test_order_processor_pool() {
        let engine = Arc::new(Mutex::new(MatchingEngine::new()));

        {
            let mut engine_ref = engine.lock();
            engine_ref.add_symbol("AAPL");
        }

        let pool = OrderProcessorPool::new(2, Arc::clone(&engine));

        let sell_order = Order::new("AAPL".to_string(), Side::Sell, OrderType::Limit, 100, 10, 1);
        pool.submit_order(sell_order).unwrap();

        thread::sleep(Duration::from_millis(50));

        let buy_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 5, 2);
        pool.submit_order(buy_order).unwrap();

        thread::sleep(Duration::from_millis(50));

        let engine_ref = engine.lock();
        let order_book = engine_ref.order_books.get("AAPL").unwrap();

        let sell_order = order_book.get_order(1).unwrap();
        assert_eq!(sell_order.read().filled_quantity, 5);
        assert_eq!(sell_order.read().remaining_quantity(), 5);
    }

    #[test]
    fn test_order_processor_pool_concurrent() {
        let engine = Arc::new(Mutex::new(MatchingEngine::new()));

        {
            let mut engine_ref = engine.lock();
            engine_ref.add_symbol("AAPL");
        }

        let pool = OrderProcessorPool::new(4, Arc::clone(&engine));
        let processed = Arc::new(AtomicUsize::new(0));

        for i in 0..10 {
            let price = 100 + i;
            let sell_order = Order::new(
                "AAPL".to_string(),
                Side::Sell,
                OrderType::Limit,
                price,
                10,
                i,
            );
            pool.submit_order(sell_order).unwrap();
        }

        thread::sleep(Duration::from_millis(100));

        let mut all_orders_processed = false;
        for _ in 0..500 {
            let engine_ref = engine.lock();
            let order_book = engine_ref.order_books.get("AAPL").unwrap();
            let mut found_orders = 0;

            for i in 0..10 {
                if order_book.get_order(i).is_some() {
                    found_orders += 1;
                }
            }

            if found_orders == 10 {
                all_orders_processed = true;
                break;
            }
            drop(engine_ref);
            thread::sleep(Duration::from_millis(10));
        }

        assert!(all_orders_processed, "Not all sell orders were processed");

        for i in 0..10 {
            let price = 100 + i;
            let buy_order = Order::new(
                "AAPL".to_string(),
                Side::Buy,
                OrderType::Limit,
                price,
                5,
                i + 100,
            );
            pool.submit_order(buy_order).unwrap();
        }

        let mut all_trades_processed = false;
        for _ in 0..100 {
            let engine_ref = engine.lock();
            let order_book = engine_ref.order_books.get("AAPL").unwrap();
            let mut completed_trades = 0;

            for i in 0..10 {
                if let Some(order) = order_book.get_order(i) {
                    if order.read().filled_quantity == 5 {
                        completed_trades += 1;
                    }
                }
            }

            if completed_trades == 10 {
                all_trades_processed = true;
                break;
            }
            drop(engine_ref);
            thread::sleep(Duration::from_millis(10));
        }

        assert!(all_trades_processed, "Not all trades were processed");

        let engine_ref = engine.lock();
        let order_book = engine_ref.order_books.get("AAPL").unwrap();

        for i in 0..10 {
            let sell_order = order_book.get_order(i).unwrap();
            let filled = sell_order.read().filled_quantity;
            let remaining = sell_order.read().remaining_quantity();
            assert_eq!(filled, 5, "Order {} should have 5 shares filled", i);
            assert_eq!(remaining, 5, "Order {} should have 5 shares remaining", i);
        }
    }

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4).unwrap();

        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..100 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        thread::sleep(Duration::from_millis(10));

        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
}
