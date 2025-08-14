use std::sync::Arc;
use parking_lot::RwLock;

use exchange_rs::matching_engine::MatchingEngine;
use exchange_rs::metrics::{OrderMetrics, LatencyMetrics};
use exchange_rs::sbe::simple::SimpleSbeManager;

#[tokio::test]
async fn test_simple_sbe_manager() {
    let order_metrics = Arc::new(OrderMetrics::new());
    let latency_metrics = Arc::new(LatencyMetrics::new());
    let matching_engine = Arc::new(RwLock::new(
        MatchingEngine::new()
    ));
    
    let mut manager = SimpleSbeManager::new(matching_engine);
    assert!(manager.start().await.is_ok());
    assert!(manager.stop().await.is_ok());
}

#[tokio::test]
async fn test_sbe_demo_function() {
    use exchange_rs::sbe::simple::run_sbe_demo;
    assert!(run_sbe_demo().await.is_ok());
}