use std::collections::HashMap;
use std::sync::Arc;

use crate::order::{Order, OrderStatus, OrderType, Side};
use crate::snapshot::OrderBookSnapshot;
use crate::snapshot::{OrderSnapshot, PriceLevelSnapshot};
use crossbeam_utils::CachePadded;
use dashmap::DashMap;
use parking_lot::RwLock;

pub struct PriceLevel {
    price: u64,
    pub orders: Vec<Arc<RwLock<Order>>>,
    pub total_volume: u64,
    pub visible_volume: u64,
}

impl PriceLevel {
    pub fn new(price: u64) -> Self {
        Self {
            price,
            orders: Vec::new(),
            total_volume: 0,
            visible_volume: 0,
        }
    }

    pub fn add_order(&mut self, order: Arc<RwLock<Order>>) {
        let order_ref = order.read();
        self.total_volume += order_ref.remaining_quantity() as u64;
        self.visible_volume += order_ref.visible_quantity() as u64;
        drop(order_ref);
        self.orders.push(order);
    }

    pub fn remove_order(&mut self, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        let position = self.orders.iter().position(|o| o.read().id == order_id)?;
        let order = self.orders.remove(position);

        let remaining_qty;
        let visible_qty;
        {
            let order_ref = order.read();
            remaining_qty = order_ref.remaining_quantity();
            visible_qty = order_ref.visible_quantity();
        }

        self.total_volume -= remaining_qty as u64;
        self.visible_volume -= visible_qty as u64;

        Some(order)
    }

    pub fn update_visible_quantity(&mut self) {
        self.visible_volume = 0;
        for order in &self.orders {
            let order_ref = order.read();
            if let Some(display_qty) = order_ref.display_quantity {
                self.visible_volume +=
                    std::cmp::min(display_qty as u64, order_ref.remaining_quantity() as u64);
            } else {
                self.visible_volume += order_ref.remaining_quantity() as u64;
            }
        }
    }

    pub fn get_visible_volume(&self) -> u64 {
        self.visible_volume
    }

    pub fn update_after_trade(
        &mut self,
        order_id: u64,
        executed_qty: u32,
    ) -> Result<(), &'static str> {
        if let Some(order) = self.orders.iter().find(|o| o.read().id == order_id) {
            let mut order_ref = order.write();
            order_ref.filled_quantity += executed_qty;

            if let Some(display_qty) = order_ref.display_quantity {
                let remaining = order_ref.remaining_quantity() as u64;
                self.visible_volume = std::cmp::min(display_qty as u64, remaining);
            } else {
                self.visible_volume = self.visible_volume.saturating_sub(executed_qty as u64);
            }

            Ok(())
        } else {
            Err("Order not found")
        }
    }

    pub fn replenish_iceberg_order(&mut self, order_id: u64) -> Result<(), &'static str> {
        if let Some(position) = self.orders.iter().position(|o| o.read().id == order_id) {
            let order = &self.orders[position];
            let order_ref = order.read();

            if order_ref.order_type != OrderType::Iceberg {
                return Err("Not an iceberg order");
            }

            let display_qty = order_ref
                .display_quantity
                .ok_or("Missing display quantity")?;
            let remaining = order_ref.remaining_quantity();
            let new_visible = std::cmp::min(display_qty, remaining);

            self.visible_volume = new_visible as u64;

            Ok(())
        } else {
            Err("Order not found in price level")
        }
    }

    pub fn get_price(&self) -> u64 {
        self.price
    }
}

pub struct StopOrderBook {
    symbol: String,
    buy_stop_orders: HashMap<u64, Vec<Arc<RwLock<Order>>>>,
    sell_stop_orders: HashMap<u64, Vec<Arc<RwLock<Order>>>>,
    order_map: HashMap<u64, Arc<RwLock<Order>>>,
}

impl StopOrderBook {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            buy_stop_orders: HashMap::new(),
            sell_stop_orders: HashMap::new(),
            order_map: HashMap::new(),
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn add_stop_order(&mut self, order: Arc<RwLock<Order>>) -> Result<(), &'static str> {
        let order_ref = order.read();

