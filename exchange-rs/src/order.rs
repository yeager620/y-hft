use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Iceberg, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    GTC, 
    IOC, 
    FOK, 
    GTD, 
    Day, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub price: u64, 
    pub quantity: u32,
    pub filled_quantity: u32,
    pub status: OrderStatus,
    pub timestamp: i64, 
    pub user_id: u64,
    pub time_in_force: TimeInForce,
    pub expiration_time: i64, 
    pub stop_price: Option<u64>, 
    pub display_quantity: Option<u32>, 
}

impl Order {
    pub fn new(
        symbol: String,
        side: Side,
        order_type: OrderType,
        price: u64,
        quantity: u32,
        user_id: u64,
    ) -> Self {
        Self {
            id: 0, 
            symbol,
            side,
            order_type,
            price,
            quantity,
            filled_quantity: 0,
            status: OrderStatus::New,
            timestamp: Self::get_nano_timestamp(),
            user_id,
            time_in_force: TimeInForce::GTC,
            expiration_time: 0,
            stop_price: None,
            display_quantity: None,
        }
    }

    pub fn remaining_quantity(&self) -> u32 {
        self.quantity - self.filled_quantity
    }

    pub fn visible_quantity(&self) -> u32 {
        if self.order_type == OrderType::Iceberg && self.display_quantity.is_some() {
            std::cmp::min(self.display_quantity.unwrap(), self.remaining_quantity())
        } else {
            self.remaining_quantity()
        }
    }

    pub fn is_filled(&self) -> bool {
        self.filled_quantity >= self.quantity
    }

    pub fn is_stop_order(&self) -> bool {
        self.order_type == OrderType::StopLimit || self.order_type == OrderType::StopMarket
    }

    pub fn is_stop_triggered(&self, last_price: u64) -> bool {
        if !self.is_stop_order() || self.stop_price.is_none() {
            return false;
        }

        match self.side {
            Side::Buy => last_price >= self.stop_price.unwrap(),
            Side::Sell => last_price <= self.stop_price.unwrap(),
        }
    }

    pub fn is_expired(&self, current_time: i64) -> bool {
        match self.time_in_force {
            TimeInForce::GTD => current_time >= self.expiration_time,
            TimeInForce::Day => {
                let ns_per_day = 86_400_000_000_000i64;
                let current_day = current_time / ns_per_day;
                let order_day = self.timestamp / ns_per_day;
                
                current_day > order_day
            }
            _ => false,
        }
    }

