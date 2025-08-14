use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::{StandardHeader, Trailer, Header};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NewOrderSingle {
    pub header: StandardHeader,
    pub cl_ord_id: String,           // Tag 11
    pub account: Option<String>,      // Tag 1
    pub handl_inst: char,            // Tag 21 - '1', '2', '3'
    pub symbol: String,              // Tag 55
    pub side: char,                  // Tag 54 - '1' Buy, '2' Sell
    pub transact_time: String,       // Tag 60
    pub order_qty: u32,              // Tag 38
    pub ord_type: char,              // Tag 40 - '1' Market, '2' Limit, '3' Stop, '4' Stop limit
    pub price: Option<f64>,          // Tag 44 - Required for limit orders
    pub stop_px: Option<f64>,        // Tag 99 - Required for stop orders
    pub time_in_force: Option<char>, // Tag 59 - '0' Day, '1' GTC, '3' IOC, '4' FOK
    pub exec_inst: Option<String>,   // Tag 18
    pub trailer: Trailer,
}

impl NewOrderSingle {
    pub fn parse(fields: HashMap<u32, FixField>) -> Result<NewOrderSingle, FixError> {
        let header = Header::parse(&fields)?;
        let trailer = Trailer::parse(&fields)?;

        let cl_ord_id = Self::get_required_string(&fields, 11, "ClOrdID")?;
        let account = Self::get_optional_string(&fields, 1);
        let handl_inst = Self::get_required_char(&fields, 21, "HandlInst")?;
        let symbol = Self::get_required_string(&fields, 55, "Symbol")?;
        let side = Self::get_required_char(&fields, 54, "Side")?;
        let transact_time = Self::get_required_string(&fields, 60, "TransactTime")?;
        let order_qty = Self::get_required_int(&fields, 38, "OrderQty")? as u32;
        let ord_type = Self::get_required_char(&fields, 40, "OrdType")?;
        
        let price = Self::get_optional_float(&fields, 44);
        let stop_px = Self::get_optional_float(&fields, 99);
        let time_in_force = Self::get_optional_char(&fields, 59);
        let exec_inst = Self::get_optional_string(&fields, 18);

        let order = NewOrderSingle {
            header,
            cl_ord_id,
            account,
            handl_inst,
            symbol,
            side,
            transact_time,
            order_qty,
            ord_type,
            price,
            stop_px,
            time_in_force,
            exec_inst,
            trailer,
        };

        order.validate()?;
        Ok(order)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.header.validate()?;
        self.trailer.validate()?;

        if self.cl_ord_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 11 });
        }

        if self.symbol.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 55 });
        }

        if !matches!(self.side, '1' | '2') {
            return Err(ValidationError::InvalidFieldValue {
                tag: 54,
                value: self.side.to_string(),
            });
        }

        if !matches!(self.ord_type, '1' | '2' | '3' | '4') {
            return Err(ValidationError::InvalidFieldValue {
                tag: 40,
                value: self.ord_type.to_string(),
            });
        }

        if matches!(self.ord_type, '2' | '4') && self.price.is_none() {
            return Err(ValidationError::MissingRequiredField { tag: 44 });
        }

        if matches!(self.ord_type, '3' | '4') && self.stop_px.is_none() {
            return Err(ValidationError::MissingRequiredField { tag: 99 });
        }

        if self.order_qty == 0 {
            return Err(ValidationError::InvalidFieldValue {
                tag: 38,
                value: self.order_qty.to_string(),
            });
        }

        Ok(())
    }

    fn get_required_string(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<String, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_string())
            .map(|s| s.to_string())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_required_int(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<i64, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_int())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_required_char(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<char, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_char())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_optional_string(fields: &HashMap<u32, FixField>, tag: u32) -> Option<String> {
        fields.get(&tag).and_then(|f| f.as_string()).map(|s| s.to_string())
    }

    fn get_optional_float(fields: &HashMap<u32, FixField>, tag: u32) -> Option<f64> {
        fields.get(&tag).and_then(|f| f.as_float())
    }

    fn get_optional_char(fields: &HashMap<u32, FixField>, tag: u32) -> Option<char> {
        fields.get(&tag).and_then(|f| f.as_char())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Buy,  // '1'
    Sell, // '2'
}

impl Side {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Side::Buy),
            '2' => Some(Side::Sell),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Side::Buy => '1',
            Side::Sell => '2',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrdType {
    Market,    // '1'
    Limit,     // '2'
    Stop,      // '3'
    StopLimit, // '4'
}

impl OrdType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(OrdType::Market),
            '2' => Some(OrdType::Limit),
            '3' => Some(OrdType::Stop),
            '4' => Some(OrdType::StopLimit),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            OrdType::Market => '1',
            OrdType::Limit => '2',
            OrdType::Stop => '3',
            OrdType::StopLimit => '4',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeInForce {
    Day,              // '0'
    GoodTillCancel,   // '1'
    ImmediateOrCancel, // '3'
    FillOrKill,       // '4'
}

impl TimeInForce {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(TimeInForce::Day),
            '1' => Some(TimeInForce::GoodTillCancel),
            '3' => Some(TimeInForce::ImmediateOrCancel),
            '4' => Some(TimeInForce::FillOrKill),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            TimeInForce::Day => '0',
            TimeInForce::GoodTillCancel => '1',
            TimeInForce::ImmediateOrCancel => '3',
            TimeInForce::FillOrKill => '4',
        }
    }
}