use exchange_rs::order::{Order, OrderStatus, OrderType, Side, TimeInForce};
use exchange_rs::orderbook::{OrderBook, PriceLevel, StopOrderBook};
use parking_lot::RwLock;
use std::sync::Arc;

#[test]
fn test_price_level() {
    let mut level = PriceLevel::new(100);

    let mut order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
    order.id = 1;

    let order_arc = Arc::new(RwLock::new(order));

    level.add_order(Arc::clone(&order_arc));

    assert_eq!(level.total_volume, 10);
    assert_eq!(level.visible_volume, 10);

    let removed = level.remove_order(1);
    assert!(removed.is_some());

    assert_eq!(level.total_volume, 0);
    assert_eq!(level.visible_volume, 0);
}

#[test]
fn test_price_level_multiple_orders() {
    let mut level = PriceLevel::new(100);

    for i in 1..=5 {
        let mut order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, i);
        order.id = i;

        let order_arc = Arc::new(RwLock::new(order));
        level.add_order(Arc::clone(&order_arc));
    }

    assert_eq!(level.total_volume, 50);
    assert_eq!(level.visible_volume, 50);

    let removed = level.remove_order(3);
    assert!(removed.is_some());

    assert_eq!(level.total_volume, 40);
    assert_eq!(level.visible_volume, 40);

    assert_eq!(level.orders.len(), 4);
}

#[test]
fn test_order_book_basic() {
    let mut book = OrderBook::new("AAPL");

    let mut buy_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
    buy_order.id = 1;

    let buy_order_arc = Arc::new(RwLock::new(buy_order));

    book.add_order(Arc::clone(&buy_order_arc)).unwrap();

    assert_eq!(book.get_best_bid_price(), Some(100));

    let mut sell_order = Order::new("AAPL".to_string(), Side::Sell, OrderType::Limit, 110, 5, 2);
    sell_order.id = 2;

    let sell_order_arc = Arc::new(RwLock::new(sell_order));

    book.add_order(Arc::clone(&sell_order_arc)).unwrap();

    assert_eq!(book.get_best_ask_price(), Some(110));

    let canceled = book.cancel_order(1);
    assert!(canceled.is_some());

    assert_eq!(book.get_best_bid_price(), None);
}

#[test]
fn test_order_book_multiple_price_levels() {
    let mut book = OrderBook::new("AAPL");

    for i in 0..5 {
        let price = 100 - i;
        let mut order = Order::new(
            "AAPL".to_string(),
            Side::Buy,
            OrderType::Limit,
            price,
            10,
            i,
        );
        order.id = i;

        let order_arc = Arc::new(RwLock::new(order));
        book.add_order(Arc::clone(&order_arc)).unwrap();
    }

    for i in 5..10 {
        let price = 110 + (i - 5);
        let mut order = Order::new(
            "AAPL".to_string(),
            Side::Sell,
            OrderType::Limit,
            price,
            10,
            i,
        );
        order.id = i;

        let order_arc = Arc::new(RwLock::new(order));
        book.add_order(Arc::clone(&order_arc)).unwrap();
    }

    assert_eq!(book.get_best_bid_price(), Some(100));
    assert_eq!(book.get_best_ask_price(), Some(110));

    let removed = book.remove_order(0);
    assert!(removed.is_some());

    assert_eq!(book.get_best_bid_price(), Some(99));
}

#[test]
fn test_stop_order() {
    let mut book = OrderBook::new("AAPL");

    let mut stop_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::StopLimit,
        110,
        10,
        1,
    );
    stop_order.id = 1;
    stop_order.stop_price = Some(105);

    let stop_order_arc = Arc::new(RwLock::new(stop_order));

    book.add_stop_order(Arc::clone(&stop_order_arc)).unwrap();

    book.update_last_trade_price(100).unwrap();

    assert_eq!(book.get_best_bid_price(), None);

    book.update_last_trade_price(106).unwrap();

    assert_eq!(book.get_best_bid_price(), Some(110));

    let order = book.get_order(1).unwrap();
    assert_eq!(order.read().order_type, OrderType::Limit);
}

