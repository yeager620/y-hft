use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use thiserror::Error;
use tracing::{debug, info, warn};
use chrono;

use crate::order::{Order, Side, OrderType, OrderStatus, TimeInForce};
use crate::matching_engine::{Trade, MatchingEngine};
use crate::orderbook::OrderBook;
use crate::sbe::{InstrumentKind, InstrumentType, OptionType};
use crate::PRICE_SCALE_FACTOR;
use crate::sbe::parser::{
    SbeMessage, BookMessage, BookChange, TradesMessage, Trade as SbeTrade,
    TickerMessage, SnapshotMessage, InstrumentMessage, SnapshotLevel
};

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Unknown instrument ID: {0}")]
    UnknownInstrument(u32),
    #[error("Invalid side value: {0}")]
    InvalidSide(u8),
    #[error("Invalid order type")]
    InvalidOrderType,
    #[error("Price conversion error: {0}")]
    PriceConversion(String),
    #[error("Matching engine error: {0}")]
    MatchingEngine(String),
}

#[derive(Debug, Clone)]
pub struct DeribitInstrument {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub kind: InstrumentKind,
    pub instrument_type: InstrumentType,
    pub option_type: OptionType,
    pub base_currency: String,
    pub quote_currency: String,
    pub tick_size: f64,
    pub contract_size: f64,
    pub min_trade_amount: f64,
    pub creation_timestamp: u64,
    pub expiration_timestamp: u64,
    pub strike_price: Option<f64>,
    pub is_active: bool,
}



#[derive(Debug, Clone)]
pub struct MarketDataUpdate {
    pub instrument_id: u32,
    pub symbol: String,
    pub timestamp: u64,
    pub best_bid: Option<(f64, f64)>,
    pub best_ask: Option<(f64, f64)>,
    pub last_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub index_price: Option<f64>,
}

pub struct SbeBridge {
    pub instruments: RwLock<HashMap<u32, DeribitInstrument>>,
    symbol_to_id: RwLock<HashMap<String, u32>>,
    external_user_id_counter: RwLock<u64>,
}

impl SbeBridge {
    pub fn new(_price_scale: u64) -> Self {
        
        Self {
            instruments: RwLock::new(HashMap::new()),
            symbol_to_id: RwLock::new(HashMap::new()),
            external_user_id_counter: RwLock::new(1000), 
        }
    }

    fn get_next_external_user_id(&self) -> u64 {
        let mut counter = self.external_user_id_counter.write();
        *counter += 1;
        *counter
    }

    pub fn process_message(&self, message: SbeMessage) -> Result<Vec<MarketDataUpdate>, BridgeError> {
        match message {
            SbeMessage::Instrument(msg) => {
                self.handle_instrument(msg)?;
                Ok(Vec::new())
            }
            SbeMessage::Book(msg) => {
                self.handle_book_update(msg)
            }
            SbeMessage::Trades(msg) => {
                self.handle_trades(msg)
            }
            SbeMessage::Ticker(msg) => {
                self.handle_ticker(msg)
            }
            SbeMessage::Snapshot(msg) => {
                self.handle_snapshot(msg)
            }
            _ => {
                debug!("Unhandled message type: {:?}", message);
                Ok(Vec::new())
            }
        }
    }

    fn handle_instrument(&self, msg: InstrumentMessage) -> Result<(), BridgeError> {
        let instrument = DeribitInstrument {
            id: msg.instrument_id,
            name: msg.instrument_name.clone(),
            symbol: msg.instrument_name.clone(),
            kind: self.convert_instrument_kind(msg.kind),
            instrument_type: self.convert_instrument_type(msg.instrument_type),
            option_type: self.convert_option_type(msg.option_type),
            base_currency: msg.base_currency,
            quote_currency: msg.quote_currency,
            tick_size: msg.tick_size,
            contract_size: msg.contract_size,
            min_trade_amount: msg.min_trade_amount,
            creation_timestamp: msg.creation_timestamp_ms,
            expiration_timestamp: msg.expiration_timestamp_ms,
            strike_price: msg.strike_price,
            is_active: msg.instrument_state != 2, 
        };

        info!("Registered instrument: {} (ID: {})", instrument.name, instrument.id);

        {
            let mut instruments = self.instruments.write();
            instruments.insert(msg.instrument_id, instrument.clone());
        }

        {
            let mut symbol_map = self.symbol_to_id.write();
            symbol_map.insert(instrument.name, msg.instrument_id);
        }

        Ok(())
    }


