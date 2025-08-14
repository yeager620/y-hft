pub mod header;
pub mod trailer;
pub mod new_order_single;
pub mod execution_report;
pub mod order_cancel_request;
pub mod heartbeat;
pub mod logon;

pub use header::{Header, StandardHeader};
pub use trailer::Trailer;
pub use new_order_single::NewOrderSingle;
pub use execution_report::ExecutionReport;
pub use order_cancel_request::OrderCancelRequest;
pub use heartbeat::Heartbeat;
pub use logon::Logon;

use crate::fix::parser::FixField;
use crate::fix::error::FixError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FixMessage {
    NewOrderSingle(NewOrderSingle),
    ExecutionReport(ExecutionReport),
    OrderCancelRequest(OrderCancelRequest),
    Heartbeat(Heartbeat),
    Logon(Logon),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Heartbeat,              
    TestRequest,           
    ResendRequest,         
    Reject,                
    SequenceReset,         
    Logout,                
    IOI,                   
    Advertisement,         
    ExecutionReport,       
    OrderCancelReject,     
    Logon,                 
    News,                  
    Email,                 
    NewOrderSingle,        
    OrderCancelRequest,    
    OrderCancelReplaceRequest, 
    OrderStatusRequest,    
    Allocation,            
    ListCancelRequest,     
    ListExecute,           
    ListStatusRequest,     
    ListStatus,            
    AllocationAck,         
    DontKnowTrade,         
    QuoteRequest,          
    Quote,                 
    SettlementInstructions, 
    MarketDataRequest,     
    MarketDataSnapshotFullRefresh, 
    MarketDataIncrementalRefresh, 
    MarketDataRequestReject, 
    QuoteCancel,           
    QuoteStatusRequest,    
    QuoteAcknowledgement,  
    SecurityDefinitionRequest, 
    SecurityDefinition,    
    SecurityStatusRequest, 
    SecurityStatus,        
    TradingSessionStatusRequest, 
    TradingSessionStatus,  
    MassQuote,             
    BusinessMessageReject, 
    BidRequest,            
    BidResponse,           
    ListStrikePrice,       
}

impl MessageType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(MessageType::Heartbeat),
            "1" => Some(MessageType::TestRequest),
            "2" => Some(MessageType::ResendRequest),
            "3" => Some(MessageType::Reject),
            "4" => Some(MessageType::SequenceReset),
            "5" => Some(MessageType::Logout),
            "6" => Some(MessageType::IOI),
            "7" => Some(MessageType::Advertisement),
            "8" => Some(MessageType::ExecutionReport),
            "9" => Some(MessageType::OrderCancelReject),
            "A" => Some(MessageType::Logon),
            "B" => Some(MessageType::News),
            "C" => Some(MessageType::Email),
            "D" => Some(MessageType::NewOrderSingle),
            "F" => Some(MessageType::OrderCancelRequest),
            "G" => Some(MessageType::OrderCancelReplaceRequest),
            "H" => Some(MessageType::OrderStatusRequest),
            "J" => Some(MessageType::Allocation),
            "K" => Some(MessageType::ListCancelRequest),
            "L" => Some(MessageType::ListExecute),
            "M" => Some(MessageType::ListStatusRequest),
            "N" => Some(MessageType::ListStatus),
            "P" => Some(MessageType::AllocationAck),
            "Q" => Some(MessageType::DontKnowTrade),
            "R" => Some(MessageType::QuoteRequest),
            "S" => Some(MessageType::Quote),
            "T" => Some(MessageType::SettlementInstructions),
            "V" => Some(MessageType::MarketDataRequest),
            "W" => Some(MessageType::MarketDataSnapshotFullRefresh),
            "X" => Some(MessageType::MarketDataIncrementalRefresh),
            "Y" => Some(MessageType::MarketDataRequestReject),
            "Z" => Some(MessageType::QuoteCancel),
            "a" => Some(MessageType::QuoteStatusRequest),
            "b" => Some(MessageType::QuoteAcknowledgement),
            "c" => Some(MessageType::SecurityDefinitionRequest),
            "d" => Some(MessageType::SecurityDefinition),
            "e" => Some(MessageType::SecurityStatusRequest),
            "f" => Some(MessageType::SecurityStatus),
            "g" => Some(MessageType::TradingSessionStatusRequest),
            "h" => Some(MessageType::TradingSessionStatus),
            "i" => Some(MessageType::MassQuote),
            "j" => Some(MessageType::BusinessMessageReject),
            "k" => Some(MessageType::BidRequest),
            "l" => Some(MessageType::BidResponse),
            "m" => Some(MessageType::ListStrikePrice),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Heartbeat => "0",
            MessageType::TestRequest => "1",
            MessageType::ResendRequest => "2",
            MessageType::Reject => "3",
            MessageType::SequenceReset => "4",
            MessageType::Logout => "5",
            MessageType::IOI => "6",
            MessageType::Advertisement => "7",
            MessageType::ExecutionReport => "8",
            MessageType::OrderCancelReject => "9",
            MessageType::Logon => "A",
            MessageType::News => "B",
            MessageType::Email => "C",
            MessageType::NewOrderSingle => "D",
            MessageType::OrderCancelRequest => "F",
            MessageType::OrderCancelReplaceRequest => "G",
            MessageType::OrderStatusRequest => "H",
            MessageType::Allocation => "J",
            MessageType::ListCancelRequest => "K",
            MessageType::ListExecute => "L",
            MessageType::ListStatusRequest => "M",
            MessageType::ListStatus => "N",
            MessageType::AllocationAck => "P",
            MessageType::DontKnowTrade => "Q",
            MessageType::QuoteRequest => "R",
            MessageType::Quote => "S",
            MessageType::SettlementInstructions => "T",
            MessageType::MarketDataRequest => "V",
            MessageType::MarketDataSnapshotFullRefresh => "W",
            MessageType::MarketDataIncrementalRefresh => "X",
            MessageType::MarketDataRequestReject => "Y",
            MessageType::QuoteCancel => "Z",
            MessageType::QuoteStatusRequest => "a",
            MessageType::QuoteAcknowledgement => "b",
            MessageType::SecurityDefinitionRequest => "c",
            MessageType::SecurityDefinition => "d",
            MessageType::SecurityStatusRequest => "e",
            MessageType::SecurityStatus => "f",
            MessageType::TradingSessionStatusRequest => "g",
            MessageType::TradingSessionStatus => "h",
            MessageType::MassQuote => "i",
            MessageType::BusinessMessageReject => "j",
            MessageType::BidRequest => "k",
            MessageType::BidResponse => "l",
            MessageType::ListStrikePrice => "m",
        }
    }
}