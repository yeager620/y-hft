use std::fmt;
use thiserror::Error;
use tracing::{debug, error, warn};

use super::{ReadBuf};
use crate::sbe::message_header_codec::decoder::MessageHeaderDecoder;

#[derive(Error, Debug)]
pub enum SbeParseError {
    #[error("Invalid message length: {0}")]
    InvalidLength(usize),
    #[error("Unknown template ID: {0}")]
    UnknownTemplateId(u16),
    #[error("Schema version mismatch: expected {expected}, got {actual}")]
    SchemaVersionMismatch { expected: u16, actual: u16 },
    #[error("SBE decoding error: {0}")]
    DecodingError(String),
    #[error("Buffer underrun at position {0}")]
    BufferUnderrun(usize),
}

#[derive(Debug, Clone)]
pub enum SbeMessage {
    Instrument(InstrumentMessage),
    InstrumentV2(InstrumentV2Message),
    Book(BookMessage),
    Trades(TradesMessage),
    Ticker(TickerMessage),
    Snapshot(SnapshotMessage),
    SnapshotStart(SnapshotStartMessage),
    SnapshotEnd(SnapshotEndMessage),
    PriceIndex(PriceIndexMessage),
    Rfq(RfqMessage),
    ComboLegs(ComboLegsMessage),
}

#[derive(Debug, Clone)]
pub struct InstrumentMessage {
    pub instrument_id: u32,
    pub instrument_state: u8,
    pub kind: u8,
    pub instrument_type: u8,
    pub option_type: u8,
    pub rfq: u8,
    pub settlement_period: Option<u8>,
    pub settlement_period_count: u16,
    pub base_currency: String,
    pub quote_currency: String,
    pub counter_currency: String,
    pub settlement_currency: String,
    pub size_currency: String,
    pub creation_timestamp_ms: u64,
    pub expiration_timestamp_ms: u64,
    pub strike_price: Option<f64>,
    pub contract_size: f64,
    pub min_trade_amount: f64,
    pub tick_size: f64,
    pub maker_commission: f64,
    pub taker_commission: f64,
    pub block_trade_commission: Option<f64>,
    pub max_liquidation_commission: Option<f64>,
    pub max_leverage: Option<f64>,
    pub instrument_name: String,
}

#[derive(Debug, Clone)]
pub struct InstrumentV2Message {
    pub instrument_id: u32,
    pub instrument_state: u8,
    pub kind: u8,
    pub instrument_type: u8,
    pub option_type: u8,
    pub settlement_period: Option<u8>,
    pub settlement_period_count: u16,
    pub base_currency: String,
    pub quote_currency: String,
    pub counter_currency: String,
    pub settlement_currency: String,
    pub size_currency: String,
    pub creation_timestamp_ms: u64,
    pub expiration_timestamp_ms: u64,
    pub strike_price: Option<f64>,
    pub contract_size: f64,
    pub min_trade_amount: f64,
    pub tick_size: f64,
    pub maker_commission: f64,
    pub taker_commission: f64,
    pub block_trade_commission: Option<f64>,
    pub max_liquidation_commission: Option<f64>,
    pub max_leverage: Option<f64>,
    pub tick_steps: Vec<TickStep>,
    pub instrument_name: String,
}

#[derive(Debug, Clone)]
pub struct TickStep {
    pub above_price: f64,
    pub tick_size: f64,
}

#[derive(Debug, Clone)]
pub struct BookMessage {
    pub instrument_id: u32,
    pub timestamp_ms: u64,
    pub prev_change_id: u64,
    pub change_id: u64,
    pub is_last: bool,
    pub changes: Vec<BookChange>,
}

