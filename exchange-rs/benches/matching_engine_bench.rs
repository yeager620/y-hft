use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use exchange_rs::*;
use exchange_rs::order::*;
use exchange_rs::matching_engine::MatchingEngine;
use exchange_rs::price_utils::*;
use std::sync::Arc;
use std::time::Duration;

fn single_order_placement(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_order_placement");
    
    for symbol in ["BTCUSD", "ETHUSDT", "ADABNB"].iter() {
        group.bench_with_input(BenchmarkId::new("limit_order", symbol), symbol, |b, symbol| {
            b.iter_batched(
                || {
                    let mut engine = MatchingEngine::new();
                    engine.add_symbol(symbol);
                    (engine, Order::new(
                        symbol.to_string(),
                        Side::Buy,
                        OrderType::Limit,
                        50000000000,
                        1000,
                        1,
                    ))
                },
                |(mut engine, order)| {
                    black_box(engine.place_order(order).unwrap())
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

fn order_matching_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_matching");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);
    
    group.bench_function("matching_cross_spread", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                engine.add_symbol("TESTPAIR");
                
                let buy_order = Order::new(
                    "TESTPAIR".to_string(),
                    Side::Buy,
                    OrderType::Limit,
                    50000000000,
                    1000,
                    1,
                );
                
                let sell_order = Order::new(
                    "TESTPAIR".to_string(),
                    Side::Sell,
                    OrderType::Limit,
                    50000000000,
                    1000,
                    2,
                );
                
                engine.place_order(buy_order).unwrap();
                (engine, sell_order)
            },
            |(mut engine, sell_order)| {
                black_box(engine.place_order(sell_order).unwrap())
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

fn market_order_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("market_orders");
    
    for depth in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("market_buy", depth), depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut engine = MatchingEngine::new();
                    engine.add_symbol("DEPTH_TEST");
                    
                    for i in 0..depth {
                        let sell_order = Order::new(
                            "DEPTH_TEST".to_string(),
                            Side::Sell,
                            OrderType::Limit,
                            50000000000 + (i as u64 * 1000),
                            100,
                            i as u64,
                        );
                        engine.place_order(sell_order).unwrap();
                    }
                    
                    let market_order = Order::new(
                        "DEPTH_TEST".to_string(),
                        Side::Buy,
                        OrderType::Market,
                        0,
                        depth as u32 * 50,
                        depth as u64 + 1,
                    );
                    
                    (engine, market_order)
                },
                |(mut engine, market_order)| {
                    black_box(engine.place_order(market_order).unwrap())
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

fn orderbook_depth_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("orderbook_depth");
    group.throughput(Throughput::Elements(1));
    
    for levels in [5, 10, 20, 50].iter() {
        group.bench_with_input(BenchmarkId::new("get_market_depth", levels), levels, |b, &levels| {
            b.iter_batched(
                || {
                    let mut engine = MatchingEngine::new();
                    engine.add_symbol("DEPTH_SYMBOL");
                    
                    for i in 0..100 {
                        let buy_order = Order::new(
                            "DEPTH_SYMBOL".to_string(),
                            Side::Buy,
                            OrderType::Limit,
                            49999000000 - (i as u64 * 1000),
                            100,
                            i as u64,
                        );
                        engine.place_order(buy_order).unwrap();
                        
                        let sell_order = Order::new(
                            "DEPTH_SYMBOL".to_string(),
                            Side::Sell,
                            OrderType::Limit,
                            50001000000 + (i as u64 * 1000),
                            100,
                            (i + 100) as u64,
                        );
                        engine.place_order(sell_order).unwrap();
                    }
                    
                    engine
                },
                |engine| {
                    let orderbook = engine.order_books.get("DEPTH_SYMBOL").unwrap();
                    black_box(orderbook.get_market_depth())
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

fn concurrent_order_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_processing");
    group.sample_size(100);
    
    group.bench_function("parallel_non_matching_orders", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                engine.add_symbol("CONCURRENT_TEST");
                
                let orders: Vec<Order> = (0..1000).map(|i| {
                    Order::new(
                        "CONCURRENT_TEST".to_string(),
                        if i % 2 == 0 { Side::Buy } else { Side::Sell },
                        OrderType::Limit,
                        if i % 2 == 0 { 49000000000 - (i as u64 * 1000) } else { 51000000000 + (i as u64 * 1000) },
                        100,
                        i as u64,
                    )
                }).collect();
                
                (engine, orders)
            },
            |(mut engine, orders)| {
                for order in orders {
                    black_box(engine.place_order(order).unwrap());
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

fn price_utils_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("price_utils");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("float_to_scaled_price", |b| {
        b.iter(|| {
            black_box(float_to_scaled_price(black_box(123.456789)).unwrap())
        })
    });
    
    group.bench_function("scaled_price_to_float", |b| {
        b.iter(|| {
            black_box(scaled_price_to_float(black_box(123456789)))
        })
    });
    
    group.bench_function("float_to_scaled_quantity", |b| {
        b.iter(|| {
            black_box(float_to_scaled_quantity(black_box(10.5)).unwrap())
        })
    });
    
    group.finish();
}

fn iceberg_order_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("iceberg_orders");
    
    group.bench_function("iceberg_visibility_calculation", |b| {
        b.iter_batched(
            || {
                let mut iceberg = Order::new(
                    "ICEBERG_TEST".to_string(),
                    Side::Buy,
                    OrderType::Iceberg,
                    50000000000,
                    10000,
                    1,
                );
                iceberg.display_quantity = Some(1000);
                iceberg
            },
            |mut iceberg| {
                for i in 0..100 {
                    iceberg.filled_quantity = i * 50;
                    black_box(iceberg.visible_quantity());
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

fn stop_order_triggering(c: &mut Criterion) {
    let mut group = c.benchmark_group("stop_orders");
    
    group.bench_function("stop_trigger_evaluation", |b| {
        b.iter_batched(
            || {
                let mut stop_order = Order::new(
                    "STOP_TEST".to_string(),
                    Side::Buy,
                    OrderType::StopLimit,
                    52000000000,
                    1000,
                    1,
                );
                stop_order.stop_price = Some(51000000000);
                stop_order
            },
            |order| {
                for price in [50000000000u64, 50500000000, 51000000000, 51500000000, 52000000000].iter() {
                    black_box(order.is_stop_triggered(*price));
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

fn large_orderbook_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_tests");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(30));
    
    group.bench_function("large_orderbook_operations", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                engine.add_symbol("STRESS_TEST");
                
                for i in 0..10000 {
                    let buy_order = Order::new(
                        "STRESS_TEST".to_string(),
                        Side::Buy,
                        OrderType::Limit,
                        49000000000 - (i as u64 * 100),
                        100,
                        i as u64,
                    );
                    engine.place_order(buy_order).unwrap();
                    
                    let sell_order = Order::new(
                        "STRESS_TEST".to_string(),
                        Side::Sell,
                        OrderType::Limit,
                        51000000000 + (i as u64 * 100),
                        100,
                        (i + 10000) as u64,
                    );
                    engine.place_order(sell_order).unwrap();
                }
                
                engine
            },
            |mut engine| {
                let market_buy = Order::new(
                    "STRESS_TEST".to_string(),
                    Side::Buy,
                    OrderType::Market,
                    0,
                    500000,
                    20001,
                );
                black_box(engine.place_order(market_buy).unwrap())
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

criterion_group!(
    benches,
    single_order_placement,
    order_matching_latency,
    market_order_execution,
    orderbook_depth_performance,
    concurrent_order_processing,
    price_utils_performance,
    iceberg_order_processing,
    stop_order_triggering,
    large_orderbook_stress
);
criterion_main!(benches);