        if !order_ref.is_stop_order() {
            return Err("Not a stop order");
        }

        let stop_price = order_ref.stop_price.ok_or("Missing stop price")?;
        let order_id = order_ref.id;
        let side = order_ref.side;

        drop(order_ref);

        self.order_map.insert(order_id, Arc::clone(&order));

        let orders_map = match side {
            Side::Buy => &mut self.buy_stop_orders,
            Side::Sell => &mut self.sell_stop_orders,
        };

        orders_map
            .entry(stop_price)
            .or_insert_with(Vec::new)
            .push(Arc::clone(&order));

        Ok(())
    }

    pub fn remove_stop_order(&mut self, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        let order = self.order_map.get(&order_id)?;
        let order_ref = order.read();

        if !order_ref.is_stop_order() {
            return None;
        }

        let stop_price = order_ref.stop_price?;
        let side = order_ref.side;

        drop(order_ref);

        let orders_map = match side {
            Side::Buy => &mut self.buy_stop_orders,
            Side::Sell => &mut self.sell_stop_orders,
        };

        if let Some(orders) = orders_map.get_mut(&stop_price) {
            let position = orders.iter().position(|o| o.read().id == order_id)?;
            let order = orders.remove(position);

            if orders.is_empty() {
                orders_map.remove(&stop_price);
            }

            self.order_map.remove(&order_id);
            return Some(order);
        }

        None
    }

    pub fn get_triggered_orders(&self, last_price: u64) -> Vec<Arc<RwLock<Order>>> {
        let mut triggered = Vec::new();

        for (&stop_price, orders) in &self.buy_stop_orders {
            if last_price >= stop_price {
                for order in orders {
                    triggered.push(Arc::clone(order));
                }
            }
        }

        for (&stop_price, orders) in &self.sell_stop_orders {
            if last_price <= stop_price {
                for order in orders {
                    triggered.push(Arc::clone(order));
                }
            }
        }

        triggered
    }

    pub fn remove_triggered_orders(&mut self, triggered_orders: &[Arc<RwLock<Order>>]) {
        for order in triggered_orders {
            let order_id = order.read().id;
            self.remove_stop_order(order_id);
        }
    }
}

#[derive(Default, Clone)]
pub struct MarketDepth {
    pub bid_levels: Vec<(u64, u64)>, 
    pub ask_levels: Vec<(u64, u64)>, 
}

pub struct OrderBook {
    symbol: String,
    pub buy_levels: HashMap<u64, PriceLevel>,
    pub sell_levels: HashMap<u64, PriceLevel>,
    order_map: HashMap<u64, Arc<RwLock<Order>>>,
    stop_order_book: StopOrderBook,
    pub last_trade_price: Option<u64>,
    depth: RwLock<MarketDepth>,
    depth_levels: usize, 
}