    fn handle_book_update(&self, msg: BookMessage) -> Result<Vec<MarketDataUpdate>, BridgeError> {
        let instrument = {
            let instruments = self.instruments.read();
            instruments.get(&msg.instrument_id)
                .ok_or(BridgeError::UnknownInstrument(msg.instrument_id))?
                .clone()
        };

        debug!("Processing book update for {}: {} changes", instrument.symbol, msg.changes.len());

        let mut best_bid: Option<(f64, f64)> = None;
        let mut best_ask: Option<(f64, f64)> = None;

        for change in &msg.changes {
            if change.change == 2 { 
                continue;
            }

            match change.side {
                1 => { 
                    if best_bid.is_none() || change.price > best_bid.unwrap().0 {
                        best_bid = Some((change.price, change.amount));
                    }
                }
                0 => { 
                    if best_ask.is_none() || change.price < best_ask.unwrap().0 {
                        best_ask = Some((change.price, change.amount));
                    }
                }
                _ => warn!("Unknown side: {}", change.side),
            }
        }

        let update = MarketDataUpdate {
            instrument_id: msg.instrument_id,
            symbol: instrument.symbol,
            timestamp: msg.timestamp_ms,
            best_bid,
            best_ask,
            last_price: None,
            mark_price: None,
            index_price: None,
        };

        Ok(vec![update])
    }

    fn handle_trades(&self, msg: TradesMessage) -> Result<Vec<MarketDataUpdate>, BridgeError> {
        let instrument = {
            let instruments = self.instruments.read();
            instruments.get(&msg.instrument_id)
                .ok_or(BridgeError::UnknownInstrument(msg.instrument_id))?
                .clone()
        };

        debug!("Processing {} trades for {}", msg.trades.len(), instrument.symbol);

        let mut updates = Vec::new();

        if let Some(last_trade) = msg.trades.last() {
            let update = MarketDataUpdate {
                instrument_id: msg.instrument_id,
                symbol: instrument.symbol.clone(),
                timestamp: last_trade.timestamp_ms,
                best_bid: None,
                best_ask: None,
                last_price: Some(last_trade.price),
                mark_price: Some(last_trade.mark_price),
                index_price: Some(last_trade.index_price),
            };
            updates.push(update);
        }

        Ok(updates)
    }

    fn handle_ticker(&self, msg: TickerMessage) -> Result<Vec<MarketDataUpdate>, BridgeError> {
        let instrument = {
            let instruments = self.instruments.read();
            instruments.get(&msg.instrument_id)
                .ok_or(BridgeError::UnknownInstrument(msg.instrument_id))?
                .clone()
        };

        debug!("Processing ticker for {}", instrument.symbol);

        let update = MarketDataUpdate {
            instrument_id: msg.instrument_id,
            symbol: instrument.symbol,
            timestamp: msg.timestamp_ms,
            best_bid: Some((msg.best_bid_price, msg.best_bid_amount)),
            best_ask: Some((msg.best_ask_price, msg.best_ask_amount)),
            last_price: msg.last_price,
            mark_price: Some(msg.mark_price),
            index_price: Some(msg.index_price),
        };

        Ok(vec![update])
    }

    fn handle_snapshot(&self, msg: SnapshotMessage) -> Result<Vec<MarketDataUpdate>, BridgeError> {
        let instrument = {
            let instruments = self.instruments.read();
            instruments.get(&msg.instrument_id)
                .ok_or(BridgeError::UnknownInstrument(msg.instrument_id))?
                .clone()
        };

        debug!("Processing snapshot for {}: {} levels", instrument.symbol, msg.levels.len());

        let mut best_bid: Option<(f64, f64)> = None;
        let mut best_ask: Option<(f64, f64)> = None;

        for level in &msg.levels {
            match level.side {
                1 => { 
                    if best_bid.is_none() || level.price > best_bid.unwrap().0 {
                        best_bid = Some((level.price, level.amount));
                    }
                }
                0 => { 
                    if best_ask.is_none() || level.price < best_ask.unwrap().0 {
                        best_ask = Some((level.price, level.amount));
                    }
                }
                _ => warn!("Unknown side in snapshot: {}", level.side),
            }
        }

        let update = MarketDataUpdate {
            instrument_id: msg.instrument_id,
            symbol: instrument.symbol,
            timestamp: msg.timestamp_ms,
            best_bid,
            best_ask,
            last_price: None,
            mark_price: None,
            index_price: None,
        };

        Ok(vec![update])
    }

    fn convert_instrument_kind(&self, kind: u8) -> InstrumentKind {
        match kind {
            0 => InstrumentKind::future,
            1 => InstrumentKind::option,
            2 => InstrumentKind::future_combo,
            3 => InstrumentKind::option_combo,
            4 => InstrumentKind::spot,
            _ => InstrumentKind::future, 
        }
    }

