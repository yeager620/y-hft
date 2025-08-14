use super::*;

#[cfg(test)]
mod order_tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "BTCUSD".to_string(),
            Side::Buy,
            OrderType::Limit,
            50000000000,
            1000,
            123,
        );
        
        assert_eq!(order.symbol, "BTCUSD");
        assert_eq!(order.side, Side::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.price, 50000000000);
        assert_eq!(order.quantity, 1000);
        assert_eq!(order.user_id, 123);
        assert_eq!(order.filled_quantity, 0);
        assert_eq!(order.status, OrderStatus::New);
        assert!(order.timestamp > 0);
    }

    #[test]
    fn test_order_id_generation() {
        let order1 = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
        let order2 = Order::new("TEST".to_string(), Side::Sell, OrderType::Limit, 100, 10, 1);
        
        assert_ne!(order1.order_id, order2.order_id);
        assert!(order2.order_id > order1.order_id);
    }

    #[test]
    fn test_remaining_quantity() {
        let mut order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        
        assert_eq!(order.remaining_quantity(), 1000);
        
        order.filled_quantity = 300;
        assert_eq!(order.remaining_quantity(), 700);
        
        order.filled_quantity = 1000;
        assert_eq!(order.remaining_quantity(), 0);
    }

    #[test]
    fn test_is_fully_filled() {
        let mut order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        
        assert!(!order.is_fully_filled());
        
        order.filled_quantity = 500;
        assert!(!order.is_fully_filled());
        
        order.filled_quantity = 1000;
        assert!(order.is_fully_filled());
    }

    #[test]
    fn test_can_match() {
        let buy_order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        let sell_order = Order::new("TEST".to_string(), Side::Sell, OrderType::Limit, 100, 1000, 2);
        let higher_sell = Order::new("TEST".to_string(), Side::Sell, OrderType::Limit, 110, 1000, 3);
        
        assert!(buy_order.can_match(&sell_order));
        assert!(!buy_order.can_match(&higher_sell));
        assert!(sell_order.can_match(&buy_order));
    }

    #[test]
    fn test_stop_order_triggering() {
        let mut stop_buy = Order::new("TEST".to_string(), Side::Buy, OrderType::StopLimit, 110, 1000, 1);
        stop_buy.stop_price = Some(105);
        
        assert!(!stop_buy.is_stop_triggered(100));
        assert!(!stop_buy.is_stop_triggered(104));
        assert!(stop_buy.is_stop_triggered(105));
        assert!(stop_buy.is_stop_triggered(110));
        
        let mut stop_sell = Order::new("TEST".to_string(), Side::Sell, OrderType::StopLimit, 90, 1000, 2);
        stop_sell.stop_price = Some(95);
        
        assert!(!stop_sell.is_stop_triggered(100));
        assert!(!stop_sell.is_stop_triggered(96));
        assert!(stop_sell.is_stop_triggered(95));
        assert!(stop_sell.is_stop_triggered(90));
    }

    #[test]
    fn test_time_in_force_expiration() {
        let current_time = Order::get_nano_timestamp();
        
        let gtc_order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        assert!(!gtc_order.is_expired(current_time + 86400000000000));
        
        let mut day_order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 2);
        day_order.time_in_force = TimeInForce::Day;
        assert!(!day_order.is_expired(current_time));
        assert!(day_order.is_expired(current_time + 86400000000000 + 1));
        
        let mut gtd_order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 3);
        let expire_time = current_time + 3600000000000;
        gtd_order.time_in_force = TimeInForce::GTD(expire_time);
        assert!(!gtd_order.is_expired(current_time));
        assert!(!gtd_order.is_expired(expire_time - 1));
        assert!(gtd_order.is_expired(expire_time + 1));
    }

    #[test]
    fn test_iceberg_order_visibility() {
        let mut iceberg = Order::new("TEST".to_string(), Side::Buy, OrderType::Iceberg, 100, 10000, 1);
        iceberg.display_quantity = Some(2000);
        
        assert_eq!(iceberg.visible_quantity(), 2000);
        
        iceberg.filled_quantity = 1000;
        assert_eq!(iceberg.visible_quantity(), 2000);
        
        iceberg.filled_quantity = 8500;
        assert_eq!(iceberg.visible_quantity(), 1500);
        
        iceberg.filled_quantity = 9500;
        assert_eq!(iceberg.visible_quantity(), 500);
        
        iceberg.filled_quantity = 10000;
        assert_eq!(iceberg.visible_quantity(), 0);
    }

    #[test]
    fn test_order_status_transitions() {
        let mut order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        
        assert_eq!(order.status, OrderStatus::New);
        
        order.status = OrderStatus::PartiallyFilled;
        assert_eq!(order.status, OrderStatus::PartiallyFilled);
        
        order.status = OrderStatus::Filled;
        assert_eq!(order.status, OrderStatus::Filled);
        
        order.status = OrderStatus::Canceled;
        assert_eq!(order.status, OrderStatus::Canceled);
    }

    #[test]
    fn test_market_order_price_handling() {
        let market_buy = Order::new("TEST".to_string(), Side::Buy, OrderType::Market, 0, 1000, 1);
        let market_sell = Order::new("TEST".to_string(), Side::Sell, OrderType::Market, 0, 1000, 2);
        
        assert_eq!(market_buy.price, 0);
        assert_eq!(market_sell.price, 0);
        
        let limit_buy = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 3);
        let limit_sell = Order::new("TEST".to_string(), Side::Sell, OrderType::Limit, 110, 1000, 4);
        
        assert!(market_buy.can_match(&limit_sell));
        assert!(market_sell.can_match(&limit_buy));
    }

    #[test]
    fn test_order_priority_comparison() {
        let earlier_time = Order::get_nano_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let later_time = Order::get_nano_timestamp();
        
        let mut order1 = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        let mut order2 = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 110, 1000, 2);
        let mut order3 = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 3);
        
        order1.timestamp = earlier_time;
        order3.timestamp = later_time;
        
        assert!(order2.price > order1.price);
        assert!(order1.timestamp < order3.timestamp);
    }

    #[test]
    fn test_order_validation() {
        let valid_order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        assert!(valid_order.quantity > 0);
        assert!(!valid_order.symbol.is_empty());
        
        let mut invalid_order = Order::new("".to_string(), Side::Buy, OrderType::Limit, 100, 0, 1);
        invalid_order.quantity = 0;
        assert_eq!(invalid_order.quantity, 0);
        assert!(invalid_order.symbol.is_empty());
    }

    #[test]
    fn test_order_clone_and_equality() {
        let order1 = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 1000, 1);
        let order2 = order1.clone();
        
        assert_eq!(order1.order_id, order2.order_id);
        assert_eq!(order1.symbol, order2.symbol);
        assert_eq!(order1.side, order2.side);
        assert_eq!(order1.price, order2.price);
        assert_eq!(order1.quantity, order2.quantity);
    }

    #[test]
    fn test_side_opposite() {
        assert_eq!(Side::Buy.opposite(), Side::Sell);
        assert_eq!(Side::Sell.opposite(), Side::Buy);
    }

    #[test]
    fn test_order_type_variants() {
        let limit = OrderType::Limit;
        let market = OrderType::Market;
        let stop_limit = OrderType::StopLimit;
        let stop_market = OrderType::StopMarket;
        let iceberg = OrderType::Iceberg;
        
        assert_ne!(limit, market);
        assert_ne!(market, stop_limit);
        assert_ne!(stop_limit, stop_market);
        assert_ne!(stop_market, iceberg);
    }

    #[test]
    fn test_time_in_force_variants() {
        let gtc = TimeInForce::GTC;
        let ioc = TimeInForce::IOC;
        let fok = TimeInForce::FOK;
        let day = TimeInForce::Day;
        let gtd = TimeInForce::GTD(123456789);
        
        assert_ne!(gtc, ioc);
        assert_ne!(ioc, fok);
        assert_ne!(fok, day);
        assert_ne!(day, gtd);
        
        if let TimeInForce::GTD(timestamp) = gtd {
            assert_eq!(timestamp, 123456789);
        }
    }
}