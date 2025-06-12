use criterion::Criterion;
use exchange_rs::optimizations::OrderPool;

pub fn bench_order_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_pool");

    group.bench_function("allocate_and_free_10000_orders", |b| {
        b.iter(|| {
            let pool = OrderPool::new(1000);
            let mut orders = Vec::with_capacity(10000);

            for i in 0..10000 {
                let order = pool.acquire();
                {
                    let mut order_ref = order.write();
                    order_ref.id = i;
                    order_ref.symbol = "AAPL".to_string();
                    order_ref.price = 100;
                    order_ref.quantity = 10;
                }
                orders.push(order);
            }

            for order in orders {
                pool.release(order);
            }
        })
    });

    group.bench_function("high_reuse_scenario", |b| {
        b.iter(|| {
            let pool = OrderPool::new(100);

            for _ in 0..1000 {
                let order = pool.acquire();
                {
                    let mut order_ref = order.write();
                    order_ref.symbol = "AAPL".to_string();
                    order_ref.price = 100;
                    order_ref.quantity = 10;
                }
                pool.release(order);
            }
        })
    });

    group.finish();
}
