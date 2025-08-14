mod matching_engine;
mod optimizations;
mod order;
mod orderbook;
mod metrics;
mod snapshot;
mod fix;
mod fix_gateway;

use parking_lot::Mutex;
use std::sync::Arc;

use matching_engine::MatchingEngine;
use optimizations::{OrderPool, OrderProcessorPool};
use order::{Order, OrderType, Side};
use fix_gateway::FixGateway;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    println!("Exchange-RS: High-performance limit order book implementation with FIX support");

    let engine = Arc::new(Mutex::new(MatchingEngine::new()));

    {
        let mut engine_ref = engine.lock();
        engine_ref.add_symbol("AAPL");
        engine_ref.add_symbol("GOOGL");
        engine_ref.add_symbol("MSFT");
        engine_ref.add_symbol("TSLA");
        engine_ref.add_symbol("NVDA");
    }

    let num_workers = num_cpus::get();
    println!("Starting order processor pool with {} workers", num_workers);
    let pool = OrderProcessorPool::new(num_workers, Arc::clone(&engine));

    let order_pool = OrderPool::new(1000);
    println!("Created order pool with initial capacity of 1000 orders");

    println!("\nRunning standard order demo...");
    run_standard_demo(&pool).await;

    println!("\nStarting FIX gateway on 0.0.0.0:9878...");
    let mut fix_gateway = FixGateway::new(Arc::clone(&engine));
    fix_gateway.add_symbol("AAPL");
    fix_gateway.add_symbol("GOOGL");
    fix_gateway.add_symbol("MSFT");
    fix_gateway.add_symbol("TSLA");
    fix_gateway.add_symbol("NVDA");

    println!("FIX gateway ready! Connect FIX clients to 0.0.0.0:9878");
    
    if let Err(e) = fix_gateway.start_server("0.0.0.0:9878").await {
        eprintln!("FIX gateway error: {}", e);
    }
}

async fn run_standard_demo(pool: &OrderProcessorPool) {
    let sell_order = Order::new("AAPL".to_string(), Side::Sell, OrderType::Limit, 1000000, 10, 1);
    println!("Submitting sell order: 10 shares of AAPL at $100.00");
    if let Err(e) = pool.submit_order(sell_order) {
        eprintln!("Error submitting sell order: {}", e);
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let buy_order = Order::new("AAPL".to_string(), Side::Buy, OrderType::Limit, 1000000, 5, 2);
    println!("Submitting buy order: 5 shares of AAPL at $100.00");
    if let Err(e) = pool.submit_order(buy_order) {
        eprintln!("Error submitting buy order: {}", e);
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let mut stop_order = Order::new(
        "AAPL".to_string(),
        Side::Buy,
        OrderType::StopLimit,
        1100000,
        10,
        3,
    );
    stop_order.stop_price = Some(1050000);
    println!("Submitting stop buy order: 10 shares of AAPL at $110.00, stop price $105.00");
    if let Err(e) = pool.submit_order(stop_order) {
        eprintln!("Error submitting stop order: {}", e);
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let sell_order_2 = Order::new("AAPL".to_string(), Side::Sell, OrderType::Limit, 1050000, 5, 4);
    println!("Submitting sell order: 5 shares of AAPL at $105.00 (should trigger stop order)");
    if let Err(e) = pool.submit_order(sell_order_2) {
        eprintln!("Error submitting sell order: {}", e);
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    println!("Standard demo completed successfully!");
}