impl OrderBook {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            buy_levels: HashMap::new(),
            sell_levels: HashMap::new(),
            order_map: HashMap::new(),
            stop_order_book: StopOrderBook::new(symbol),
            last_trade_price: None,
            depth: RwLock::new(MarketDepth::default()),
            depth_levels: 10, 
        }
    }

    pub fn add_order(&mut self, order: Arc<RwLock<Order>>) -> Result<(), &'static str> {
        let order_ref = order.read();
        let order_id = order_ref.id;
        let price = order_ref.price;
        let side = order_ref.side;

        drop(order_ref);

        self.order_map.insert(order_id, Arc::clone(&order));

        let levels = match side {
            Side::Buy => &mut self.buy_levels,
            Side::Sell => &mut self.sell_levels,
        };

        let level = levels
            .entry(price)
            .or_insert_with(|| PriceLevel::new(price));
        level.add_order(Arc::clone(&order));

        self.update_depth();

        Ok(())
    }

    pub fn add_stop_order(&mut self, order: Arc<RwLock<Order>>) -> Result<(), &'static str> {
        let order_ref = order.read();

        if !order_ref.is_stop_order() {
            return Err("Not a stop order");
        }

        drop(order_ref);

        self.stop_order_book.add_stop_order(order)
    }

    pub fn remove_order(&mut self, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        if let Some(order) = self.order_map.get(&order_id) {
            let order_ref = order.read();
            let price = order_ref.price;
            let side = order_ref.side;

            drop(order_ref);

            let levels = match side {
                Side::Buy => &mut self.buy_levels,
                Side::Sell => &mut self.sell_levels,
            };

            if let Some(level) = levels.get_mut(&price) {
                if let Some(removed_order) = level.remove_order(order_id) {
                    self.order_map.remove(&order_id);

                    if level.orders.is_empty() {
                        levels.remove(&price);
                    }

                    self.update_depth();

                    return Some(removed_order);
                }
            }
        }

        self.stop_order_book.remove_stop_order(order_id)
    }

    pub fn cancel_order(&mut self, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        let result = self.remove_order(order_id);

        self.update_depth();

        result
    }

    pub fn get_best_bid_price(&self) -> Option<u64> {
        self.buy_levels.keys().max().copied()
    }

    pub fn get_best_ask_price(&self) -> Option<u64> {
        self.sell_levels.keys().min().copied()
    }

    pub fn update_last_trade_price(&mut self, price: u64) -> Result<(), &'static str> {
        self.last_trade_price = Some(price);

        let triggered_orders = self.stop_order_book.get_triggered_orders(price);

        if !triggered_orders.is_empty() {
            self.stop_order_book
                .remove_triggered_orders(&triggered_orders);

            for order in triggered_orders {
                let mut order_ref = order.write();

                if order_ref.order_type == OrderType::StopMarket {
                    order_ref.order_type = OrderType::Market;
                    order_ref.price = match order_ref.side {
                        Side::Buy => self.get_best_ask_price().unwrap_or(price),
                        Side::Sell => self.get_best_bid_price().unwrap_or(price),
                    };
                } else if order_ref.order_type == OrderType::StopLimit {
                    order_ref.order_type = OrderType::Limit;
                }

                drop(order_ref);

                self.add_order(Arc::clone(&order))?;
            }
        }

        Ok(())
    }

    pub fn expire_orders(&mut self, current_time: i64) -> Vec<Arc<RwLock<Order>>> {
        let mut expired_order_ids = Vec::new();
        let mut expired_orders = Vec::new();

        for (&order_id, order) in &self.order_map {
            let order_ref = order.read();
            if order_ref.is_expired(current_time) {
                expired_order_ids.push(order_id);
            }
        }

        for order_id in expired_order_ids {
            if let Some(order) = self.remove_order(order_id) {
                let mut order_ref = order.write();
                order_ref.status = OrderStatus::Expired;
                drop(order_ref);
                expired_orders.push(order);
            }
        }

        expired_orders
    }

    pub fn get_order(&self, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        self.order_map.get(&order_id).cloned()
    }

    pub fn replenish_iceberg_order(
        &mut self,
        order: Arc<RwLock<Order>>,
    ) -> Result<(), &'static str> {
        let order_ref = order.read();
        let price = order_ref.price;
        let side = order_ref.side;
        let display_qty = order_ref.display_quantity.unwrap_or(0);
        let remaining_qty = order_ref.remaining_quantity();
        drop(order_ref);

        let levels = match side {
            Side::Buy => &mut self.buy_levels,
            Side::Sell => &mut self.sell_levels,
        };

        if let Some(level) = levels.get_mut(&price) {
            let new_visible = std::cmp::min(display_qty as u64, remaining_qty as u64);
            level.visible_volume = new_visible;

            Ok(())
        } else {
            Err("Price level not found")
        }
    }

    fn update_depth(&self) {
        let mut depth = self.depth.write();
        depth.bid_levels.clear();
        depth.ask_levels.clear();

        let mut bid_prices: Vec<_> = self.buy_levels.keys().cloned().collect();
        bid_prices.sort_by(|a, b| b.cmp(a));
        for &price in bid_prices.iter().take(self.depth_levels) {
            if let Some(level) = self.buy_levels.get(&price) {
                depth.bid_levels.push((price, level.visible_volume));
            }
        }

        let mut ask_prices: Vec<_> = self.sell_levels.keys().cloned().collect();
        ask_prices.sort();
        for &price in ask_prices.iter().take(self.depth_levels) {
            if let Some(level) = self.sell_levels.get(&price) {
                depth.ask_levels.push((price, level.visible_volume));
            }
        }
    }

    pub fn get_market_depth(&self) -> MarketDepth {
        self.depth.read().clone()
    }

    pub fn set_depth_levels(&mut self, levels: usize) {
        self.depth_levels = levels;
        self.update_depth();
    }

    pub fn create_snapshot(&self) -> OrderBookSnapshot {
        let mut buy_levels = HashMap::new();
        let mut sell_levels = HashMap::new();
        let mut stop_orders = Vec::new();

        for (&price, level) in &self.buy_levels {
            let orders = level
                .orders
                .iter()
                .map(|o| OrderSnapshot::from(&*o.read()))
                .collect();

            buy_levels.insert(
                price,
                PriceLevelSnapshot {
                    price,
                    orders,
                    total_volume: level.total_volume,
                    visible_volume: level.visible_volume,
                },
            );
        }

        for (&price, level) in &self.sell_levels {
            let orders = level
                .orders
                .iter()
                .map(|o| OrderSnapshot::from(&*o.read()))
                .collect();

            sell_levels.insert(
                price,
                PriceLevelSnapshot {
                    price,
                    orders,
                    total_volume: level.total_volume,
                    visible_volume: level.visible_volume,
                },
            );
        }

        {
            let snapshot_stop_orders: Vec<_> = self
                .stop_order_book
                .order_map
                .values()
                .map(|o| OrderSnapshot::from(&*o.read()))
                .collect();
            stop_orders.extend(snapshot_stop_orders);
        }

        OrderBookSnapshot {
            symbol: self.symbol.clone(),
            buy_levels,
            sell_levels,
            stop_orders,
            last_trade_price: self.last_trade_price,
        }
    }

    pub fn restore_from_snapshot(snapshot: &OrderBookSnapshot) -> Self {
        snapshot.restore()
    }
}

