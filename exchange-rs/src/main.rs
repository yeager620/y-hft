mod matching_engine;
mod optimizations;
mod order;
mod orderbook;

mod metrics;

mod snapshot;

use parking_lot::Mutex;
use std::sync::Arc;

use matching_engine::MatchingEngine;
use optimizations::{OrderPool, OrderProcessorPool};
use order::{Order, OrderType, Side};

fn main() {
    println!("Exchange-RS: High-performance limit order book implementation");

    let engine = Arc::new(Mutex::new(MatchingEngine::new()));

    {
        let mut engine_ref = engine.lock();
        engine_ref.add_symbol("AAPL");
    }

    let num_workers = num_cpus::get();
    println!("Starting order processor pool with {} workers", num_workers);
    let pool = OrderProcessorPool::new(num_workers, Arc::clone(&engine));

    let order_pool = OrderPool::new(1000);
    println!("Created order pool with initial capacity of 1000 orders");

    println!("\nSubmitting orders...");

    let sell_order = Order::new("AAPL".to_string(), Side::Sell, OrderType::Limit, 100, 10, 1);
    println!("Submitting sell order: 10 shares of AAPL at $100");
    if let Err(e) = pool.submit_order(sell_order) {
        eprintln!("Error submitting sell order: {}", e);
    }

    std::thread::sleep(std::time::Duration::from_millis(100));

    let buy_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 5, 2);
    println!("Submitting buy order: 5 shares of AAPL at $100");
    if let Err(e) = pool.submit_order(buy_order) {
        eprintln!("Error submitting buy order: {}", e);
    }

    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut stop_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::StopLimit,
        110,
        10,
        3,
    );
    stop_order.stop_price = Some(105);
    println!("Submitting stop buy order: 10 shares of AAPL at $110, stop price $105");
    if let Err(e) = pool.submit_order(stop_order) {
        eprintln!("Error submitting stop order: {}", e);
    }

    std::thread::sleep(std::time::Duration::from_millis(100));

    let sell_order_2 = Order::new("AAPL".to_string(), Side::Sell, OrderType::Limit, 105, 5, 4);
    println!("Submitting sell order: 5 shares of AAPL at $105 (should trigger stop order)");
    if let Err(e) = pool.submit_order(sell_order_2) {
        eprintln!("Error submitting sell order: {}", e);
    }

    std::thread::sleep(std::time::Duration::from_millis(500));

    println!("\nAll orders processed. Check the output above for any errors.");
    println!("Exchange-RS demo completed successfully!");
}
