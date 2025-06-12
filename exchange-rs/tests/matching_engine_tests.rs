use exchange_rs::{
    matching_engine::{MatchingEngine, MatchingError},
    order::{Order, OrderStatus, OrderType, Side, TimeInForce},
};

mod test_utils;
use test_utils::setup;

#[test]
fn test_limit_order_operations() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("AAPL");

    let sell_order = Order::new(
        "AAPL".to_string(),
        Side::Sell,
        OrderType::Limit,
        100,
        10,
        1,
    );

    let result = engine.place_order(sell_order).unwrap();
    assert_eq!(result.trades.len(), 0);
    assert!(result.remaining_order.is_some());

    let buy_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        5,
        2,
    );

    let result = engine.place_order(buy_order).unwrap();
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].quantity, 5);
    assert_eq!(result.trades[0].price, 100);
}

#[test]
fn test_market_orders() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("AAPL");

    let sell_order = Order::new(
        "AAPL".to_string(),
        Side::Sell,
        OrderType::Limit,
        100,
        10,
        1,
    );

    let result = engine.place_order(sell_order).unwrap();
    assert_eq!(result.trades.len(), 0);

    let market_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Market,
        0,
        5,
        2,
    );

    let result = engine.place_order(market_order).unwrap();
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].quantity, 5);
    assert_eq!(result.trades[0].price, 100);
}

#[test]
fn test_ioc_orders() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("AAPL");

    let sell_order = Order::new(
        "AAPL".to_string(),
        Side::Sell,
        OrderType::Limit,
        100,
        10,
        1,
    );

    engine.place_order(sell_order).unwrap();

    let mut ioc_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        15,
        2,
    );
    ioc_order.time_in_force = TimeInForce::IOC;

    let result = engine.place_order(ioc_order).unwrap();
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].quantity, 10);
    assert_eq!(result.filled_orders[0].read().status, OrderStatus::Canceled);
}

#[test]
fn test_fok_orders() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("AAPL");

    let mut fok_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        10,
        1,
    );
    fok_order.time_in_force = TimeInForce::FOK;

    let result = engine.place_order(fok_order);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), MatchingError::FOKCannotBeFilled);
}

#[test]
fn test_iceberg_orders() {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("AAPL");

    let mut iceberg_order = Order::new(
        "AAPL".to_string(),
        Side::Sell,
        OrderType::Iceberg,
        100,
        100,
        1,
    );
    iceberg_order.display_quantity = Some(10);

    engine.place_order(iceberg_order).unwrap();

    {
        let order_book = engine.order_books.get("AAPL").unwrap();
        let level = order_book.sell_levels.get(&100).unwrap();
        assert_eq!(level.visible_volume, 10);
    }

    let buy_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        5,
        2,
    );

    engine.place_order(buy_order).unwrap();

    let order_book = engine.order_books.get("AAPL").unwrap();
    let level = order_book.sell_levels.get(&100).unwrap();
    assert_eq!(level.visible_volume, 5);
}
