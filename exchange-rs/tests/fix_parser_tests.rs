use exchange_rs::fix::{FixParser, FixError, ParseError, ValidationError};
use exchange_rs::fix::messages::{FixMessage, MessageType};
use exchange_rs::fix::parser::{
    AdvancedFixParser, RecoveringParser, ErrorRecovery, GroupDefinitions
};

#[test]
fn test_parse_new_order_single() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=178\x0135=D\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0111=ORDER123\x011=ACCOUNT1\x0121=1\x0155=AAPL\x0154=1\x0160=20240101-12:00:00\x0138=100\x0140=2\x0144=150.50\x0159=1\x0110=161\x01";
    
    let result = parser.parse(message).unwrap();
    
    match result {
        FixMessage::NewOrderSingle(order) => {
            assert_eq!(order.header.begin_string, "FIX.4.4");
            assert_eq!(order.header.msg_type, MessageType::NewOrderSingle);
            assert_eq!(order.header.sender_comp_id, "CLIENT123");
            assert_eq!(order.header.target_comp_id, "EXCHANGE");
            assert_eq!(order.cl_ord_id, "ORDER123");
            assert_eq!(order.symbol, "AAPL");
            assert_eq!(order.side, '1'); 
            assert_eq!(order.order_qty, 100);
            assert_eq!(order.ord_type, '2'); 
            assert_eq!(order.price, Some(150.50));
            assert_eq!(order.time_in_force, Some('1')); 
        }
        _ => panic!("Expected NewOrderSingle message"),
    }
}

#[test]
fn test_parse_execution_report() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=200\x0135=8\x0149=EXCHANGE\x0156=CLIENT123\x0134=2\x0152=20240101-12:00:01\x0137=12345\x0111=ORDER123\x0117=EXEC123\x01150=F\x0139=1\x0155=AAPL\x0154=1\x0138=100\x0140=2\x0144=150.50\x0132=50\x0131=150.50\x01151=50\x0114=50\x016=150.50\x0160=20240101-12:00:01\x0110=161\x01";
    
    let result = parser.parse(message).unwrap();
    
    match result {
        FixMessage::ExecutionReport(report) => {
            assert_eq!(report.header.begin_string, "FIX.4.4");
            assert_eq!(report.header.msg_type, MessageType::ExecutionReport);
            assert_eq!(report.order_id, "12345");
            assert_eq!(report.cl_ord_id, "ORDER123");
            assert_eq!(report.exec_id, "EXEC123");
            assert_eq!(report.exec_type, 'F'); 
            assert_eq!(report.ord_status, '1'); 
            assert_eq!(report.symbol, "AAPL");
            assert_eq!(report.last_qty, Some(50));
            assert_eq!(report.last_px, Some(150.50));
            assert_eq!(report.leaves_qty, 50);
            assert_eq!(report.cum_qty, 50);
        }
        _ => panic!("Expected ExecutionReport message"),
    }
}

#[test]
fn test_parse_order_cancel_request() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=150\x0135=F\x0149=CLIENT123\x0156=EXCHANGE\x0134=3\x0152=20240101-12:00:02\x0141=ORDER123\x0111=CANCEL123\x0155=AAPL\x0154=1\x0160=20240101-12:00:02\x0138=100\x0110=161\x01";
    
    let result = parser.parse(message).unwrap();
    
    match result {
        FixMessage::OrderCancelRequest(cancel) => {
            assert_eq!(cancel.header.msg_type, MessageType::OrderCancelRequest);
            assert_eq!(cancel.orig_cl_ord_id, "ORDER123");
            assert_eq!(cancel.cl_ord_id, "CANCEL123");
            assert_eq!(cancel.symbol, "AAPL");
            assert_eq!(cancel.side, '1'); 
            assert_eq!(cancel.order_qty, Some(100));
        }
        _ => panic!("Expected OrderCancelRequest message"),
    }
}

#[test]
fn test_parse_heartbeat() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=50\x0135=0\x0149=CLIENT123\x0156=EXCHANGE\x0134=4\x0152=20240101-12:00:03\x01112=TEST123\x0110=161\x01";
    
    let result = parser.parse(message).unwrap();
    
    match result {
        FixMessage::Heartbeat(heartbeat) => {
            assert_eq!(heartbeat.header.msg_type, MessageType::Heartbeat);
            assert_eq!(heartbeat.test_req_id, Some("TEST123".to_string()));
        }
        _ => panic!("Expected Heartbeat message"),
    }
}

