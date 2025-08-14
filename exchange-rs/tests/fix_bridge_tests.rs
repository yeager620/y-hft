use exchange_rs::fix::bridge::FixOrderConverter;
use exchange_rs::fix::messages::{NewOrderSingle, StandardHeader, Trailer, MessageType};
use exchange_rs::order::{OrderType, Side, TimeInForce};

#[test]
fn test_convert_limit_buy_order() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT123".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 1,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 123 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER123".to_string(),
        account: None,
        handl_inst: '1',
        symbol: "AAPL".to_string(),
        side: '1', // Buy
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 100,
        ord_type: '2', // Limit
        price: Some(150.50),
        stop_px: None,
        time_in_force: Some('1'), // GTC
        exec_inst: None,
        trailer,
    };

    let order = converter.convert_new_order_single(fix_order).unwrap();
    
    assert_eq!(order.symbol, "AAPL");
    assert_eq!(order.side, Side::Buy);
    assert_eq!(order.order_type, OrderType::Limit);
    assert_eq!(order.quantity, 100);
    assert_eq!(order.price, 1505000); // 150.50 * 10000
    assert_eq!(order.time_in_force, TimeInForce::GTC);
    assert_eq!(order.stop_price, None);
}

#[test]
fn test_convert_market_sell_order() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT456".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 2,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 124 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER456".to_string(),
        account: Some("ACCOUNT456".to_string()),
        handl_inst: '1',
        symbol: "GOOGL".to_string(),
        side: '2', // Sell
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 50,
        ord_type: '1', // Market
        price: None,
        stop_px: None,
        time_in_force: Some('3'), // IOC
        exec_inst: None,
        trailer,
    };

    let order = converter.convert_new_order_single(fix_order).unwrap();
    
    assert_eq!(order.symbol, "GOOGL");
    assert_eq!(order.side, Side::Sell);
    assert_eq!(order.order_type, OrderType::Market);
    assert_eq!(order.quantity, 50);
    assert_eq!(order.price, 0); // Market orders have no price
    assert_eq!(order.time_in_force, TimeInForce::IOC);
    assert_eq!(order.stop_price, None);
    assert_eq!(order.user_id, 456); // Extracted from account
}

#[test]
fn test_convert_stop_limit_order() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT789".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 3,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 125 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER789".to_string(),
        account: None,
        handl_inst: '1',
        symbol: "TSLA".to_string(),
        side: '2', // Sell
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 25,
        ord_type: '4', // Stop Limit
        price: Some(200.00),
        stop_px: Some(195.00),
        time_in_force: Some('4'), // FOK
        exec_inst: None,
        trailer,
    };

    let order = converter.convert_new_order_single(fix_order).unwrap();
    
    assert_eq!(order.symbol, "TSLA");
    assert_eq!(order.side, Side::Sell);
    assert_eq!(order.order_type, OrderType::StopLimit);
    assert_eq!(order.quantity, 25);
    assert_eq!(order.price, 2000000); // 200.00 * 10000
    assert_eq!(order.stop_price, Some(1950000)); // 195.00 * 10000
    assert_eq!(order.time_in_force, TimeInForce::FOK);
    assert_eq!(order.user_id, 789); // Extracted from sender_comp_id
}

#[test]
fn test_convert_stop_market_order() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT101".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 4,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 126 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER101".to_string(),
        account: None,
        handl_inst: '1',
        symbol: "NVDA".to_string(),
        side: '1', // Buy
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 75,
        ord_type: '3', // Stop Market
        price: None,
        stop_px: Some(500.00),
        time_in_force: Some('0'), // Day
        exec_inst: None,
        trailer,
    };

    let order = converter.convert_new_order_single(fix_order).unwrap();
    
    assert_eq!(order.symbol, "NVDA");
    assert_eq!(order.side, Side::Buy);
    assert_eq!(order.order_type, OrderType::StopMarket);
    assert_eq!(order.quantity, 75);
    assert_eq!(order.price, 0); // Stop market orders have no limit price
    assert_eq!(order.stop_price, Some(5000000)); // 500.00 * 10000
    assert_eq!(order.time_in_force, TimeInForce::Day);
    assert_eq!(order.user_id, 101); // Extracted from sender_comp_id
}

#[test]
fn test_invalid_order_type() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT999".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 5,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 127 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER999".to_string(),
        account: None,
        handl_inst: '1',
        symbol: "INVALID".to_string(),
        side: '1',
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 100,
        ord_type: 'X', // Invalid order type
        price: Some(100.00),
        stop_px: None,
        time_in_force: Some('1'),
        exec_inst: None,
        trailer,
    };

    let result = converter.convert_new_order_single(fix_order);
    assert!(result.is_err());
}

#[test]
fn test_missing_price_for_limit_order() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT888".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 6,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 128 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER888".to_string(),
        account: None,
        handl_inst: '1',
        symbol: "MSFT".to_string(),
        side: '1',
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 100,
        ord_type: '2', // Limit order
        price: None,   // Missing price
        stop_px: None,
        time_in_force: Some('1'),
        exec_inst: None,
        trailer,
    };

    let result = converter.convert_new_order_single(fix_order);
    assert!(result.is_err());
}

#[test]
fn test_missing_stop_price_for_stop_order() {
    let converter = FixOrderConverter::new();
    
    let header = StandardHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 100,
        msg_type: MessageType::NewOrderSingle,
        sender_comp_id: "CLIENT777".to_string(),
        target_comp_id: "EXCHANGE".to_string(),
        msg_seq_num: 7,
        sending_time: "20240101-12:00:00".to_string(),
        poss_dup_flag: None,
        poss_resend: None,
        secure_data_len: None,
        secure_data: None,
    };

    let trailer = Trailer { checksum: 129 };

    let fix_order = NewOrderSingle {
        header,
        cl_ord_id: "ORDER777".to_string(),
        account: None,
        handl_inst: '1',
        symbol: "AMZN".to_string(),
        side: '2',
        transact_time: "20240101-12:00:00".to_string(),
        order_qty: 10,
        ord_type: '3', // Stop market order
        price: None,
        stop_px: None, // Missing stop price
        time_in_force: Some('1'),
        exec_inst: None,
        trailer,
    };

    let result = converter.convert_new_order_single(fix_order);
    assert!(result.is_err());
}