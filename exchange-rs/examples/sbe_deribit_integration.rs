use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::{sleep, Duration};

use exchange_rs::{
    sbe::{
        integration::{DeribitExchangeIntegration, DeribitMarketDataFeed},
        bridge::{SbeBridge, MarketDataUpdate},
        parser::{SbeMessageParser, SbeMessage, TickerMessage},
    },
    matching_engine::MatchingEngine,
    metrics::LatencyMetrics,
    optimizations::OptimizationConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    println!("Starting Deribit SBE Integration Example");
    let config = OptimizationConfig::default();
    let matching_engine = Arc::new(RwLock::new(MatchingEngine::new(config)));
    let latency_metrics = Arc::new(LatencyMetrics::new());

    let mut integration = DeribitExchangeIntegration::new(
        Arc::clone(&matching_engine),
        Arc::clone(&latency_metrics)
    );

    println!("Created Deribit integration components");

    integration.add_instrument_orderbook("BTC-PERPETUAL".to_string())?;
    integration.add_instrument_orderbook("ETH-PERPETUAL".to_string())?;

    println!("Added test instrument orderbooks");

    let sbe_bridge = SbeBridge::default();
    let ticker_message = SbeMessage::Ticker(TickerMessage {
        instrument_id: 1,
        instrument_state: 1, // Open
        timestamp_ms: 1640995200000,
        open_interest: Some(1000.0),
        min_sell_price: 40000.0,
        max_buy_price: 60000.0,
        last_price: Some(50000.0),
        index_price: 50005.0,
        mark_price: 50002.0,
        best_bid_price: 49995.0,
        best_bid_amount: 1.0,
        best_ask_price: 50005.0,
        best_ask_amount: 1.0,
        current_funding: Some(0.0001),
        funding_8h: Some(0.0008),
        estimated_delivery_price: None,
        delivery_price: None,
        settlement_price: None,
    });

    println!("Processing ticker message: {}", ticker_message);
    
    let market_updates = sbe_bridge.process_message(ticker_message)?;
    for update in market_updates {
        println!("Market Update: {:?}", update);
        integration.process_market_data_update(update).await?;
    }

    let mut feed = DeribitMarketDataFeed::new(
        Arc::clone(&matching_engine),
        Arc::clone(&latency_metrics)
    );

    feed.subscribe_to_instrument("BTC-PERPETUAL".to_string()).await?;
    feed.subscribe_to_instrument("ETH-PERPETUAL".to_string()).await?;
    
    println!("Subscribed to market data feeds");

    let instruments = integration.list_instruments();
    println!("Available instruments: {} total", instruments.len());
    for instrument in &instruments {
        println!("   - {} (ID: {})", instrument.name, instrument.id);
    }

    use exchange_rs::sbe::multicast::deribit;
    
    let configs = deribit::all_instruments_config();
    println!("Multicast configurations available:");
    for (i, config) in configs.iter().enumerate() {
        println!("   {}. {}:{}", i + 1, config.multicast_addr, config.port);
    }

    let parser = SbeMessageParser::new();
    println!("SBE Parser initialized - ready to parse binary messages");

    let mock_binary_data = create_mock_ticker_message();
    match parser.parse_message(&mock_binary_data) {
        Ok(message) => println!("Successfully parsed SBE message: {}", message),
        Err(e) => println!("Failed to parse mock message: {}", e),
    }

    println!("\nPerformance Metrics:");
    println!("   - Latency metrics: {:?}", latency_metrics.get_snapshot());
    
    println!("\nIntegration Status:");
    println!("   - Instruments registered: {}", integration.list_instruments().len());
    println!("   - Integration ready for multicast data");

    println!("\nSBE Integration example completed successfully!");
    println!("To use with real Deribit data, configure actual multicast addresses");
    
    Ok(())
}

fn create_mock_ticker_message() -> Vec<u8> {
    let mut data = Vec::with_capacity(128);
    
    data.extend_from_slice(&80u16.to_le_bytes());
    data.extend_from_slice(&1003u16.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes());
    data.extend_from_slice(&3u16.to_le_bytes());
    data.extend_from_slice(&0u16.to_le_bytes());
    data.extend_from_slice(&0u16.to_le_bytes());
    
    data.extend_from_slice(&123u32.to_le_bytes());
    data.push(1);
    data.extend_from_slice(&1640995200000u64.to_le_bytes());
    
    while data.len() < 132 {
        data.push(0);
    }
    
    data
}