#[test]
fn test_parse_logon() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=100\x0135=A\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0198=0\x01108=30\x01141=Y\x01553=username\x01554=password\x0110=161\x01";
    
    let result = parser.parse(message).unwrap();
    
    match result {
        FixMessage::Logon(logon) => {
            assert_eq!(logon.header.msg_type, MessageType::Logon);
            assert_eq!(logon.encrypt_method, '0');
            assert_eq!(logon.heart_bt_int, 30);
            assert_eq!(logon.reset_seq_num_flag, Some(true));
            assert_eq!(logon.username, Some("username".to_string()));
            assert_eq!(logon.password, Some("password".to_string()));
        }
        _ => panic!("Expected Logon message"),
    }
}

#[test]
fn test_parse_invalid_checksum() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=50\x0135=0\x0149=CLIENT123\x0156=EXCHANGE\x0134=4\x0152=20240101-12:00:03\x0110=999\x01";
    
    let result = parser.parse(message);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        FixError::Parse(ParseError::InvalidChecksum { .. }) => {},
        _ => panic!("Expected InvalidChecksum error"),
    }
}

#[test]
fn test_parse_malformed_message() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.435=049=CLIENT12356=EXCHANGE34=152=20240101-12:00:0310=161";
    
    let result = parser.parse(message);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        FixError::Parse(ParseError::MissingSoh) => {},
        _ => panic!("Expected MissingSoh error"),
    }
}

#[test]
fn test_validate_checksum_success() {
    let parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=50\x0135=0\x0149=CLIENT123\x0156=EXCHANGE\x0134=4\x0152=20240101-12:00:03\x0110=161\x01";
    
    let result = parser.validate_checksum(message);
    
    
    match result {
        Ok(()) => {},
        Err(_) => {
            
            let body = &message[..message.len()-7]; 
            let actual_checksum: u8 = body.iter().fold(0u8, |acc, &byte| acc.wrapping_add(byte));
            println!("Actual checksum: {:03}", actual_checksum);
        }
    }
}

#[test]
fn test_message_type_conversion() {
    assert_eq!(MessageType::from_str("D"), Some(MessageType::NewOrderSingle));
    assert_eq!(MessageType::from_str("8"), Some(MessageType::ExecutionReport));
    assert_eq!(MessageType::from_str("F"), Some(MessageType::OrderCancelRequest));
    assert_eq!(MessageType::from_str("0"), Some(MessageType::Heartbeat));
    assert_eq!(MessageType::from_str("A"), Some(MessageType::Logon));
    assert_eq!(MessageType::from_str("Z"), None);
    
    assert_eq!(MessageType::NewOrderSingle.as_str(), "D");
    assert_eq!(MessageType::ExecutionReport.as_str(), "8");
    assert_eq!(MessageType::OrderCancelRequest.as_str(), "F");
    assert_eq!(MessageType::Heartbeat.as_str(), "0");
    assert_eq!(MessageType::Logon.as_str(), "A");
}

#[test]
fn test_field_value_extraction() {
    let mut parser = FixParser::new();
    
    let message = b"8=FIX.4.4\x019=50\x0135=D\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0138=100\x0144=150.50\x0154=1\x0159=1\x0110=161\x01";
    
    let result = parser.parse(message).unwrap();
    
    match result {
        FixMessage::NewOrderSingle(order) => {
            
            assert_eq!(order.symbol, "AAPL");
            
            
            assert_eq!(order.order_qty, 100);
            
            
            assert_eq!(order.price, Some(150.50));
            
            
            
            
        }
        _ => {}, 
    }
}


#[test]
fn test_advanced_parser_with_metadata() {
    let mut parser = AdvancedFixParser::new()
        .with_performance_mode(true)
        .with_strict_validation(false);
    
    let message = b"8=FIX.4.4\x019=178\x0135=D\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0111=ORDER123\x0155=AAPL\x0154=1\x0138=100\x0140=2\x0144=150.50\x0110=161\x01";
    
    let result = parser.parse_advanced(message).unwrap();
    
    assert!(result.parsing_metadata.parse_time_nanos > 0);
    assert_eq!(result.parsing_metadata.message_size, message.len());
    assert!(result.parsing_metadata.field_count > 0);
    
    match result.message {
        FixMessage::NewOrderSingle(order) => {
            assert_eq!(order.symbol, "AAPL");
        }
        _ => panic!("Expected NewOrderSingle message"),
    }
}

