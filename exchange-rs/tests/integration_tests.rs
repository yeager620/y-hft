use exchange_rs::order::*;
use exchange_rs::orderbook::OrderBook;
use exchange_rs::matching_engine::MatchingEngine;
use exchange_rs::price_utils::*;
use std::sync::Arc;
use parking_lot::RwLock;

#[test]
fn test_full_order_lifecycle() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("BTCUSD");
    
    let buy_order = Order::new(
        "BTCUSD".to_string(),
        Side::Buy,
        OrderType::Limit,
        50000000000,
        1000,
        1,
    );
    
    let sell_order = Order::new(
        "BTCUSD".to_string(),
        Side::Sell,
        OrderType::Limit,
        50000000000,
        800,
        2,
    );
    
    // Place the buy order first
    let result1 = engine.place_order(buy_order).unwrap();
    assert!(result1.trades.is_empty());
    assert!(result1.remaining_order.is_some());
    
    // Place matching sell order
    let result2 = engine.place_order(sell_order).unwrap();
    assert_eq!(result2.trades.len(), 1);
    assert_eq!(result2.trades[0].quantity, 800);
    assert_eq!(result2.trades[0].price, 50000000000);
    assert!(result2.remaining_order.is_none());
}

#[test]
fn test_orderbook_operations() {
    let mut orderbook = OrderBook::new("TESTPAIR");
    
    let order1 = Arc::new(RwLock::new(Order::new(
        "TESTPAIR".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        1000,
        1,
    )));
    
    let order2 = Arc::new(RwLock::new(Order::new(
        "TESTPAIR".to_string(),
        Side::Sell,
        OrderType::Limit,
        110,
        1500,
        2,
    )));
    
    orderbook.add_order(order1).unwrap();
    orderbook.add_order(order2).unwrap();
    
    let depth = orderbook.get_market_depth();
    assert_eq!(depth.bid_levels.len(), 1);
    assert_eq!(depth.ask_levels.len(), 1);
    assert_eq!(depth.bid_levels[0].0, 100);
    assert_eq!(depth.ask_levels[0].0, 110);
}

#[test]
fn test_price_conversion_utils() {
    let price_float = 123.456789;
    let scaled_price = float_to_scaled_price(price_float).unwrap();
    assert_eq!(scaled_price, 123456789);
    
    let converted_back = scaled_price_to_float(scaled_price);
    assert!((converted_back - price_float).abs() < 1e-6);
    
    let quantity_float = 10.5;
    let scaled_quantity = float_to_scaled_quantity(quantity_float).unwrap();
    assert_eq!(scaled_quantity, 10500);
    
    let converted_quantity = scaled_quantity_to_float(scaled_quantity);
    assert!((converted_quantity - quantity_float).abs() < 1e-3);
}

#[test]
fn test_market_order_execution() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("ADAUSDT");
    
    let limit_order = Order::new(
        "ADAUSDT".to_string(),
        Side::Sell,
        OrderType::Limit,
        500000000,
        2000,
        1,
    );
    
    let market_order = Order::new(
        "ADAUSDT".to_string(),
        Side::Buy,
        OrderType::Market,
        0,
        1000,
        2,
    );
    
    engine.place_order(limit_order).unwrap();
    let result = engine.place_order(market_order).unwrap();
    
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].quantity, 1000);
    assert_eq!(result.trades[0].price, 500000000);
    assert!(result.remaining_order.is_none());
}

#[test]
fn test_partial_fill_scenario() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("ETHUSDT");
    
    let large_order = Order::new(
        "ETHUSDT".to_string(),
        Side::Buy,
        OrderType::Limit,
        2000000000000,
        5000,
        1,
    );
    
    let small_order = Order::new(
        "ETHUSDT".to_string(),
        Side::Sell,
        OrderType::Limit,
        2000000000000,
        2000,
        2,
    );
    
    engine.place_order(large_order).unwrap();
    let result = engine.place_order(small_order).unwrap();
    
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].quantity, 2000);
    assert!(result.remaining_order.is_none());
}

#[test]
fn test_stop_order_validation() {
    let mut stop_order = Order::new(
        "TEST".to_string(),
        Side::Buy,
        OrderType::StopLimit,
        110,
        1000,
        1,
    );
    stop_order.stop_price = Some(105);
    
    assert!(!stop_order.is_stop_triggered(100));
    assert!(!stop_order.is_stop_triggered(104));
    assert!(stop_order.is_stop_triggered(105));
    assert!(stop_order.is_stop_triggered(110));
}

#[test]
fn test_time_in_force_validation() {
    let mut ioc_order = Order::new(
        "TEST".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        1000,
        1,
    );
    ioc_order.time_in_force = TimeInForce::IOC;
    
    assert_eq!(ioc_order.time_in_force, TimeInForce::IOC);
    
    let mut fok_order = Order::new(
        "TEST".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        1000,
        2,
    );
    fok_order.time_in_force = TimeInForce::FOK;
    
    assert_eq!(fok_order.time_in_force, TimeInForce::FOK);
}

#[test]
fn test_iceberg_order_visibility() {
    let mut iceberg = Order::new(
        "TEST".to_string(),
        Side::Buy,
        OrderType::Iceberg,
        100,
        10000,
        1,
    );
    iceberg.display_quantity = Some(2000);
    
    assert_eq!(iceberg.visible_quantity(), 2000);
    
    iceberg.filled_quantity = 8500;
    assert_eq!(iceberg.visible_quantity(), 1500);
}

#[test]
fn test_cross_platform_compatibility() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("DOGEUSDT");
    
    let order = Order::new(
        "DOGEUSDT".to_string(),
        Side::Buy,
        OrderType::Limit,
        8000000,
        10000,
        1,
    );
    
    let result = engine.place_order(order);
    assert!(result.is_ok());
    
    let timestamp = Order::get_nano_timestamp();
    assert!(timestamp > 0);
}