#[test]
fn test_stop_order_book() {
    let mut stop_book = StopOrderBook::new("AAPL");

    let mut buy_stop = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::StopLimit,
        110,
        10,
        1,
    );
    buy_stop.id = 1;
    buy_stop.stop_price = Some(105);

    let buy_stop_arc = Arc::new(RwLock::new(buy_stop));

    stop_book.add_stop_order(Arc::clone(&buy_stop_arc)).unwrap();

    let mut sell_stop = Order::new(
        "AAPL".to_string(),
        Side::Sell,
        OrderType::StopLimit,
        95,
        10,
        2,
    );
    sell_stop.id = 2;
    sell_stop.stop_price = Some(90);

    let sell_stop_arc = Arc::new(RwLock::new(sell_stop));

    stop_book
        .add_stop_order(Arc::clone(&sell_stop_arc))
        .unwrap();

    let triggered_at_100 = stop_book.get_triggered_orders(100);
    assert_eq!(triggered_at_100.len(), 0);

    let triggered_at_105 = stop_book.get_triggered_orders(105);
    assert_eq!(triggered_at_105.len(), 1);
    assert_eq!(triggered_at_105[0].read().id, 1);

    let triggered_at_90 = stop_book.get_triggered_orders(90);
    assert_eq!(triggered_at_90.len(), 1);
    assert_eq!(triggered_at_90[0].read().id, 2);

    let triggered_at_85 = stop_book.get_triggered_orders(85);
    assert_eq!(triggered_at_85.len(), 1);
    assert_eq!(triggered_at_85[0].read().id, 2);

    let removed = stop_book.remove_stop_order(1);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().read().id, 1);

    let triggered_after_remove = stop_book.get_triggered_orders(105);
    assert_eq!(triggered_after_remove.len(), 0);
}

#[test]
fn test_order_expiration() {
    let mut book = OrderBook::new("AAPL");

    let current_time = Order::get_nano_timestamp();
    let one_day_ns: i64 = 86_400_000_000_000;

    let mut gtd_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
    gtd_order.id = 1;
    gtd_order.time_in_force = TimeInForce::GTD;
    gtd_order.expiration_time = current_time + one_day_ns;

    let gtd_order_arc = Arc::new(RwLock::new(gtd_order));
    book.add_order(Arc::clone(&gtd_order_arc)).unwrap();

    let mut day_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 99, 10, 2);
    day_order.id = 2;
    day_order.time_in_force = TimeInForce::Day;

    let day_order_arc = Arc::new(RwLock::new(day_order));
    book.add_order(Arc::clone(&day_order_arc)).unwrap();

    assert!(book.get_order(1).is_some());
    assert!(book.get_order(2).is_some());

    let expired = book.expire_orders(current_time);
    assert_eq!(expired.len(), 0);

    let expired = book.expire_orders(current_time + one_day_ns + 1);
    assert_eq!(expired.len(), 2);

    assert!(book.get_order(1).is_none());
    assert!(book.get_order(2).is_none());

    assert_eq!(expired[0].read().status, OrderStatus::Expired);
    assert_eq!(expired[1].read().status, OrderStatus::Expired);
}

#[test]
fn test_iceberg_order() {
    let mut book = OrderBook::new("AAPL");

    let mut iceberg = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::Iceberg,
        100,
        100,
        1,
    );
    iceberg.id = 1;
    iceberg.display_quantity = Some(10);

    let iceberg_arc = Arc::new(RwLock::new(iceberg));

    book.add_order(Arc::clone(&iceberg_arc)).unwrap();

    {
        let level = book.buy_levels.get(&100).unwrap();
        assert_eq!(level.total_volume, 100);
        assert_eq!(level.get_visible_volume(), 10);
    }

    {
        let mut order_ref = iceberg_arc.write();
        order_ref.filled_quantity = 5;
    }

    book.replenish_iceberg_order(Arc::clone(&iceberg_arc))
        .unwrap();

    {
        let level = book.buy_levels.get(&100).unwrap();
        assert_eq!(level.get_visible_volume(), 5);
    }

    {
        let mut order_ref = iceberg_arc.write();
        order_ref.filled_quantity = 10;
    }

    book.replenish_iceberg_order(Arc::clone(&iceberg_arc))
        .unwrap();

    {
        let level = book.buy_levels.get(&100).unwrap();
        assert_eq!(level.get_visible_volume(), 10);
    }

    {
        let mut order_ref = iceberg_arc.write();
        order_ref.filled_quantity = 95;
    }

    book.replenish_iceberg_order(Arc::clone(&iceberg_arc))
        .unwrap();

    {
        let level = book.buy_levels.get(&100).unwrap();
        assert_eq!(level.get_visible_volume(), 5);
    }
}

#[test]
fn test_empty_order_book() {
    let book = OrderBook::new("AAPL");

    assert_eq!(book.buy_levels.len(), 0);
    assert_eq!(book.sell_levels.len(), 0);

    assert_eq!(book.get_best_bid_price(), None);
    assert_eq!(book.get_best_ask_price(), None);

    assert_eq!(book.last_trade_price, None);
}

#[test]
fn test_order_book_error_handling() {
    let mut book = OrderBook::new("AAPL");

    let mut regular_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 100, 10, 1);
    regular_order.id = 1;

    let regular_order_arc = Arc::new(RwLock::new(regular_order));

    let result = book.add_stop_order(Arc::clone(&regular_order_arc));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Not a stop order");

    let mut stop_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::StopLimit,
        100,
        10,
        2,
    );
    stop_order.id = 2;

    let stop_order_arc = Arc::new(RwLock::new(stop_order));

    let result = book.add_stop_order(Arc::clone(&stop_order_arc));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Missing stop price");
}
