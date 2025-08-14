use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use exchange_rs::sbe::parser::{SbeMessageParser, SbeMessage};

fn sbe_message_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("sbe_parsing");
    group.throughput(Throughput::Bytes(64));
    
    group.bench_function("parse_instrument_message", |b| {
        let sample_data = create_sample_instrument_data();
        let parser = SbeMessageParser::new();
        
        b.iter(|| {
            black_box(parser.parse_message(black_box(&sample_data)).unwrap())
        })
    });
    
    group.bench_function("parse_book_message", |b| {
        let sample_data = create_sample_book_data();
        let parser = SbeMessageParser::new();
        
        b.iter(|| {
            black_box(parser.parse_message(black_box(&sample_data)).unwrap())
        })
    });
    
    group.bench_function("parse_ticker_message", |b| {
        let sample_data = create_sample_ticker_data();
        let parser = SbeMessageParser::new();
        
        b.iter(|| {
            black_box(parser.parse_message(black_box(&sample_data)).unwrap())
        })
    });
    
    group.finish();
}

fn batch_sbe_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_sbe");
    
    for batch_size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("parse_batch", batch_size), batch_size, |b, &size| {
            b.iter_batched(
                || {
                    let mut messages = Vec::new();
                    for _ in 0..size {
                        messages.push(create_sample_instrument_data());
                    }
                    messages
                },
                |messages| {
                    let parser = SbeMessageParser::new();
                    for message_data in messages {
                        black_box(parser.parse_message(&message_data).unwrap());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

fn sbe_message_type_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("sbe_message_types");
    
    let test_cases = vec![
        ("instrument", create_sample_instrument_data()),
        ("book", create_sample_book_data()),
        ("trades", create_sample_trades_data()),
        ("ticker", create_sample_ticker_data()),
        ("snapshot", create_sample_snapshot_data()),
    ];
    
    for (name, data) in test_cases {
        group.bench_with_input(BenchmarkId::new("parse", name), &data, |b, data| {
            b.iter(|| {
                let parser = SbeMessageParser::new();
                black_box(parser.parse_message(black_box(data)).unwrap())
            })
        });
    }
    
    group.finish();
}

fn create_sample_instrument_data() -> Vec<u8> {
    let mut buffer = vec![0u8; 256];
    
    buffer[0..2].copy_from_slice(&120u16.to_le_bytes());
    buffer[2..4].copy_from_slice(&0u16.to_le_bytes());
    buffer[4..6].copy_from_slice(&1000u16.to_le_bytes());
    buffer[6..8].copy_from_slice(&3u16.to_le_bytes());
    buffer[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    buffer[12..16].copy_from_slice(&12345u32.to_le_bytes());
    buffer[16] = 1;
    buffer[17] = 2;
    buffer[18] = 1;
    buffer[19] = 0;
    buffer[20] = 1;
    buffer[21] = 0;
    buffer[22..24].copy_from_slice(&100u16.to_le_bytes());
    
    for i in 24..120 {
        buffer[i] = 0;
    }
    
    buffer[62..70].copy_from_slice(&1692345678000u64.to_le_bytes());
    buffer[70..78].copy_from_slice(&1692345678000u64.to_le_bytes());
    
    buffer.truncate(132);
    buffer
}

fn create_sample_book_data() -> Vec<u8> {
    let mut buffer = vec![0u8; 256];
    
    buffer[0..2].copy_from_slice(&29u16.to_le_bytes());
    buffer[2..4].copy_from_slice(&0u16.to_le_bytes());
    buffer[4..6].copy_from_slice(&1001u16.to_le_bytes());
    buffer[6..8].copy_from_slice(&3u16.to_le_bytes());
    buffer[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    buffer[12..16].copy_from_slice(&67890u32.to_le_bytes());
    buffer[16..24].copy_from_slice(&1692345679000u64.to_le_bytes());
    buffer[24..32].copy_from_slice(&100u64.to_le_bytes());
    buffer[32..40].copy_from_slice(&101u64.to_le_bytes());
    buffer[40] = 1;
    
    buffer.truncate(41);
    buffer
}

fn create_sample_trades_data() -> Vec<u8> {
    let mut buffer = vec![0u8; 256];
    
    buffer[0..2].copy_from_slice(&4u16.to_le_bytes());
    buffer[2..4].copy_from_slice(&0u16.to_le_bytes());
    buffer[4..6].copy_from_slice(&1002u16.to_le_bytes());
    buffer[6..8].copy_from_slice(&3u16.to_le_bytes());
    buffer[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    buffer[12..16].copy_from_slice(&11111u32.to_le_bytes());
    
    buffer.truncate(16);
    buffer
}

fn create_sample_ticker_data() -> Vec<u8> {
    let mut buffer = vec![0u8; 256];
    
    buffer[0..2].copy_from_slice(&120u16.to_le_bytes());
    buffer[2..4].copy_from_slice(&0u16.to_le_bytes());
    buffer[4..6].copy_from_slice(&1003u16.to_le_bytes());
    buffer[6..8].copy_from_slice(&3u16.to_le_bytes());
    buffer[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    buffer[12..16].copy_from_slice(&22222u32.to_le_bytes());
    buffer[16] = 1;
    buffer[17..25].copy_from_slice(&1692345680000u64.to_le_bytes());
    
    for i in 25..132 {
        buffer[i] = 0;
    }
    
    buffer.truncate(132);
    buffer
}

fn create_sample_snapshot_data() -> Vec<u8> {
    let mut buffer = vec![0u8; 256];
    
    buffer[0..2].copy_from_slice(&20u16.to_le_bytes());
    buffer[2..4].copy_from_slice(&0u16.to_le_bytes());
    buffer[4..6].copy_from_slice(&1004u16.to_le_bytes());
    buffer[6..8].copy_from_slice(&3u16.to_le_bytes());
    buffer[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    buffer[12..16].copy_from_slice(&33333u32.to_le_bytes());
    buffer[16..24].copy_from_slice(&1692345681000u64.to_le_bytes());
    buffer[24..32].copy_from_slice(&200u64.to_le_bytes());
    
    buffer.truncate(32);
    buffer
}

criterion_group!(
    sbe_benches,
    sbe_message_parsing,
    batch_sbe_processing,
    sbe_message_type_parsing
);
criterion_main!(sbe_benches);