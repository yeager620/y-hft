# Exchange-RS

- fast limit order book
- order types: market, limit, stop, iceberg
- thread safety
- time-in-force options: GTC, IOC, FOK, GTD

three main components:
1. **Order Book** (`orderbook.rs`) manages all orders and matches
2. **Matching Engine** (`matching_engine.rs`) processes trades and handles order lifecycle
3. **Order Management** (`order.rs`) - order types and their states

## performance considerations
- lock-free data structures (where possible)
- minimal memory allocation
- cache-aware
- concurrent access patterns

**Run the Tests**
```bash
cargo test
```

**Check Performance**
```bash
cargo bench
```

## Next Steps

- [ ] API for order submission
- [ ] WebSocket feed for market data
- [ ] support for FIX
- [ ] book monitoring dashboard
- [ ] market making features
- [ ] risk mgmt?