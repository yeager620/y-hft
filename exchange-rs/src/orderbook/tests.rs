use super::*;
use crate::order::*;
use std::sync::Arc;
use parking_lot::RwLock;

#[cfg(test)]
mod orderbook_tests {
    use super::*;

    fn create_test_order(side: Side, price: u64, quantity: u32, user_id: u64) -> Arc<RwLock<Order>> {
        Arc::new(RwLock::new(Order::new(
            "TEST".to_string(),
            side,
            OrderType::Limit,
            price,
            quantity,
            user_id,
        )))
    }

    #[test]
    fn test_orderbook_creation() {
        let orderbook = OrderBook::new("BTCUSD");
        let depth = orderbook.get_market_depth();
        assert!(depth.bid_levels.is_empty());
        assert!(depth.ask_levels.is_empty());
    }

    #[test]
    fn test_add_buy_order() {
        let mut orderbook = OrderBook::new("TEST");
        let order = create_test_order(Side::Buy, 100, 1000, 1);
        
        let result = orderbook.add_order(order.clone());
        assert!(result.is_ok());
        
        let depth = orderbook.get_market_depth();
        assert_eq!(depth.bid_levels.len(), 1);
        assert_eq!(depth.bid_levels[0].0, 100);
        assert_eq!(depth.bid_levels[0].1, 1000);
    }

    #[test]
    fn test_add_sell_order() {
        let mut orderbook = OrderBook::new("TEST");
        let order = create_test_order(Side::Sell, 110, 1000, 1);
        
        let result = orderbook.add_order(order.clone());
        assert!(result.is_ok());
        
        let depth = orderbook.get_market_depth();
        assert_eq!(depth.ask_levels.len(), 1);
        assert_eq!(depth.ask_levels[0].0, 110);
        assert_eq!(depth.ask_levels[0].1, 1000);
    }

    #[test]
    fn test_price_level_sorting() {
        let mut orderbook = OrderBook::new("TEST");
        orderbook.set_depth_levels(10);
        
        let orders = vec![
            create_test_order(Side::Buy, 100, 1000, 1),
            create_test_order(Side::Buy, 105, 2000, 2),
            create_test_order(Side::Buy, 95, 1500, 3),
            create_test_order(Side::Sell, 110, 1000, 4),
            create_test_order(Side::Sell, 115, 2000, 5),
            create_test_order(Side::Sell, 108, 1500, 6),
        ];
        
        for order in orders {
            orderbook.add_order(order).unwrap();
        }
        
        let depth = orderbook.get_market_depth();
        assert_eq!(depth.bid_levels.len(), 3);
        assert_eq!(depth.ask_levels.len(), 3);
        
        assert!(depth.bid_levels[0].0 > depth.bid_levels[1].0);
        assert!(depth.bid_levels[1].0 > depth.bid_levels[2].0);
        assert_eq!(depth.bid_levels[0].0, 105);
        assert_eq!(depth.bid_levels[1].0, 100);
        assert_eq!(depth.bid_levels[2].0, 95);
        
        assert!(depth.ask_levels[0].0 < depth.ask_levels[1].0);
        assert!(depth.ask_levels[1].0 < depth.ask_levels[2].0);
        assert_eq!(depth.ask_levels[0].0, 108);
        assert_eq!(depth.ask_levels[1].0, 110);
        assert_eq!(depth.ask_levels[2].0, 115);
    }

    #[test]
    fn test_same_price_level_aggregation() {
        let mut orderbook = OrderBook::new("TEST");
        
        let order1 = create_test_order(Side::Buy, 100, 1000, 1);
        let order2 = create_test_order(Side::Buy, 100, 2000, 2);
        let order3 = create_test_order(Side::Buy, 100, 1500, 3);
        
        orderbook.add_order(order1).unwrap();
        orderbook.add_order(order2).unwrap();
        orderbook.add_order(order3).unwrap();
        
        let depth = orderbook.get_market_depth();
        assert_eq!(depth.bid_levels.len(), 1);
        assert_eq!(depth.bid_levels[0].0, 100);
        assert_eq!(depth.bid_levels[0].1, 4500);
    }

