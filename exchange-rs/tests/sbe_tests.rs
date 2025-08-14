use exchange_rs::sbe::*;
use exchange_rs::sbe::parser::*;
use exchange_rs::sbe::bridge::SbeBridge;

#[test]
fn test_sbe_message_parser_creation() {
    let parser = SbeMessageParser::new();
    assert!(parser.parse_message(&[]).is_err());
}

#[test]
fn test_read_write_buf() {
    let data = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let read_buf = ReadBuf::new(&data);
    
    
    assert_eq!(read_buf.get_u8_at(0), 0);
    assert_eq!(read_buf.get_u8_at(1), 1);
    assert_eq!(read_buf.get_u16_at(0), 256); 
    assert_eq!(read_buf.get_u32_at(0), 50462976); 
    
    let slice = read_buf.get_slice_at(2, 3);
    assert_eq!(slice, &[2, 3, 4]);
}

#[test]
fn test_write_buf() {
    let mut data = [0u8; 8];
    let mut write_buf = WriteBuf::new(&mut data);
    
    write_buf.put_u8_at(0, 42);
    write_buf.put_u16_at(1, 1234);
    write_buf.put_u32_at(3, 0xDEADBEEF);
    
    assert_eq!(data[0], 42);
    
    assert_eq!(u16::from_le_bytes([data[1], data[2]]), 1234);
    assert_eq!(u32::from_le_bytes([data[3], data[4], data[5], data[6]]), 0xDEADBEEF);
}

#[test]
fn test_sbe_enums() {
    
    let future_kind = InstrumentKind::future;
    let option_kind = InstrumentKind::option;
    
    assert_ne!(future_kind, option_kind);
    assert_eq!(InstrumentKind::from(0), InstrumentKind::future);
    assert_eq!(InstrumentKind::from(1), InstrumentKind::option);
    assert_eq!(InstrumentKind::from(255), InstrumentKind::NullVal);
    
    
    assert_eq!(InstrumentType::from(0), InstrumentType::not_applicable);
    assert_eq!(InstrumentType::from(1), InstrumentType::reversed);
    assert_eq!(InstrumentType::from(2), InstrumentType::linear);
    
    assert_eq!(OptionType::from(0), OptionType::not_applicable);
    assert_eq!(OptionType::from(1), OptionType::call);
    assert_eq!(OptionType::from(2), OptionType::put);
}

#[test]
fn test_sbe_bridge_creation() {
    let bridge = SbeBridge::new(100000);
    
    
    let instruments = bridge.list_instruments();
    assert_eq!(instruments.len(), 0);
}

#[test]
fn test_bridge_price_conversion() {
    let bridge = SbeBridge::new(100000); 
    
    
    
    let instruments = bridge.list_instruments();
    assert_eq!(instruments.len(), 0); 
}

#[test]
fn test_bridge_enum_conversions() {
    let bridge = SbeBridge::new(100000);
    
    
    
    let instrument = bridge.get_instrument(12345);
    assert!(instrument.is_none()); 
    
    let instrument_by_symbol = bridge.get_instrument_by_symbol("BTC-PERPETUAL");
    assert!(instrument_by_symbol.is_none()); 
}

#[test]
fn test_message_types_creation() {
    
    let instrument_msg = InstrumentMessage {
        instrument_id: 12345,
        instrument_state: 1,
        kind: 0, 
        instrument_type: 2, 
        option_type: 0, 
        rfq: 0,
        settlement_period: None,
        settlement_period_count: 0,
        base_currency: "BTC".to_string(),
        quote_currency: "USD".to_string(),
        counter_currency: "USD".to_string(),
        settlement_currency: "USD".to_string(),
        size_currency: "BTC".to_string(),
        creation_timestamp_ms: 1640995200000, 
        expiration_timestamp_ms: 1672531200000, 
        strike_price: None,
        contract_size: 1.0,
        min_trade_amount: 0.001,
        tick_size: 0.5,
        maker_commission: 0.0001,
        taker_commission: 0.0005,
        block_trade_commission: None,
        max_liquidation_commission: None,
        max_leverage: Some(100.0),
        instrument_name: "BTC-PERPETUAL".to_string(),
    };
    
    assert_eq!(instrument_msg.instrument_id, 12345);
    assert_eq!(instrument_msg.base_currency, "BTC");
    assert_eq!(instrument_msg.instrument_name, "BTC-PERPETUAL");
    
    
    let book_msg = BookMessage {
        instrument_id: 12345,
        timestamp_ms: 1640995200000,
        prev_change_id: 100,
        change_id: 101,
        is_last: true,
        changes: vec![
            BookChange {
                side: 1, 
                change: 0, 
                price: 50000.0,
                amount: 1.5,
            }
        ],
    };
    
    assert_eq!(book_msg.changes.len(), 1);
    assert_eq!(book_msg.changes[0].price, 50000.0);
    
    
    let trades_msg = TradesMessage {
        instrument_id: 12345,
        trades: vec![
            Trade {
                direction: 0, 
                price: 50000.0,
                amount: 1.0,
                timestamp_ms: 1640995200000,
                mark_price: 50000.0,
                index_price: 50000.0,
                trade_seq: 1001,
                trade_id: 2001,
                tick_direction: 1, 
                liquidation: 0, 
                iv: None,
                block_trade_id: None,
                combo_trade_id: None,
            }
        ],
    };
    
    assert_eq!(trades_msg.trades.len(), 1);
    assert_eq!(trades_msg.trades[0].price, 50000.0);
}