    fn convert_instrument_type(&self, instrument_type: u8) -> InstrumentType {
        match instrument_type {
            0 => InstrumentType::not_applicable,
            1 => InstrumentType::reversed,
            2 => InstrumentType::linear,
            _ => InstrumentType::not_applicable,
        }
    }

    fn convert_option_type(&self, option_type: u8) -> OptionType {
        match option_type {
            0 => OptionType::not_applicable,
            1 => OptionType::call,
            2 => OptionType::put,
            _ => OptionType::not_applicable,
        }
    }

    pub fn convert_sbe_trade_to_internal(&self, 
        sbe_trade: &SbeTrade, 
        _instrument_id: u32,
        trade_id: u64
    ) -> Result<Trade, BridgeError> {
        let price_scaled = crate::price_utils::float_to_scaled_price(sbe_trade.price)
            .map_err(|err| BridgeError::PriceConversion(err))?;
        let quantity = crate::price_utils::float_to_scaled_quantity(sbe_trade.amount)
            .map_err(|err| BridgeError::PriceConversion(err))?;

        Ok(Trade {
            id: trade_id,
            buy_order_id: if sbe_trade.direction == 0 { sbe_trade.trade_id } else { 0 }, 
            sell_order_id: if sbe_trade.direction == 1 { sbe_trade.trade_id } else { 0 },
            price: price_scaled,
            quantity,
            timestamp: sbe_trade.timestamp_ms as i64,
        })
    }

    pub fn create_external_order_from_book_change(&self, 
        change: &BookChange,
        instrument_id: u32,
        order_id: u64
    ) -> Result<Order, BridgeError> {
        let side = match change.side {
            0 => Side::Sell, 
            1 => Side::Buy,  
            _ => return Err(BridgeError::InvalidSide(change.side)),
        };

        let instrument = {
            let instruments = self.instruments.read();
            instruments.get(&instrument_id)
                .ok_or(BridgeError::UnknownInstrument(instrument_id))?
                .clone()
        };

        let price_scaled = crate::price_utils::float_to_scaled_price(change.price)
            .map_err(|err| BridgeError::PriceConversion(err))?;
        let quantity = crate::price_utils::float_to_scaled_quantity(change.amount)
            .map_err(|err| BridgeError::PriceConversion(err))?;

        let external_user_id = self.get_next_external_user_id();

        Ok(Order {
            id: order_id,
            symbol: instrument.symbol,
            side,
            order_type: OrderType::Limit,
            price: price_scaled,
            quantity,
            filled_quantity: 0,
            status: OrderStatus::New,
            timestamp: Order::get_nano_timestamp(), 
            user_id: external_user_id, 
            time_in_force: TimeInForce::GTC,
            expiration_time: 0, 
            stop_price: None,
            display_quantity: Some(quantity), 
        })
    }

    

    pub fn get_instrument(&self, instrument_id: u32) -> Option<DeribitInstrument> {
        let instruments = self.instruments.read();
        instruments.get(&instrument_id).cloned()
    }

    pub fn get_instrument_by_symbol(&self, symbol: &str) -> Option<DeribitInstrument> {
        let symbol_map = self.symbol_to_id.read();
        let instrument_id = symbol_map.get(symbol)?;
        
        let instruments = self.instruments.read();
        instruments.get(instrument_id).cloned()
    }

    pub fn list_instruments(&self) -> Vec<DeribitInstrument> {
        let instruments = self.instruments.read();
        instruments.values().cloned().collect()
    }

    pub fn update_orderbook_from_market_data(
        &self,
        _orderbook: &mut OrderBook,
        update: &MarketDataUpdate
    ) -> Result<(), BridgeError> {
        debug!("Updating orderbook for {} with market data", update.symbol);

        if let Some((bid_price, bid_amount)) = update.best_bid {
            let _price_scaled = crate::price_utils::float_to_scaled_price(bid_price)
                .map_err(|err| BridgeError::PriceConversion(err))?;
            let _quantity = crate::price_utils::float_to_scaled_quantity(bid_amount)
                .map_err(|err| BridgeError::PriceConversion(err))?;
            
        }

        if let Some((ask_price, ask_amount)) = update.best_ask {
            let _price_scaled = crate::price_utils::float_to_scaled_price(ask_price)
                .map_err(|err| BridgeError::PriceConversion(err))?;
            let _quantity = crate::price_utils::float_to_scaled_quantity(ask_amount)
                .map_err(|err| BridgeError::PriceConversion(err))?;
            
        }

        Ok(())
    }
}

impl Default for SbeBridge {
    fn default() -> Self {
        Self::new(PRICE_SCALE_FACTOR) 
    }
}