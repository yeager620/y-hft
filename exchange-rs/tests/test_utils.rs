use exchange_rs::matching_engine::MatchingEngine;

pub fn setup() -> MatchingEngine {
    let mut engine = MatchingEngine::new();
    engine.add_symbol("AAPL");
    engine
}
