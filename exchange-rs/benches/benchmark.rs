use criterion::{criterion_group, criterion_main};

mod concurrent_benchmarks;
mod matching_engine_benchmarks;
mod order_benchmarks;

use concurrent_benchmarks::bench_concurrent_order_processing;
use matching_engine_benchmarks::bench_limit_order_matching;
use order_benchmarks::bench_order_pool;

criterion_group!(
    benches,
    bench_limit_order_matching,
    bench_order_pool,
    bench_concurrent_order_processing
);
criterion_main!(benches);
