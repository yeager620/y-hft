use exchange_rs::order::{Order, Side, OrderType, TimeInForce, OrderStatus};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_order_creation() {
    let order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        10,
        1,
    );

    assert_eq!(order.symbol, "AAPL");
    assert_eq!(order.side, Side::Buy);
    assert_eq!(order.order_type, OrderType::Limit);
    assert_eq!(order.price, 100);
    assert_eq!(order.quantity, 10);
    assert_eq!(order.filled_quantity, 0);
    assert_eq!(order.status, OrderStatus::New);
}

#[test]
fn test_iceberg_order_visibility() {
    let mut order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Iceberg,
        100,
        100,
        1,
    );
    order.display_quantity = Some(10);

    assert_eq!(order.visible_quantity(), 10);
    order.filled_quantity = 95;
    assert_eq!(order.visible_quantity(), 5);
}

#[test]
fn test_order_expiration() {
    let mut order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Limit,
        100,
        10,
        1,
    );
    order.time_in_force = TimeInForce::GTD;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64;

    order.expiration_time = current_time - 1;
    assert!(order.is_expired(current_time));

    order.expiration_time = current_time + 1000000;
    assert!(!order.is_expired(current_time));
}