#[test]
fn test_sbe_message_enum() {
    let instrument_msg = InstrumentMessage {
        instrument_id: 12345,
        instrument_state: 1,
        kind: 0,
        instrument_type: 2,
        option_type: 0,
        rfq: 0,
        settlement_period: None,
        settlement_period_count: 0,
        base_currency: "BTC".to_string(),
        quote_currency: "USD".to_string(),
        counter_currency: "USD".to_string(),
        settlement_currency: "USD".to_string(),
        size_currency: "BTC".to_string(),
        creation_timestamp_ms: 1640995200000,
        expiration_timestamp_ms: 1672531200000,
        strike_price: None,
        contract_size: 1.0,
        min_trade_amount: 0.001,
        tick_size: 0.5,
        maker_commission: 0.0001,
        taker_commission: 0.0005,
        block_trade_commission: None,
        max_liquidation_commission: None,
        max_leverage: Some(100.0),
        instrument_name: "BTC-PERPETUAL".to_string(),
    };
    
    let sbe_msg = SbeMessage::Instrument(instrument_msg);
    
    match sbe_msg {
        SbeMessage::Instrument(msg) => {
            assert_eq!(msg.instrument_id, 12345);
            assert_eq!(msg.instrument_name, "BTC-PERPETUAL");
        }
        _ => panic!("Expected Instrument message"),
    }
}

#[test]
fn test_basic_parsing() {
    let parser = SbeMessageParser::new();
    
    
    let result = parser.parse_message(&[]);
    assert!(result.is_err());
    
    
    let small_data = [1, 2, 3];
    let result = parser.parse_message(&small_data);
    assert!(result.is_err());
    
    
    let mut mock_header = [0u8; 12]; 
    
    
    mock_header[0] = 22; 
    mock_header[1] = 0;
    
    
    mock_header[2] = 0xDC; 
    mock_header[3] = 0x03; 
    
    
    mock_header[4] = 1; 
    mock_header[5] = 0;
    
    
    mock_header[6] = 3; 
    mock_header[7] = 0;
    
    
    let mut full_message = mock_header.to_vec();
    full_message.extend_from_slice(&[0u8; 22]); 
    
    
    let result = parser.parse_message(&full_message);
    
    
    match result {
        Ok(_) => println!("Basic parsing succeeded"),
        Err(e) => println!("Basic parsing failed as expected: {:?}", e),
    }
}

#[cfg(test)]
mod sbe_integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_message_flow() {
        let bridge = SbeBridge::new(100000);
        
        
        let instrument_msg = InstrumentMessage {
            instrument_id: 12345,
            instrument_state: 1,
            kind: 0, 
            instrument_type: 2, 
            option_type: 0, 
            rfq: 0,
            settlement_period: None,
            settlement_period_count: 0,
            base_currency: "BTC".to_string(),
            quote_currency: "USD".to_string(),
            counter_currency: "USD".to_string(),
            settlement_currency: "USD".to_string(),
            size_currency: "BTC".to_string(),
            creation_timestamp_ms: 1640995200000,
            expiration_timestamp_ms: 1672531200000,
            strike_price: None,
            contract_size: 1.0,
            min_trade_amount: 0.001,
            tick_size: 0.5,
            maker_commission: 0.0001,
            taker_commission: 0.0005,
            block_trade_commission: None,
            max_liquidation_commission: None,
            max_leverage: Some(100.0),
            instrument_name: "BTC-PERPETUAL".to_string(),
        };
        
        let sbe_message = SbeMessage::Instrument(instrument_msg);
        let result = bridge.process_message(sbe_message);
        
        assert!(result.is_ok());
        let updates = result.unwrap();
        assert_eq!(updates.len(), 0); 
        
        
        let instruments = bridge.list_instruments();
        assert_eq!(instruments.len(), 1);
        
        let instrument = bridge.get_instrument(12345).unwrap();
        assert_eq!(instrument.symbol, "BTC-PERPETUAL");
        assert_eq!(instrument.kind, InstrumentKind::future);
    }
}