#[derive(Debug, Clone)]
pub struct BookChange {
    pub side: u8, 
    pub change: u8, 
    pub price: f64,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct TradesMessage {
    pub instrument_id: u32,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub direction: u8, 
    pub price: f64,
    pub amount: f64,
    pub timestamp_ms: u64,
    pub mark_price: f64,
    pub index_price: f64,
    pub trade_seq: u64,
    pub trade_id: u64,
    pub tick_direction: u8,
    pub liquidation: u8,
    pub iv: Option<f64>,
    pub block_trade_id: Option<u64>,
    pub combo_trade_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TickerMessage {
    pub instrument_id: u32,
    pub instrument_state: u8,
    pub timestamp_ms: u64,
    pub open_interest: Option<f64>,
    pub min_sell_price: f64,
    pub max_buy_price: f64,
    pub last_price: Option<f64>,
    pub index_price: f64,
    pub mark_price: f64,
    pub best_bid_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_price: f64,
    pub best_ask_amount: f64,
    pub current_funding: Option<f64>,
    pub funding_8h: Option<f64>,
    pub estimated_delivery_price: Option<f64>,
    pub delivery_price: Option<f64>,
    pub settlement_price: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct SnapshotMessage {
    pub instrument_id: u32,
    pub timestamp_ms: u64,
    pub change_id: u64,
    pub is_book_complete: bool,
    pub is_last_in_book: bool,
    pub levels: Vec<SnapshotLevel>,
}

#[derive(Debug, Clone)]
pub struct SnapshotLevel {
    pub side: u8,
    pub price: f64,
    pub amount: f64,
}


#[derive(Debug, Clone)]
pub struct SnapshotStartMessage {
    pub snapshot_delay: u32,
}

#[derive(Debug, Clone)]
pub struct SnapshotEndMessage;

#[derive(Debug, Clone)]
pub struct PriceIndexMessage {
    pub index_name: String,
    pub price: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct RfqMessage {
    pub instrument_id: u32,
    pub state: u8,
    pub side: u8,
    pub amount: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ComboLegsMessage {
    pub instrument_id: u32,
    pub legs: Vec<ComboLeg>,
}

#[derive(Debug, Clone)]
pub struct ComboLeg {
    pub instrument_id: u32,
    pub ratio: f64,
    pub direction: u8,
}



#[derive(Debug, Clone)]
pub struct SbeMessageParser;

impl SbeMessageParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_message(&self, data: &[u8]) -> Result<SbeMessage, SbeParseError> {
        if data.len() < 12 {
            return Err(SbeParseError::InvalidLength(data.len()));
        }

        let buf = ReadBuf::new(data);
        
        let template_id = buf.get_u16_at(4);
        let schema_version = buf.get_u16_at(6);
        let block_length = buf.get_u16_at(0);
        if schema_version != 3 {
            warn!("Schema version mismatch: expected 3, got {}", schema_version);
        }

        let message_start = 12;

        debug!("Parsing message with template_id: {}, block_length: {}", template_id, block_length);

        match template_id {
            1000 => self.parse_instrument_basic(data, message_start),
            1001 => self.parse_book_basic(data, message_start),
            1002 => self.parse_trades_basic(data, message_start),
            1003 => self.parse_ticker_basic(data, message_start),
            1004 => self.parse_snapshot_basic(data, message_start),
            1005 => self.parse_snapshot_start_basic(data, message_start),
            1006 => self.parse_snapshot_end_basic(),
            1007 => self.parse_combo_legs_basic(data, message_start),
            1008 => self.parse_price_index_basic(data, message_start),
            1009 => self.parse_rfq_basic(data, message_start),
            1010 => self.parse_instrument_v2_basic(data, message_start),
            _ => {
                error!("Unknown template ID: {}", template_id);
                Err(SbeParseError::UnknownTemplateId(template_id))
            }
        }
    }


    fn parse_instrument_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 120 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        
        let instrument_id = buf.get_u32_at(0);
        let instrument_state = buf.get_u8_at(4);
        let kind = buf.get_u8_at(5);
        let instrument_type = buf.get_u8_at(6);
        let option_type = buf.get_u8_at(7);

        let message = InstrumentMessage {
            instrument_id,
            instrument_state,
            kind,
            instrument_type,
            option_type,
            rfq: buf.get_u8_at(8),
            settlement_period: None,
            settlement_period_count: buf.get_u16_at(10),
            base_currency: "BTC".to_string(),
            quote_currency: "USD".to_string(),
            counter_currency: "USD".to_string(),
            settlement_currency: "USD".to_string(),
            size_currency: "BTC".to_string(),
            creation_timestamp_ms: buf.get_u64_at(50),
            expiration_timestamp_ms: buf.get_u64_at(58),
            strike_price: None,
            contract_size: 1.0,
            min_trade_amount: 0.001,
            tick_size: 0.5,
            maker_commission: 0.0002,
            taker_commission: 0.0005,
            block_trade_commission: None,
            max_liquidation_commission: None,
            max_leverage: None,
            instrument_name: format!("INSTRUMENT_{}", instrument_id),
        };

        Ok(SbeMessage::Instrument(message))
    }

    fn parse_book_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 29 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        
        let instrument_id = buf.get_u32_at(0);
        let timestamp_ms = buf.get_u64_at(4);
        let prev_change_id = buf.get_u64_at(12);
        let change_id = buf.get_u64_at(20);
        let is_last = buf.get_u8_at(28) != 0;

        let changes = Vec::new();

        let message = BookMessage {
            instrument_id,
            timestamp_ms,
            prev_change_id,
            change_id,
            is_last,
            changes,
        };

        Ok(SbeMessage::Book(message))
    }

    fn parse_trades_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 4 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        let instrument_id = buf.get_u32_at(0);

        let trades = Vec::new();

        let message = TradesMessage {
            instrument_id,
            trades,
        };

        Ok(SbeMessage::Trades(message))
    }

    fn parse_ticker_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 120 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        
        let instrument_id = buf.get_u32_at(0);
        let instrument_state = buf.get_u8_at(4);
        let timestamp_ms = buf.get_u64_at(5);

        let message = TickerMessage {
            instrument_id,
            instrument_state,
            timestamp_ms,
            open_interest: None,
            min_sell_price: 0.0,
            max_buy_price: 100000.0,
            last_price: Some(50000.0),
            index_price: 50000.0,
            mark_price: 50000.0,
            best_bid_price: 49995.0,
            best_bid_amount: 1.0,
            best_ask_price: 50005.0,
            best_ask_amount: 1.0,
            current_funding: None,
            funding_8h: None,
            estimated_delivery_price: None,
            delivery_price: None,
            settlement_price: None,
        };

        Ok(SbeMessage::Ticker(message))
    }