    #[test]
    fn test_remove_order() {
        let mut orderbook = OrderBook::new("TEST");
        
        let order = create_test_order(Side::Buy, 100, 1000, 1);
        let order_id = order.read().id;
        
        orderbook.add_order(order.clone()).unwrap();
        let depth_before = orderbook.get_market_depth();
        assert_eq!(depth_before.bid_levels.len(), 1);
        
        let result = orderbook.remove_order(order_id);
        assert!(result.is_some());
        
        let depth_after = orderbook.get_market_depth();
        assert!(depth_after.bid_levels.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_order() {
        let mut orderbook = OrderBook::new("TEST");
        let result = orderbook.remove_order(999999);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_best_bid_ask() {
        let mut orderbook = OrderBook::new("TEST");
        
        assert!(orderbook.get_best_bid_price().is_none());
        assert!(orderbook.get_best_ask_price().is_none());
        
        orderbook.add_order(create_test_order(Side::Buy, 100, 1000, 1)).unwrap();
        orderbook.add_order(create_test_order(Side::Buy, 95, 2000, 2)).unwrap();
        orderbook.add_order(create_test_order(Side::Sell, 110, 1000, 3)).unwrap();
        orderbook.add_order(create_test_order(Side::Sell, 115, 2000, 4)).unwrap();
        
        assert_eq!(orderbook.get_best_bid_price().unwrap(), 100);
        assert_eq!(orderbook.get_best_ask_price().unwrap(), 110);
    }

    #[test]
    fn test_market_depth_levels() {
        let mut orderbook = OrderBook::new("TEST");
        orderbook.set_depth_levels(5);
        
        for i in 0..20 {
            orderbook.add_order(create_test_order(Side::Buy, 100 - i, 1000, i as u64)).unwrap();
            orderbook.add_order(create_test_order(Side::Sell, 110 + i, 1000, (i + 20) as u64)).unwrap();
        }
        
        let depth = orderbook.get_market_depth();
        assert_eq!(depth.bid_levels.len(), 5);
        assert_eq!(depth.ask_levels.len(), 5);
        
        orderbook.set_depth_levels(10);
        let depth_10 = orderbook.get_market_depth();
        assert_eq!(depth_10.bid_levels.len(), 10);
        assert_eq!(depth_10.ask_levels.len(), 10);
    }

    #[test]
    fn test_orderbook_statistics() {
        let mut orderbook = OrderBook::new("TEST");
        
        orderbook.add_order(create_test_order(Side::Buy, 100, 1000, 1)).unwrap();
        orderbook.add_order(create_test_order(Side::Buy, 99, 2000, 2)).unwrap();
        orderbook.add_order(create_test_order(Side::Sell, 101, 1500, 3)).unwrap();
        orderbook.add_order(create_test_order(Side::Sell, 102, 2500, 4)).unwrap();
        
        let depth = orderbook.get_market_depth();
        let total_bid_volume: u64 = depth.bid_levels.iter().map(|&(_, qty)| qty).sum();
        let total_ask_volume: u64 = depth.ask_levels.iter().map(|&(_, qty)| qty).sum();
        
        assert_eq!(total_bid_volume, 3000);
        assert_eq!(total_ask_volume, 4000);
    }

    #[test]
    fn test_concurrent_order_operations() {
        let mut orderbook = OrderBook::new("TEST");
        orderbook.set_depth_levels(50);
        
        let orders: Vec<_> = (0..100).map(|i| {
            create_test_order(
                if i % 2 == 0 { Side::Buy } else { Side::Sell },
                if i % 2 == 0 { 100 - (i / 2) } else { 110 + (i / 2) },
                100,
                i as u64,
            )
        }).collect();
        
        for order in orders {
            orderbook.add_order(order).unwrap();
        }
        
        let depth = orderbook.get_market_depth();
        
        assert_eq!(depth.bid_levels.len(), 50);
        assert_eq!(depth.ask_levels.len(), 50);
        
        for i in 1..depth.bid_levels.len() {
            assert!(depth.bid_levels[i-1].0 > depth.bid_levels[i].0);
        }
        
        for i in 1..depth.ask_levels.len() {
            assert!(depth.ask_levels[i-1].0 < depth.ask_levels[i].0);
        }
    }
}