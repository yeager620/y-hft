use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use parking_lot::RwLock;
use thiserror::Error;

use crate::order::{Order, OrderStatus, OrderType, Side, TimeInForce};
use crate::orderbook::OrderBook;

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: u64,
    pub buy_order_id: u64,
    pub sell_order_id: u64,
    pub price: u64,
    pub quantity: u32,
    pub timestamp: i64,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MatchingError {
    #[error("Symbol not found")]
    SymbolNotFound,

    #[error("No liquidity available")]
    NoLiquidity,

    #[error("FOK order cannot be filled")]
    FOKCannotBeFilled,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<&str> for MatchingError {
    fn from(error: &str) -> Self {
        MatchingError::InternalError(error.to_string())
    }
}

#[derive(Debug)]
pub struct TradeExecutionResult {
    pub trades: Vec<Trade>,
    pub remaining_order: Option<Arc<RwLock<Order>>>,
    pub filled_orders: Vec<Arc<RwLock<Order>>>,
    pub rejected: bool,
}

impl TradeExecutionResult {
    pub fn new() -> Self {
        Self {
            trades: Vec::new(),
            remaining_order: None,
            filled_orders: Vec::new(),
            rejected: false,
        }
    }
}

pub struct MatchingEngine {
    pub order_books: HashMap<String, OrderBook>,
    next_order_id: u64,
    next_trade_id: u64,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
            next_order_id: 1,
            next_trade_id: 1,
        }
    }

    pub fn add_symbol(&mut self, symbol: &str) {
        if !self.order_books.contains_key(symbol) {
            self.order_books.insert(symbol.to_string(), OrderBook::new(symbol));
        }
    }

    pub fn place_order(&mut self, mut new_order: Order) -> Result<TradeExecutionResult, MatchingError> {
        let mut result = TradeExecutionResult::new();

        if !self.order_books.contains_key(&new_order.symbol) {
            return Err(MatchingError::SymbolNotFound);
        }

        new_order.id = self.next_order_id;
        self.next_order_id += 1;

        let order = Arc::new(RwLock::new(new_order));

        let order_book = self.order_books.get_mut(&order.read().symbol).unwrap();

        {
            let mut order_ref = order.write();
            if order_ref.order_type == OrderType::Market {
                match order_ref.side {
                    Side::Buy => {
                        if let Some(price) = order_book.get_best_ask_price() {
                            order_ref.price = price;
                        } else {
                            result.rejected = true;
                            return Err(MatchingError::NoLiquidity);
                        }
                    },
                    Side::Sell => {
                        if let Some(price) = order_book.get_best_bid_price() {
                            order_ref.price = price;
                        } else {
                            result.rejected = true;
                            return Err(MatchingError::NoLiquidity);
                        }
                    },
                }
            }
        }

        let time_in_force;
        let is_stop_order;

        {
            let order_ref = order.read();
            time_in_force = order_ref.time_in_force;
            is_stop_order = order_ref.is_stop_order();
        }

        if time_in_force == TimeInForce::IOC || time_in_force == TimeInForce::FOK {
            if time_in_force == TimeInForce::FOK {
                if !MatchingEngine::can_fill_order(order_book, &order)? {
                    result.rejected = true;
                    let mut order_ref = order.write();
                    order_ref.status = OrderStatus::Rejected;
                    return Err(MatchingError::FOKCannotBeFilled);
                }
            }

            MatchingEngine::match_order(&mut self.next_trade_id, order_book, Arc::clone(&order), &mut result)?;

            {
                let mut order_ref = order.write();
                if order_ref.is_filled() {
                    order_ref.status = OrderStatus::Filled;
                } else if time_in_force == TimeInForce::IOC {
                    order_ref.status = OrderStatus::Canceled;
                } else {
                    order_ref.status = OrderStatus::Rejected;
                    result.rejected = true;
                    return Err(MatchingError::FOKCannotBeFilled);
                }
            }

            result.filled_orders.push(Arc::clone(&order));
            result.remaining_order = None;
            return Ok(result);
        } else if is_stop_order {
            let should_trigger = if let Some(last_price) = order_book.last_trade_price {
                let order_ref = order.read();
                order_ref.is_stop_triggered(last_price)
            } else {
                false
            };

            if should_trigger {
                {
                    let mut order_ref = order.write();
                    if order_ref.order_type == OrderType::StopMarket {
                        order_ref.order_type = OrderType::Market;
                    } else if order_ref.order_type == OrderType::StopLimit {
                        order_ref.order_type = OrderType::Limit;
                    }
                }

                MatchingEngine::match_order(&mut self.next_trade_id, order_book, Arc::clone(&order), &mut result)?;
            } else {
                order_book.add_stop_order(Arc::clone(&order))?;
                result.remaining_order = Some(Arc::clone(&order));
                return Ok(result);
            }
        } else {
            MatchingEngine::match_order(&mut self.next_trade_id, order_book, Arc::clone(&order), &mut result)?;
        }

        let order_ref = order.read();
        if order_ref.is_filled() {
            let order_id = order_ref.id;
            let already_added = result.filled_orders.iter().any(|o| o.read().id == order_id);
            if !already_added {
                result.filled_orders.push(Arc::clone(&order));
            }
        } else if order_ref.order_type == OrderType::Limit || order_ref.order_type == OrderType::Iceberg {
            drop(order_ref);
            order_book.add_order(Arc::clone(&order))?;
            result.remaining_order = Some(Arc::clone(&order));

            let order_ref = order.read();
            if order_ref.order_type == OrderType::Iceberg {
                drop(order_ref);
                order_book.replenish_iceberg_order(Arc::clone(&order))?;
            }
        } else {
            result.remaining_order = Some(Arc::clone(&order));
            return Err(MatchingError::NoLiquidity);
        }

        Ok(result)
    }

    fn can_fill_order(order_book: &OrderBook, order: &Arc<RwLock<Order>>) -> Result<bool, MatchingError> {
        let order_ref = order.read();
        let remaining_qty = order_ref.quantity;
        let side = order_ref.side;
        let price = order_ref.price;
        let order_type = order_ref.order_type;

        let opposite_levels = match side {
            Side::Buy => &order_book.sell_levels,
            Side::Sell => &order_book.buy_levels,
        };

        let mut prices: Vec<u64> = opposite_levels.keys().cloned().collect();

        if side == Side::Buy {
            prices.sort_unstable(); 
        } else {
            prices.sort_unstable_by(|a, b| b.cmp(a)); 
        }

        let mut available_qty = 0;

        for &level_price in &prices {
            let price_matches = match side {
                Side::Buy => level_price <= price,
                Side::Sell => level_price >= price,
            };

            if !price_matches && order_type == OrderType::Limit {
                break;
            }

            if let Some(level) = opposite_levels.get(&level_price) {
                for resting_order in &level.orders {
                    available_qty += resting_order.read().remaining_quantity();
                }

                if available_qty >= remaining_qty {
                    return Ok(true); 
                }
            }
        }

        Ok(false) 
    }

    fn match_order(next_trade_id: &mut u64, order_book: &mut OrderBook, incoming_order: Arc<RwLock<Order>>, result: &mut TradeExecutionResult) -> Result<(), MatchingError> {
        let mut continue_matching = true;

        while continue_matching {
            if incoming_order.read().is_filled() {
                break;
            }

            let side = incoming_order.read().side;
            let best_price = match side {
                Side::Buy => order_book.get_best_ask_price(),
                Side::Sell => order_book.get_best_bid_price(),
            };

            if best_price.is_none() {
                break;
            }

            let best_price = best_price.unwrap();

            let price_matches = {
                let order_ref = incoming_order.read();
                match side {
                    Side::Buy => best_price <= order_ref.price,
                    Side::Sell => best_price >= order_ref.price,
                }
            };

            if !price_matches && incoming_order.read().order_type == OrderType::Limit {
                break;
            }

            let opposite_levels = match side {
                Side::Buy => &mut order_book.sell_levels,
                Side::Sell => &mut order_book.buy_levels,
            };

            if let Some(level) = opposite_levels.get_mut(&best_price) {
                let mut i = 0;
                let mut orders_to_replenish = Vec::new();

                while i < level.orders.len() && !incoming_order.read().is_filled() {
                    let resting_order: Arc<RwLock<Order>> = Arc::clone(&level.orders[i]);

                    let trade_qty = std::cmp::min(
                        incoming_order.read().remaining_quantity(),
                        resting_order.read().visible_quantity()
                    );

                    if trade_qty > 0 {
                        MatchingEngine::execute_trade(
                            next_trade_id,
                            Arc::clone(&incoming_order),
                            Arc::clone(&resting_order),
                            trade_qty,
                            best_price,
                            result
                        )?;

                        if resting_order.read().is_filled() {
                            level.orders.remove(i);
                            result.filled_orders.push(Arc::clone(&resting_order));
                        } else {
                            if resting_order.read().order_type == OrderType::Iceberg {
                                orders_to_replenish.push(Arc::clone(&resting_order));
                            }
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }

                if level.orders.is_empty() {
                    opposite_levels.remove(&best_price);
                }

                for order in orders_to_replenish {
                    order_book.replenish_iceberg_order(order)?;
                }
            } else {
                continue_matching = false;
            }
        }

        if !result.trades.is_empty() {
            let last_trade = &result.trades[result.trades.len() - 1];
            order_book.update_last_trade_price(last_trade.price)?;
        }


        Ok(())
    }

    fn execute_trade(
        next_trade_id: &mut u64,
        buy_order: Arc<RwLock<Order>>,
        sell_order: Arc<RwLock<Order>>,
        quantity: u32,
        price: u64,
        result: &mut TradeExecutionResult
    ) -> Result<(), MatchingError> {
        let trade = Trade {
            id: *next_trade_id,
            buy_order_id: if buy_order.read().side == Side::Buy { buy_order.read().id } else { sell_order.read().id },
            sell_order_id: if buy_order.read().side == Side::Buy { sell_order.read().id } else { buy_order.read().id },
            price,
            quantity,
            timestamp: get_nano_timestamp(),
        };
        *next_trade_id += 1;

        {
            let mut buy_ref = buy_order.write();
            buy_ref.filled_quantity += quantity;

            if buy_ref.is_filled() {
                buy_ref.status = OrderStatus::Filled;
            } else {
                buy_ref.status = OrderStatus::PartiallyFilled;
            }
        }

        {
            let mut sell_ref = sell_order.write();
            sell_ref.filled_quantity += quantity;

            if sell_ref.is_filled() {
                sell_ref.status = OrderStatus::Filled;
            } else {
                sell_ref.status = OrderStatus::PartiallyFilled;
            }
        }

        result.trades.push(trade);

        Ok(())
    }

    pub fn cancel_order(&mut self, symbol: &str, order_id: u64) -> Option<Arc<RwLock<Order>>> {
        if let Some(order_book) = self.order_books.get_mut(symbol) {
            if let Some(canceled_order) = order_book.cancel_order(order_id) {
                let mut order_ref = canceled_order.write();
                order_ref.status = OrderStatus::Canceled;
                drop(order_ref);
                return Some(canceled_order);
            }
        }
        None
    }

    pub fn process_expired_orders(&mut self) -> Result<Vec<Arc<RwLock<Order>>>, MatchingError> {
        let current_time = get_nano_timestamp();
        let mut expired_orders = Vec::new();

        for order_book in self.order_books.values_mut() {
            let book_expired = order_book.expire_orders(current_time);
            expired_orders.extend(book_expired);
        }

        Ok(expired_orders)
    }

    fn process_ioc_order(&mut self, order: Arc<RwLock<Order>>) -> Result<TradeExecutionResult, MatchingError> {
        let mut result = TradeExecutionResult::new();
        let order_book = self.order_books.get_mut(&order.read().symbol).unwrap();
        MatchingEngine::match_order(&mut self.next_trade_id, order_book, Arc::clone(&order), &mut result)?;

        {
            let mut order_ref = order.write();
            if !order_ref.is_filled() {
                order_ref.status = OrderStatus::Canceled;
            }
        }

        result.filled_orders.push(Arc::clone(&order));
        result.remaining_order = None; 
        Ok(result)
    }

    fn process_stop_market_order(&mut self, order: Arc<RwLock<Order>>, trigger_price: u64) -> Result<TradeExecutionResult, MatchingError> {
        let mut result = TradeExecutionResult::new();
        let order_book = self.order_books.get_mut(&order.read().symbol).unwrap();

        {
            let mut order_ref = order.write();
            order_ref.order_type = OrderType::Market;

            match order_ref.side {
                Side::Buy => {
                    if let Some(price) = order_book.get_best_ask_price() {
                        order_ref.price = price;
                    } else {
                        return Err(MatchingError::NoLiquidity);
                    }
                },
                Side::Sell => {
                    if let Some(price) = order_book.get_best_bid_price() {
                        order_ref.price = price;
                    } else {
                        return Err(MatchingError::NoLiquidity);
                    }
                },
            }
        }

        MatchingEngine::match_order(&mut self.next_trade_id, order_book, Arc::clone(&order), &mut result)?;

        order_book.update_last_trade_price(trigger_price)?;

        result.filled_orders.push(Arc::clone(&order));

        Ok(result)
    }

    fn process_triggered_stop_order(&mut self, order: Arc<RwLock<Order>>, trigger_price: u64) -> Result<TradeExecutionResult, MatchingError> {
        let order_type = order.read().order_type;

        match order_type {
            OrderType::StopMarket => {
                self.process_stop_market_order(order, trigger_price)
            },
            OrderType::StopLimit => {
                let mut result = TradeExecutionResult::new();
                let order_book = self.order_books.get_mut(&order.read().symbol).unwrap();

                {
                    let mut order_ref = order.write();
                    order_ref.order_type = OrderType::Limit;
                }

                MatchingEngine::match_order(&mut self.next_trade_id, order_book, Arc::clone(&order), &mut result)?;

                order_book.update_last_trade_price(trigger_price)?;

                if !order.read().is_filled() {
                    order_book.add_order(Arc::clone(&order))?;
                    result.remaining_order = Some(Arc::clone(&order));
                } else {
                    result.filled_orders.push(Arc::clone(&order));
                }

                Ok(result)
            },
            _ => Err(MatchingError::InternalError("Invalid stop order type".to_string())),
        }
    }
}

fn get_nano_timestamp() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let nanos = duration.as_nanos() as i64;
            (nanos / 1_000_000) * 1_000_000
        }
        Err(_) => 0, 
    }
}