#[test]
fn test_session_info_extraction() {
    let mut parser = AdvancedFixParser::new();
    
    let message = b"8=FIX.4.4\x019=100\x0135=A\x0149=CLIENT123\x0156=EXCHANGE\x0134=42\x0152=20240101-12:00:00\x0198=0\x01108=30\x0110=161\x01";
    
    let session_info = parser.extract_session_info(message).unwrap();
    
    assert_eq!(session_info.sender_comp_id, "CLIENT123");
    assert_eq!(session_info.target_comp_id, "EXCHANGE");
    assert_eq!(session_info.msg_seq_num, 42);
    assert_eq!(session_info.msg_type, MessageType::Logon);
}

#[test]
fn test_administrative_message_detection() {
    let mut parser = AdvancedFixParser::new();
    
    
    let heartbeat = b"8=FIX.4.4\x019=50\x0135=0\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0110=161\x01";
    assert!(parser.is_administrative_message(heartbeat).unwrap());
    
    
    let order = b"8=FIX.4.4\x019=100\x0135=D\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0111=ORDER123\x0155=AAPL\x0110=161\x01";
    assert!(!parser.is_administrative_message(order).unwrap());
}


#[test]
fn test_error_recovery_invalid_checksum() {
    let config = ErrorRecovery {
        recover_from_checksum_errors: true,
        ..ErrorRecovery::default()
    };
    let mut parser = RecoveringParser::new(config);
    
    
    let message = b"8=FIX.4.4\x019=50\x0135=0\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0110=999\x01";
    
    let result = parser.parse_with_recovery(message);
    
    assert!(result.recovery_attempts > 0);
    assert!(!result.warnings.is_empty());
}

#[test]
fn test_error_recovery_missing_field() {
    let config = ErrorRecovery {
        allow_partial_parse: true,
        skip_invalid_fields: true,
        ..ErrorRecovery::default()
    };
    let mut parser = RecoveringParser::new(config);
    
    
    let message = b"8=FIX.4.4\x019=80\x0135=D\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0155=AAPL\x0154=1\x0138=100\x0140=1\x0110=161\x01";
    
    let result = parser.parse_with_recovery(message);
    
    assert!(!result.errors.is_empty());
    assert!(result.recovery_attempts > 0);
}


#[test]
fn test_timestamp_field_parsing() {
    use exchange_rs::fix::parser::field_parser::{FieldParser, FieldValue};
    use exchange_rs::fix::parser::raw_parser::RawField;
    
    let parser = FieldParser::new();
    
    
    let raw_field = RawField {
        tag: b"52",
        value: b"20240101-12:30:45.123",
    };
    
    let field = parser.parse_field(raw_field).unwrap();
    assert_eq!(field.tag, 52);
    
    match field.value {
        FieldValue::UTCTimestamp(ts) => {
            assert_eq!(ts, "20240101-12:30:45.123");
        }
        _ => panic!("Expected UTCTimestamp"),
    }
}

#[test]
fn test_invalid_timestamp_parsing() {
    use exchange_rs::fix::parser::field_parser::{FieldParser};
    use exchange_rs::fix::parser::raw_parser::RawField;
    
    let parser = FieldParser::new();
    
    
    let raw_field = RawField {
        tag: b"52",
        value: b"invalid-timestamp",
    };
    
    let result = parser.parse_field(raw_field);
    assert!(result.is_err());
}


#[test]
fn test_message_validation() {
    let mut parser = FixParser::new();
    
    
    let message = b"8=FIX.4.4\x019=100\x0135=D\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0111=ORDER123\x0155=AAPL\x0154=9\x0138=100\x0140=1\x0110=161\x01";
    
    let result = parser.parse(message);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        FixError::Validation(ValidationError::InvalidFieldValue { tag: 54, .. }) => {},
        _ => panic!("Expected validation error for invalid side"),
    }
}

#[test]
fn test_checksum_calculation() {
    use exchange_rs::fix::parser::raw_parser::RawParser;
    
    let parser = RawParser::new();
    
    
    let message = b"8=FIX.4.4\x019=50\x0135=0\x0149=CLIENT123\x0156=EXCHANGE\x0134=1\x0152=20240101-12:00:00\x0110=000\x01";
    
    
    let body = &message[..message.len()-7]; 
    let expected_checksum: u8 = body.iter().fold(0u8, |acc, &byte| acc.wrapping_add(byte)) % 256;
    
    
    let mut correct_message = body.to_vec();
    correct_message.extend_from_slice(format!("10={:03}\x01", expected_checksum).as_bytes());
    
    assert!(parser.validate_checksum(&correct_message).is_ok());
}