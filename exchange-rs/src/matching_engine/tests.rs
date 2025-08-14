use super::*;
use crate::order::*;

#[cfg(test)]
mod matching_engine_tests {
    use super::*;

    #[test]
    fn test_matching_engine_creation() {
        let engine = MatchingEngine::new();
        assert!(engine.order_books.is_empty());
    }

    #[test]
    fn test_add_symbol() {
        let mut engine = MatchingEngine::new();
        
        engine.add_symbol("BTCUSD");
        assert!(engine.order_books.contains_key("BTCUSD"));
        
        engine.add_symbol("BTCUSD");
        assert!(engine.order_books.contains_key("BTCUSD"));
    }

    #[test]
    fn test_place_order_unknown_symbol() {
        let mut engine = MatchingEngine::new();
        
        let order = Order::new(
            "UNKNOWN".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            1000,
            1,
        );
        
        let result = engine.place_order(order);
        assert!(result.is_err());
    }

    #[test]
    fn test_place_limit_order_no_match() {
        let mut engine = MatchingEngine::new();
        engine.add_symbol("TESTPAIR");
        
        let order = Order::new(
            "TESTPAIR".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            1000,
            1,
        );
        
        let result = engine.place_order(order).unwrap();
        assert!(result.trades.is_empty());
        assert!(result.remaining_order.is_some());
    }

    #[test]
    fn test_matching_orders() {
        let mut engine = MatchingEngine::new();
        engine.add_symbol("TESTPAIR");
        
        let buy_order = Order::new(
            "TESTPAIR".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            1000,
            1,
        );
        
        let sell_order = Order::new(
            "TESTPAIR".to_string(),
            Side::Sell,
            OrderType::Limit,
            100,
            800,
            2,
        );
        
        engine.place_order(buy_order).unwrap();
        let result = engine.place_order(sell_order).unwrap();
        
        assert_eq!(result.trades.len(), 1);
        assert_eq!(result.trades[0].quantity, 800);
        assert_eq!(result.trades[0].price, 100);
        assert_eq!(result.trades[0].buy_order_id, 1);
        assert_eq!(result.trades[0].sell_order_id, 2);
        assert!(result.remaining_order.is_none());
    }

    #[test]
    fn test_partial_fill() {
        let mut engine = MatchingEngine::new();
        engine.add_symbol("TESTPAIR");
        
        let large_buy = Order::new(
            "TESTPAIR".to_string(),
            Side::Buy,
            OrderType::Limit,
            100,
            5000,
            1,
        );
        
        let small_sell = Order::new(
            "TESTPAIR".to_string(),
            Side::Sell,
            OrderType::Limit,
            100,
            2000,
            2,
        );
        
        engine.place_order(large_buy).unwrap();
        let result = engine.place_order(small_sell).unwrap();
        
        assert_eq!(result.trades.len(), 1);
        assert_eq!(result.trades[0].quantity, 2000);
        assert!(result.remaining_order.is_none());
    }

    #[test]
    fn test_market_order_execution() {
        let mut engine = MatchingEngine::new();
        engine.add_symbol("TESTPAIR");
        
        let limit_sell = Order::new(
            "TESTPAIR".to_string(),
            Side::Sell,
            OrderType::Limit,
            105,
            1000,
            1,
        );
        
        let market_buy = Order::new(
            "TESTPAIR".to_string(),
            Side::Buy,
            OrderType::Market,
            0,
            800,
            2,
        );
        
        engine.place_order(limit_sell).unwrap();
        let result = engine.place_order(market_buy).unwrap();
        
        assert_eq!(result.trades.len(), 1);
        assert_eq!(result.trades[0].quantity, 800);
        assert_eq!(result.trades[0].price, 105);
        assert!(result.remaining_order.is_none());
    }

    #[test]
    fn test_order_id_generation() {
        let mut engine = MatchingEngine::new();
        engine.add_symbol("TEST");
        
        let order1 = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
        let order2 = Order::new("TEST".to_string(), Side::Sell, OrderType::Limit, 100, 10, 2);
        
        let result1 = engine.place_order(order1).unwrap();
        let result2 = engine.place_order(order2).unwrap();
        
        assert_eq!(result1.trades.len(), 0);
        assert_eq!(result2.trades.len(), 1);
    }

    #[test]
    fn test_trade_id_generation() {
        let mut engine = MatchingEngine::new();
        engine.add_symbol("TEST");
        
        let buy_order = Order::new("TEST".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
        let sell_order = Order::new("TEST".to_string(), Side::Sell, OrderType::Limit, 100, 10, 2);
        
        engine.place_order(buy_order).unwrap();
        let result = engine.place_order(sell_order).unwrap();
        
        assert_eq!(result.trades.len(), 1);
        assert!(result.trades[0].id > 0);
    }
}