    fn parse_snapshot_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 20 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        
        let instrument_id = buf.get_u32_at(0);
        let timestamp_ms = buf.get_u64_at(4);
        let change_id = buf.get_u64_at(12);

        let message = SnapshotMessage {
            instrument_id,
            timestamp_ms,
            change_id,
            is_book_complete: true,
            is_last_in_book: true,
            levels: Vec::new(),
        };

        Ok(SbeMessage::Snapshot(message))
    }

    fn parse_snapshot_start_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 4 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        let snapshot_delay = buf.get_u32_at(0);

        let message = SnapshotStartMessage {
            snapshot_delay,
        };

        Ok(SbeMessage::SnapshotStart(message))
    }

    fn parse_snapshot_end_basic(&self) -> Result<SbeMessage, SbeParseError> {
        Ok(SbeMessage::SnapshotEnd(SnapshotEndMessage))
    }

    fn parse_combo_legs_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 4 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        let instrument_id = buf.get_u32_at(0);

        let message = ComboLegsMessage {
            instrument_id,
            legs: Vec::new(),
        };

        Ok(SbeMessage::ComboLegs(message))
    }

    fn parse_price_index_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 32 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        let price = buf.get_f64_at(16);
        let timestamp_ms = buf.get_u64_at(24);

        let message = PriceIndexMessage {
            index_name: "BTC_USD".to_string(),
            price,
            timestamp_ms,
        };

        Ok(SbeMessage::PriceIndex(message))
    }

    fn parse_rfq_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        if data.len() < offset + 24 {
            return Err(SbeParseError::BufferUnderrun(offset));
        }

        let buf = ReadBuf::new(&data[offset..]);
        
        let instrument_id = buf.get_u32_at(0);
        let state = buf.get_u8_at(4);
        let side = buf.get_u8_at(5);
        let amount = buf.get_f64_at(8);
        let timestamp_ms = buf.get_u64_at(16);

        let message = RfqMessage {
            instrument_id,
            state,
            side,
            amount,
            timestamp_ms,
        };

        Ok(SbeMessage::Rfq(message))
    }

    fn parse_instrument_v2_basic(&self, data: &[u8], offset: usize) -> Result<SbeMessage, SbeParseError> {
        let instrument_basic = self.parse_instrument_basic(data, offset)?;
        
        if let SbeMessage::Instrument(basic_msg) = instrument_basic {
            let message = InstrumentV2Message {
                instrument_id: basic_msg.instrument_id,
                instrument_state: basic_msg.instrument_state,
                kind: basic_msg.kind,
                instrument_type: basic_msg.instrument_type,
                option_type: basic_msg.option_type,
                settlement_period: basic_msg.settlement_period,
                settlement_period_count: basic_msg.settlement_period_count,
                base_currency: basic_msg.base_currency,
                quote_currency: basic_msg.quote_currency,
                counter_currency: basic_msg.counter_currency,
                settlement_currency: basic_msg.settlement_currency,
                size_currency: basic_msg.size_currency,
                creation_timestamp_ms: basic_msg.creation_timestamp_ms,
                expiration_timestamp_ms: basic_msg.expiration_timestamp_ms,
                strike_price: basic_msg.strike_price,
                contract_size: basic_msg.contract_size,
                min_trade_amount: basic_msg.min_trade_amount,
                tick_size: basic_msg.tick_size,
                maker_commission: basic_msg.maker_commission,
                taker_commission: basic_msg.taker_commission,
                block_trade_commission: basic_msg.block_trade_commission,
                max_liquidation_commission: basic_msg.max_liquidation_commission,
                max_leverage: basic_msg.max_leverage,
                tick_steps: Vec::new(),
                instrument_name: basic_msg.instrument_name,
            };
            
            Ok(SbeMessage::InstrumentV2(message))
        } else {
            Err(SbeParseError::DecodingError("Failed to parse basic instrument".to_string()))
        }
    }
}

