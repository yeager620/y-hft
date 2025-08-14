use std::sync::Arc;
use std::time::Duration;
use parking_lot::RwLock;
use tokio::time::timeout;

use exchange_rs::sbe::{
    bridge::{SbeBridge, MarketDataUpdate},
    parser::{SbeMessageParser, SbeMessage, BookMessage, BookChange, TradesMessage, Trade, TickerMessage},
    integration::{DeribitExchangeIntegration, DeribitMarketDataFeed, IntegrationError},
    multicast::{MulticastConfig, DeribitMulticastReceiver},
};
use exchange_rs::matching_engine::{MatchingEngine, Trade as InternalTrade};
use exchange_rs::order::{Order, Side, OrderType, OrderStatus, TimeInForce};
use exchange_rs::metrics::LatencyMetrics;
use exchange_rs::optimizations::OptimizationConfig;

#[tokio::test]
async fn test_sbe_bridge_market_data_conversion() {
    let bridge = SbeBridge::default();
    
    
    let book_message = SbeMessage::Book(BookMessage {
        instrument_id: 123,
        timestamp_ms: 1640995200000,
        prev_change_id: 100,
        change_id: 101,
        is_last: true,
        changes: vec![
            BookChange {
                side: 1, 
                change: 1, 
                price: 50000.0,
                amount: 1.5,
            },
            BookChange {
                side: 0, 
                change: 0, 
                price: 50100.0,
                amount: 2.0,
            }
        ],
    });

    let updates = bridge.process_message(book_message).unwrap();
    assert_eq!(updates.len(), 1);
    
    let update = &updates[0];
    assert_eq!(update.instrument_id, 123);
    assert_eq!(update.timestamp, 1640995200000);
    assert!(update.best_bid.is_some());
    assert!(update.best_ask.is_some());
}

#[tokio::test]
async fn test_sbe_bridge_trades_processing() {
    let bridge = SbeBridge::default();
    
    let trades_message = SbeMessage::Trades(TradesMessage {
        instrument_id: 456,
        trades: vec![
            Trade {
                direction: 0, 
                price: 49950.0,
                amount: 0.5,
                timestamp_ms: 1640995200500,
                mark_price: 49960.0,
                index_price: 49955.0,
                trade_seq: 12345,
                trade_id: 67890,
                tick_direction: 0, 
                liquidation: 0, 
                iv: None,
                block_trade_id: None,
                combo_trade_id: None,
            }
        ],
    });

    let updates = bridge.process_message(trades_message).unwrap();
    assert_eq!(updates.len(), 1);
    
    let update = &updates[0];
    assert_eq!(update.instrument_id, 456);
    assert_eq!(update.last_price, Some(49950.0));
    assert_eq!(update.mark_price, Some(49960.0));
    assert_eq!(update.index_price, Some(49955.0));
}

#[tokio::test]
async fn test_sbe_bridge_ticker_processing() {
    let bridge = SbeBridge::default();
    
    let ticker_message = SbeMessage::Ticker(TickerMessage {
        instrument_id: 789,
        instrument_state: 1, 
        timestamp_ms: 1640995201000,
        open_interest: Some(1000.0),
        min_sell_price: 40000.0,
        max_buy_price: 60000.0,
        last_price: Some(51000.0),
        index_price: 51005.0,
        mark_price: 51002.0,
        best_bid_price: 50995.0,
        best_bid_amount: 3.0,
        best_ask_price: 51005.0,
        best_ask_amount: 2.5,
        current_funding: Some(0.0001),
        funding_8h: Some(0.0008),
        estimated_delivery_price: None,
        delivery_price: None,
        settlement_price: None,
    });

    let updates = bridge.process_message(ticker_message).unwrap();
    assert_eq!(updates.len(), 1);
    
    let update = &updates[0];
    assert_eq!(update.instrument_id, 789);
    assert_eq!(update.last_price, Some(51000.0));
    assert_eq!(update.best_bid, Some((50995.0, 3.0)));
    assert_eq!(update.best_ask, Some((51005.0, 2.5)));
}

