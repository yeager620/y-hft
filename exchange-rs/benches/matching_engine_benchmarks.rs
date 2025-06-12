use criterion::Criterion;
use exchange_rs::{
    order::{Order, Side, OrderType},
    matching_engine::MatchingEngine,
};

pub fn bench_limit_order_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("matching_engine");

    group.bench_function("match_1000_orders", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::new();
            engine.add_symbol("AAPL");

            for i in 0..1000 {
                let side = if i % 2 == 0 { Side::Buy } else { Side::Sell };
                let price = 100 + (i % 10); 

                let order = Order::new(
                    "AAPL".to_string(),
                    side,
                    OrderType::Limit,
                    price,
                    1,
                    i,
                );

                let _ = engine.place_order(order);
            }
        })
    });

    group.bench_function("market_orders_vs_book", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::new();
            engine.add_symbol("AAPL");

            for i in 0..100 {
                let order = Order::new(
                    "AAPL".to_string(),
                    Side::Sell,
                    OrderType::Limit,
                    100 + i,
                    10,
                    i,
                );
                let _ = engine.place_order(order);
            }

            for i in 0..50 {
                let order = Order::new(
                    "AAPL".to_string(),
                    Side::Buy,
                    OrderType::Market,
                    0,
                    5,
                    i + 1000,
                );
                let _ = engine.place_order(order);
            }
        })
    });

    group.finish();
}
