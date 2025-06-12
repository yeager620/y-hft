use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use super::order::{Order, OrderStatus, OrderType, Side, TimeInForce};
use super::orderbook::OrderBook;

#[derive(Serialize, Deserialize)]
pub struct OrderSnapshot {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub price: u64,
    pub quantity: u32,
    pub filled_quantity: u32,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub display_quantity: Option<u32>,
    pub stop_price: Option<u64>,
    pub timestamp: i64,
    pub user_id: u64,
    pub expiration_time: i64,
}

impl From<&Order> for OrderSnapshot {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id,
            symbol: order.symbol.clone(),
            side: order.side,
            order_type: order.order_type,
            price: order.price,
            quantity: order.quantity,
            filled_quantity: order.filled_quantity,
            status: order.status,
            time_in_force: order.time_in_force,
            display_quantity: order.display_quantity,
            stop_price: order.stop_price,
            timestamp: order.timestamp,
            user_id: order.user_id,
            expiration_time: order.expiration_time,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PriceLevelSnapshot {
    pub price: u64,
    pub orders: Vec<OrderSnapshot>,
    pub total_volume: u64,
    pub visible_volume: u64,
}

#[derive(Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub symbol: String,
    pub buy_levels: HashMap<u64, PriceLevelSnapshot>,
    pub sell_levels: HashMap<u64, PriceLevelSnapshot>,
    pub stop_orders: Vec<OrderSnapshot>,
    pub last_trade_price: Option<u64>,
}

impl OrderBookSnapshot {
    pub fn restore(&self) -> OrderBook {
        let mut book = OrderBook::new(&self.symbol);

        for (_price, level_snapshot) in &self.buy_levels {
            for order_snapshot in &level_snapshot.orders {
                let order = Arc::new(RwLock::new(order_snapshot.to_order()));
                book.add_order(order).unwrap();
            }
        }

        for (_price, level_snapshot) in &self.sell_levels {
            for order_snapshot in &level_snapshot.orders {
                let order = Arc::new(RwLock::new(order_snapshot.to_order()));
                book.add_order(order).unwrap();
            }
        }

        for stop_order in &self.stop_orders {
            let order = Arc::new(RwLock::new(stop_order.to_order()));
            book.add_stop_order(order).unwrap();
        }

        if let Some(price) = self.last_trade_price {
            book.update_last_trade_price(price).unwrap();
        }

        book
    }
}

impl OrderSnapshot {
    fn to_order(&self) -> Order {
        Order {
            id: self.id,
            symbol: self.symbol.clone(),
            side: self.side,
            order_type: self.order_type,
            price: self.price,
            quantity: self.quantity,
            filled_quantity: self.filled_quantity,
            status: self.status,
            time_in_force: self.time_in_force,
            display_quantity: self.display_quantity,
            stop_price: self.stop_price,
            timestamp: self.timestamp,
            user_id: self.user_id,
            expiration_time: self.expiration_time,
        }
    }
}