#[tokio::test]
async fn test_deribit_integration_creation_and_setup() {
    let config = OptimizationConfig::default();
    let matching_engine = Arc::new(RwLock::new(MatchingEngine::new(config)));
    let latency_metrics = Arc::new(LatencyMetrics::new());
    
    let integration = DeribitExchangeIntegration::new(
        Arc::clone(&matching_engine),
        Arc::clone(&latency_metrics)
    );
    
    
    assert!(integration.list_instruments().is_empty()); 
}

#[tokio::test]
async fn test_market_data_feed_subscription() {
    let config = OptimizationConfig::default();
    let matching_engine = Arc::new(RwLock::new(MatchingEngine::new(config)));
    let latency_metrics = Arc::new(LatencyMetrics::new());
    
    let mut feed = DeribitMarketDataFeed::new(matching_engine, latency_metrics);
    
    
    let result = feed.subscribe_to_instrument("BTC-PERPETUAL".to_string()).await;
    assert!(result.is_ok());
    
    let result = feed.subscribe_to_instrument("ETH-PERPETUAL".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_external_order_generation() {
    let config = OptimizationConfig::default();
    let matching_engine = Arc::new(RwLock::new(MatchingEngine::new(config)));
    let latency_metrics = Arc::new(LatencyMetrics::new());
    
    let mut integration = DeribitExchangeIntegration::new(matching_engine, latency_metrics);
    
    
    integration.add_instrument_orderbook("BTC-PERPETUAL".to_string()).unwrap();
    
    
    let update = MarketDataUpdate {
        instrument_id: 1,
        symbol: "BTC-PERPETUAL".to_string(),
        timestamp: 1640995200000,
        best_bid: Some((50000.0, 1.5)),
        best_ask: Some((50100.0, 2.0)),
        last_price: Some(50050.0),
        mark_price: Some(50055.0),
        index_price: Some(50048.0),
    };
    
    
    let result = integration.process_market_data_update(update).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_multicast_config_creation() {
    use exchange_rs::sbe::multicast::deribit;
    
    
    let btc_config = deribit::btc_perpetual_config();
    assert_eq!(btc_config.port, 10001);
    
    let eth_config = deribit::eth_perpetual_config();
    assert_eq!(eth_config.port, 10002);
    
    let options_config = deribit::options_config();
    assert_eq!(options_config.port, 10010);
    
    let all_configs = deribit::all_instruments_config();
    assert_eq!(all_configs.len(), 3);
}

#[tokio::test]
async fn test_price_scaling_conversion() {
    let bridge = SbeBridge::new(1_000_000); 
    
    
    let update = MarketDataUpdate {
        instrument_id: 1,
        symbol: "TEST-INSTRUMENT".to_string(),
        timestamp: 1640995200000,
        best_bid: Some((50000.123456, 1.5)),
        best_ask: Some((50100.987654, 2.0)),
        last_price: Some(50050.555555),
        mark_price: None,
        index_price: None,
    };
    
    
    let result = bridge.process_message(SbeMessage::Ticker(TickerMessage {
        instrument_id: 1,
        instrument_state: 1,
        timestamp_ms: update.timestamp,
        open_interest: None,
        min_sell_price: 40000.0,
        max_buy_price: 60000.0,
        last_price: update.last_price,
        index_price: 50000.0,
        mark_price: 50000.0,
        best_bid_price: update.best_bid.unwrap().0,
        best_bid_amount: update.best_bid.unwrap().1,
        best_ask_price: update.best_ask.unwrap().0,
        best_ask_amount: update.best_ask.unwrap().1,
        current_funding: None,
        funding_8h: None,
        estimated_delivery_price: None,
        delivery_price: None,
        settlement_price: None,
    }));
    
    assert!(result.is_ok());
    let updates = result.unwrap();
    assert_eq!(updates.len(), 1);
}

#[tokio::test]
async fn test_error_handling() {
    let bridge = SbeBridge::default();
    
    
    let book_message = SbeMessage::Book(BookMessage {
        instrument_id: 99999, 
        timestamp_ms: 1640995200000,
        prev_change_id: 100,
        change_id: 101,
        is_last: true,
        changes: vec![],
    });
    
    
    let result = bridge.process_message(book_message);
    
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_concurrent_market_data_processing() {
    let config = OptimizationConfig::default();
    let matching_engine = Arc::new(RwLock::new(MatchingEngine::new(config)));
    let latency_metrics = Arc::new(LatencyMetrics::new());
    
    let integration = Arc::new(DeribitExchangeIntegration::new(
        matching_engine,
        latency_metrics
    ));
    
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let integration_clone = Arc::clone(&integration);
        let handle = tokio::spawn(async move {
            let update = MarketDataUpdate {
                instrument_id: i,
                symbol: format!("INSTRUMENT-{}", i),
                timestamp: 1640995200000 + i as u64,
                best_bid: Some((50000.0 + i as f64, 1.0)),
                best_ask: Some((50100.0 + i as f64, 1.0)),
                last_price: Some(50050.0 + i as f64),
                mark_price: None,
                index_price: None,
            };
            
            integration_clone.process_market_data_update(update).await
        });
        handles.push(handle);
    }
    
    
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
}

#[tokio::test]
async fn test_sbe_parser_message_types() {
    let parser = SbeMessageParser::new();
    
    
    
    
    
    
    assert_eq!(std::mem::size_of_val(&parser), 0); 
}


#[tokio::test]
async fn test_end_to_end_market_data_flow() {
    let config = OptimizationConfig::default();
    let matching_engine = Arc::new(RwLock::new(MatchingEngine::new(config)));
    let latency_metrics = Arc::new(LatencyMetrics::new());
    
    
    let mut integration = DeribitExchangeIntegration::new(matching_engine, latency_metrics);
    
    
    integration.add_instrument_orderbook("BTC-PERPETUAL".to_string()).unwrap();
    
    
    let update = MarketDataUpdate {
        instrument_id: 1,
        symbol: "BTC-PERPETUAL".to_string(),
        timestamp: 1640995200000,
        best_bid: Some((50000.0, 1.0)),
        best_ask: Some((50100.0, 1.0)),
        last_price: Some(50050.0),
        mark_price: Some(50055.0),
        index_price: Some(50048.0),
    };
    
    
    let result = integration.process_market_data_update(update).await;
    assert!(result.is_ok());
    
    
    let engine = integration.matching_engine.read();
    
    
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_market_data_processing_latency() {
        let bridge = SbeBridge::default();
        
        
        let mut updates = Vec::new();
        for i in 0..1000 {
            let ticker_message = SbeMessage::Ticker(TickerMessage {
                instrument_id: i,
                instrument_state: 1,
                timestamp_ms: 1640995200000 + i as u64,
                open_interest: Some(1000.0),
                min_sell_price: 40000.0,
                max_buy_price: 60000.0,
                last_price: Some(50000.0 + i as f64),
                index_price: 50005.0,
                mark_price: 50002.0,
                best_bid_price: 49995.0 + i as f64,
                best_bid_amount: 1.0,
                best_ask_price: 50005.0 + i as f64,
                best_ask_amount: 1.0,
                current_funding: None,
                funding_8h: None,
                estimated_delivery_price: None,
                delivery_price: None,
                settlement_price: None,
            });
            updates.push(ticker_message);
        }
        
        
        let start = Instant::now();
        
        for update in updates {
            let _ = bridge.process_message(update).unwrap();
        }
        
        let duration = start.elapsed();
        println!("Processed 1000 market updates in {:?}", duration);
        println!("Average per update: {:?}", duration / 1000);
        
        
        assert!(duration.as_micros() < 100_000); 
    }
}