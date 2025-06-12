use criterion::Criterion;
use exchange_rs::{
    matching_engine::MatchingEngine,
    optimizations::OrderProcessorPool,
    order::{Order, OrderType, Side},
};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn bench_concurrent_order_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_processing");

    group.bench_function("process_1000_orders_concurrent", |b| {
        b.iter(|| {
            let engine = Arc::new(Mutex::new(MatchingEngine::new()));

            {
                let mut engine_ref = engine.lock();
                engine_ref.add_symbol("AAPL");
            }

            let pool = OrderProcessorPool::new(4, Arc::clone(&engine));

            for i in 0..1000 {
                let side = if i % 2 == 0 { Side::Buy } else { Side::Sell };
                let price = 100 + (i % 10);

                let order = Order::new("AAPL".to_string(), side, OrderType::Limit, price, 1, i);

                let _ = pool.submit_order(order);
            }
        })
    });

    group.bench_function("concurrent_market_data", |b| {
        b.iter(|| {
            let engine = Arc::new(Mutex::new(MatchingEngine::new()));
            let pool = OrderProcessorPool::new(4, Arc::clone(&engine));

            for i in 0..100 {
                let limit_order = Order::new(
                    "AAPL".to_string(),
                    Side::Sell,
                    OrderType::Limit,
                    100 + (i % 5),
                    10,
                    i,
                );
                let _ = pool.submit_order(limit_order);

                if i % 10 == 0 {
                    let market_order = Order::new(
                        "AAPL".to_string(),
                        Side::Buy,
                        OrderType::Market,
                        0,
                        5,
                        i + 1000,
                    );
                    let _ = pool.submit_order(market_order);
                }
            }
        })
    });

    group.finish();
}