impl Default for SbeMessageParser {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SbeMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SbeMessage::Book(msg) => write!(f, "Book(id={}, changes={})", msg.instrument_id, msg.changes.len()),
            SbeMessage::Trades(msg) => write!(f, "Trades(id={}, trades={})", msg.instrument_id, msg.trades.len()),
            SbeMessage::Ticker(msg) => write!(f, "Ticker(id={}, last={:?})", msg.instrument_id, msg.last_price),
            SbeMessage::Snapshot(msg) => write!(f, "Snapshot(id={}, levels={})", msg.instrument_id, msg.levels.len()),
            SbeMessage::Instrument(msg) => write!(f, "Instrument(id={}, name={})", msg.instrument_id, msg.instrument_name),
            SbeMessage::InstrumentV2(msg) => write!(f, "InstrumentV2(id={}, name={})", msg.instrument_id, msg.instrument_name),
            SbeMessage::PriceIndex(msg) => write!(f, "PriceIndex(name={}, price={})", msg.index_name, msg.price),
            SbeMessage::Rfq(msg) => write!(f, "RFQ(id={}, amount={})", msg.instrument_id, msg.amount),
            SbeMessage::ComboLegs(msg) => write!(f, "ComboLegs(id={}, legs={})", msg.instrument_id, msg.legs.len()),
            SbeMessage::SnapshotStart(msg) => write!(f, "SnapshotStart(delay={})", msg.snapshot_delay),
            SbeMessage::SnapshotEnd(_) => write!(f, "SnapshotEnd"),
        }
    }
}