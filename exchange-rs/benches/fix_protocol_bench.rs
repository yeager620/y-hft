use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use exchange_rs::fix::parser::{FixParser, FixField};
use exchange_rs::matching_engine::MatchingEngine;
use std::collections::HashMap;

fn fix_message_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("fix_parsing");
    group.throughput(Throughput::Bytes(64));
    
    group.bench_function("parse_new_order_single", |b| {
        let sample_data = create_sample_new_order_single();
        let mut parser = FixParser::new();
        
        b.iter(|| {
            black_box(parser.parse(black_box(&sample_data)))
        })
    });
    
    group.bench_function("parse_execution_report", |b| {
        let sample_data = create_sample_execution_report();
        let mut parser = FixParser::new();
        
        b.iter(|| {
            black_box(parser.parse(black_box(&sample_data)))
        })
    });
    
    group.bench_function("parse_order_cancel", |b| {
        let sample_data = create_sample_order_cancel();
        let mut parser = FixParser::new();
        
        b.iter(|| {
            black_box(parser.parse(black_box(&sample_data)))
        })
    });
    
    group.finish();
}

fn fix_validation_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("fix_validation");
    
    group.bench_function("validate_checksum", |b| {
        let sample_data = create_sample_new_order_single();
        let parser = FixParser::new();
        
        b.iter(|| {
            black_box(parser.validate_checksum(black_box(&sample_data)))
        })
    });
    
    group.bench_function("validate_body_length", |b| {
        let sample_data = create_sample_execution_report();
        let parser = FixParser::new();
        
        b.iter(|| {
            black_box(parser.validate_body_length(black_box(&sample_data)))
        })
    });
    
    group.finish();
}

fn fix_header_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("fix_header");
    
    group.bench_function("extract_header_fields", |b| {
        let sample_data = create_sample_new_order_single();
        let mut parser = FixParser::new();
        
        b.iter(|| {
            black_box(parser.extract_header_fields(black_box(&sample_data)))
        })
    });
    
    group.finish();
}

fn batch_fix_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_fix");
    
    for batch_size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("parse_batch", batch_size), batch_size, |b, &size| {
            b.iter_batched(
                || {
                    let mut messages = Vec::new();
                    for _ in 0..size {
                        messages.push(create_sample_new_order_single());
                    }
                    messages
                },
                |messages| {
                    let mut parser = FixParser::new();
                    for message_data in messages {
                        black_box(parser.parse(&message_data));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

fn fix_order_processing_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("fix_integration");
    
    group.bench_function("order_lifecycle", |b| {
        b.iter_batched(
            || {
                let mut engine = MatchingEngine::new();
                engine.add_symbol("BTCUSD");
                (engine, create_sample_new_order_single())
            },
            |(mut engine, fix_data)| {
                let mut parser = FixParser::new();
                let parsed = parser.parse(&fix_data);
                black_box(parsed);
                
                let order = exchange_rs::order::Order::new(
                    "BTCUSD".to_string(),
                    exchange_rs::order::Side::Buy,
                    exchange_rs::order::OrderType::Limit,
                    50000000000,
                    1000,
                    1,
                );
                black_box(engine.place_order(order))
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

fn create_sample_new_order_single() -> Vec<u8> {
    let fix_message = "8=FIX.4.2\x019=196\x0135=D\x0149=SENDER\x0156=TARGET\x0134=1\x01\
        52=20230815-12:30:45.123\x0111=ORDER123\x0155=BTCUSD\x0154=1\x0138=1000\x0140=2\x01\
        44=50000.00\x0159=0\x0122=8\x01207=XNAS\x0110=123\x01";
    
    fix_message.as_bytes().to_vec()
}

fn create_sample_execution_report() -> Vec<u8> {
    let fix_message = "8=FIX.4.2\x019=250\x0135=8\x0149=TARGET\x0156=SENDER\x0134=2\x01\
        52=20230815-12:31:00.456\x0111=ORDER123\x0137=EXEC123\x0117=EXECID123\x01\
        20=0\x01150=0\x0139=0\x0155=BTCUSD\x0154=1\x0138=1000\x0140=2\x0144=50000.00\x01\
        32=1000\x0131=50000.00\x0114=0\x016=0\x0151=0\x0160=20230815-12:31:00.456\x0110=234\x01";
    
    fix_message.as_bytes().to_vec()
}

fn create_sample_order_cancel() -> Vec<u8> {
    let fix_message = "8=FIX.4.2\x019=156\x0135=F\x0149=SENDER\x0156=TARGET\x0134=3\x01\
        52=20230815-12:32:15.789\x0111=CANCEL123\x0141=ORDER123\x0155=BTCUSD\x01\
        54=1\x0138=1000\x0160=20230815-12:32:15.789\x0110=145\x01";
    
    fix_message.as_bytes().to_vec()
}

criterion_group!(
    fix_benches,
    fix_message_parsing,
    fix_validation_performance,
    fix_header_extraction,
    batch_fix_processing,
    fix_order_processing_integration
);
criterion_main!(fix_benches);