pub struct ConcurrentOrderBook {
    symbol: String,
    buy_levels: DashMap<u64, CachePadded<PriceLevel>>,
    sell_levels: DashMap<u64, CachePadded<PriceLevel>>,
    order_map: DashMap<u64, Arc<RwLock<Order>>>,
    stop_order_book: RwLock<StopOrderBook>,
    last_trade_price: RwLock<Option<u64>>,
}

impl ConcurrentOrderBook {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            buy_levels: DashMap::new(),
            sell_levels: DashMap::new(),
            order_map: DashMap::new(),
            stop_order_book: RwLock::new(StopOrderBook::new(symbol)),
            last_trade_price: RwLock::new(None),
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn add_order(&self, order: Arc<RwLock<Order>>) -> Result<(), &'static str> {
        let order_ref = order.read();
        let order_id = order_ref.id;
        let price = order_ref.price;
        let side = order_ref.side;
        drop(order_ref);

        let levels = match side {
            Side::Buy => &self.buy_levels,
            Side::Sell => &self.sell_levels,
        };

        let mut entry = levels
            .entry(price)
            .or_insert_with(|| CachePadded::new(PriceLevel::new(price)));
        entry.value_mut().add_order(Arc::clone(&order));
        self.order_map.insert(order_id, order);
        Ok(())
    }

    pub fn get_last_trade_price(&self) -> Option<u64> {
        *self.last_trade_price.read()
    }

    pub fn update_last_trade_price(&self, price: u64) {
        *self.last_trade_price.write() = Some(price);
    }
}

#[cfg(test)]
mod tests;