    pub fn get_nano_timestamp() -> i64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let nanos = duration.as_nanos() as i64;
                (nanos / 1_000_000) * 1_000_000
            }
            Err(_) => 0, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_remaining_quantity() {
        let mut order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert_eq!(order.remaining_quantity(), 10);

        order.filled_quantity = 5;
        assert_eq!(order.remaining_quantity(), 5);

        order.filled_quantity = 10;
        assert_eq!(order.remaining_quantity(), 0);
    }

    #[test]
    fn test_visible_quantity() {
        let mut order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert_eq!(order.visible_quantity(), 10);

        let mut iceberg_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Iceberg,
            100,
            100,
            1,
        );
        iceberg_order.display_quantity = Some(10);
        assert_eq!(iceberg_order.visible_quantity(), 10);

        iceberg_order.filled_quantity = 95;
        assert_eq!(iceberg_order.visible_quantity(), 5);
    }

    #[test]
    fn test_is_filled() {
        let mut order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert!(!order.is_filled());

        order.filled_quantity = 5;
        assert!(!order.is_filled());

        order.filled_quantity = 10;
        assert!(order.is_filled());
    }

    #[test]
    fn test_is_stop_order() {
        let limit_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert!(!limit_order.is_stop_order());

        let stop_limit_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::StopLimit,
            100,
            10,
            1,
        );
        assert!(stop_limit_order.is_stop_order());

        let stop_market_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::StopMarket,
            100,
            10,
            1,
        );
        assert!(stop_market_order.is_stop_order());
    }

    #[test]
    fn test_is_stop_triggered() {
        let mut buy_stop_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::StopLimit,
            100,
            10,
            1,
        );
        buy_stop_order.stop_price = Some(105);

        assert!(!buy_stop_order.is_stop_triggered(100));
        assert!(!buy_stop_order.is_stop_triggered(104));
        assert!(buy_stop_order.is_stop_triggered(105));
        assert!(buy_stop_order.is_stop_triggered(106));

        let mut sell_stop_order = Order::new(
            "AAPL".to_string(),
            Side::Sell,
            OrderType::StopLimit,
            100,
            10,
            1,
        );
        sell_stop_order.stop_price = Some(95);

        assert!(!sell_stop_order.is_stop_triggered(100));
        assert!(!sell_stop_order.is_stop_triggered(96));
        assert!(sell_stop_order.is_stop_triggered(95));
        assert!(sell_stop_order.is_stop_triggered(94));
    }

    #[test]
    fn test_is_expired() {
        let mut gtd_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        gtd_order.time_in_force = TimeInForce::GTD;

        let current_time = Order::get_nano_timestamp();
        let one_day_ns: i64 = 86_400_000_000_000;

        gtd_order.expiration_time = current_time + one_day_ns;
        assert!(!gtd_order.is_expired(current_time));

        gtd_order.expiration_time = current_time - 1;
        assert!(gtd_order.is_expired(current_time));

        let mut day_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        day_order.time_in_force = TimeInForce::Day;

        let current_time = Order::get_nano_timestamp();
        let one_day_ns: i64 = 86_400_000_000_000;

        assert!(!day_order.is_expired(current_time));

        assert!(day_order.is_expired(current_time + one_day_ns));

        let gtc_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert!(!gtc_order.is_expired(current_time));
        assert!(!gtc_order.is_expired(current_time + one_day_ns * 365)); 
    }

    #[test]
    fn test_get_nano_timestamp() {
        let timestamp1 = Order::get_nano_timestamp();
        assert!(timestamp1 > 0);

        thread::sleep(Duration::from_millis(10));
        let timestamp2 = Order::get_nano_timestamp();
        assert!(timestamp2 > timestamp1);
    }

    #[test]
    fn test_order_types() {
        let market_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Market,
            0, 
            10,
            1,
        );
        assert_eq!(market_order.order_type, OrderType::Market);

        let limit_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert_eq!(limit_order.order_type, OrderType::Limit);

        let mut stop_limit_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::StopLimit,
            100,
            10,
            1,
        );
        stop_limit_order.stop_price = Some(105);
        assert_eq!(stop_limit_order.order_type, OrderType::StopLimit);
        assert!(stop_limit_order.is_stop_order());
        assert_eq!(stop_limit_order.stop_price, Some(105));

        let mut stop_market_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::StopMarket,
            0, 
            10,
            1,
        );
        stop_market_order.stop_price = Some(105);
        assert_eq!(stop_market_order.order_type, OrderType::StopMarket);
        assert!(stop_market_order.is_stop_order());
        assert_eq!(stop_market_order.stop_price, Some(105));

        let mut iceberg_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Iceberg,
            100,
            100,
            1,
        );
        iceberg_order.display_quantity = Some(10);
        assert_eq!(iceberg_order.order_type, OrderType::Iceberg);
        assert_eq!(iceberg_order.display_quantity, Some(10));
        assert_eq!(iceberg_order.visible_quantity(), 10);
    }

    #[test]
    fn test_time_in_force() {
        let gtc_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        assert_eq!(gtc_order.time_in_force, TimeInForce::GTC);

        let mut ioc_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        ioc_order.time_in_force = TimeInForce::IOC;
        assert_eq!(ioc_order.time_in_force, TimeInForce::IOC);

        let mut fok_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        fok_order.time_in_force = TimeInForce::FOK;
        assert_eq!(fok_order.time_in_force, TimeInForce::FOK);

        let mut gtd_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        gtd_order.time_in_force = TimeInForce::GTD;
        gtd_order.expiration_time = Order::get_nano_timestamp() + 86_400_000_000_000; 
        assert_eq!(gtd_order.time_in_force, TimeInForce::GTD);
        assert!(gtd_order.expiration_time > Order::get_nano_timestamp());

        let mut day_order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            10,
            1,
        );
        day_order.time_in_force = TimeInForce::Day;
        assert_eq!(day_order.time_in_force, TimeInForce::Day);
    }
}
