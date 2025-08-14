use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::{StandardHeader, Trailer, Header};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub header: StandardHeader,
    pub order_id: String,            // Tag 37
    pub cl_ord_id: String,           // Tag 11
    pub orig_cl_ord_id: Option<String>, // Tag 41
    pub exec_id: String,             // Tag 17
    pub exec_type: char,             // Tag 150
    pub ord_status: char,            // Tag 39
    pub account: Option<String>,     // Tag 1
    pub symbol: String,              // Tag 55
    pub side: char,                  // Tag 54
    pub order_qty: u32,              // Tag 38
    pub ord_type: char,              // Tag 40
    pub price: Option<f64>,          // Tag 44
    pub stop_px: Option<f64>,        // Tag 99
    pub time_in_force: Option<char>, // Tag 59
    pub last_qty: Option<u32>,       // Tag 32
    pub last_px: Option<f64>,        // Tag 31
    pub leaves_qty: u32,             // Tag 151
    pub cum_qty: u32,                // Tag 14
    pub avg_px: Option<f64>,         // Tag 6
    pub transact_time: String,       // Tag 60
    pub text: Option<String>,        // Tag 58
    pub trailer: Trailer,
}

impl ExecutionReport {
    pub fn parse(fields: HashMap<u32, FixField>) -> Result<ExecutionReport, FixError> {
        let header = Header::parse(&fields)?;
        let trailer = Trailer::parse(&fields)?;

        let order_id = Self::get_required_string(&fields, 37, "OrderID")?;
        let cl_ord_id = Self::get_required_string(&fields, 11, "ClOrdID")?;
        let orig_cl_ord_id = Self::get_optional_string(&fields, 41);
        let exec_id = Self::get_required_string(&fields, 17, "ExecID")?;
        let exec_type = Self::get_required_char(&fields, 150, "ExecType")?;
        let ord_status = Self::get_required_char(&fields, 39, "OrdStatus")?;
        let account = Self::get_optional_string(&fields, 1);
        let symbol = Self::get_required_string(&fields, 55, "Symbol")?;
        let side = Self::get_required_char(&fields, 54, "Side")?;
        let order_qty = Self::get_required_int(&fields, 38, "OrderQty")? as u32;
        let ord_type = Self::get_required_char(&fields, 40, "OrdType")?;
        let price = Self::get_optional_float(&fields, 44);
        let stop_px = Self::get_optional_float(&fields, 99);
        let time_in_force = Self::get_optional_char(&fields, 59);
        let last_qty = Self::get_optional_int(&fields, 32).map(|i| i as u32);
        let last_px = Self::get_optional_float(&fields, 31);
        let leaves_qty = Self::get_required_int(&fields, 151, "LeavesQty")? as u32;
        let cum_qty = Self::get_required_int(&fields, 14, "CumQty")? as u32;
        let avg_px = Self::get_optional_float(&fields, 6);
        let transact_time = Self::get_required_string(&fields, 60, "TransactTime")?;
        let text = Self::get_optional_string(&fields, 58);

        let execution_report = ExecutionReport {
            header,
            order_id,
            cl_ord_id,
            orig_cl_ord_id,
            exec_id,
            exec_type,
            ord_status,
            account,
            symbol,
            side,
            order_qty,
            ord_type,
            price,
            stop_px,
            time_in_force,
            last_qty,
            last_px,
            leaves_qty,
            cum_qty,
            avg_px,
            transact_time,
            text,
            trailer,
        };

        execution_report.validate()?;
        Ok(execution_report)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.header.validate()?;
        self.trailer.validate()?;

        if self.order_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 37 });
        }

        if self.cl_ord_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 11 });
        }

        if self.exec_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 17 });
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

    fn get_optional_int(fields: &HashMap<u32, FixField>, tag: u32) -> Option<i64> {
        fields.get(&tag).and_then(|f| f.as_int())
    }

    fn get_optional_float(fields: &HashMap<u32, FixField>, tag: u32) -> Option<f64> {
        fields.get(&tag).and_then(|f| f.as_float())
    }

    fn get_optional_char(fields: &HashMap<u32, FixField>, tag: u32) -> Option<char> {
        fields.get(&tag).and_then(|f| f.as_char())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecType {
    New,              // '0'
    PartialFill,      // '1'
    Fill,             // '2'
    DoneForDay,       // '3'
    Canceled,         // '4'
    Replace,          // '5'
    PendingCancel,    // '6'
    Stopped,          // '7'
    Rejected,         // '8'
    Suspended,        // '9'
    PendingNew,       // 'A'
    Calculated,       // 'B'
    Expired,          // 'C'
    Restated,         // 'D'
    PendingReplace,   // 'E'
    Trade,            // 'F'
    TradeCorrect,     // 'G'
    TradeCancel,      // 'H'
    OrderStatus,      // 'I'
}

impl ExecType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(ExecType::New),
            '1' => Some(ExecType::PartialFill),
            '2' => Some(ExecType::Fill),
            '3' => Some(ExecType::DoneForDay),
            '4' => Some(ExecType::Canceled),
            '5' => Some(ExecType::Replace),
            '6' => Some(ExecType::PendingCancel),
            '7' => Some(ExecType::Stopped),
            '8' => Some(ExecType::Rejected),
            '9' => Some(ExecType::Suspended),
            'A' => Some(ExecType::PendingNew),
            'B' => Some(ExecType::Calculated),
            'C' => Some(ExecType::Expired),
            'D' => Some(ExecType::Restated),
            'E' => Some(ExecType::PendingReplace),
            'F' => Some(ExecType::Trade),
            'G' => Some(ExecType::TradeCorrect),
            'H' => Some(ExecType::TradeCancel),
            'I' => Some(ExecType::OrderStatus),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            ExecType::New => '0',
            ExecType::PartialFill => '1',
            ExecType::Fill => '2',
            ExecType::DoneForDay => '3',
            ExecType::Canceled => '4',
            ExecType::Replace => '5',
            ExecType::PendingCancel => '6',
            ExecType::Stopped => '7',
            ExecType::Rejected => '8',
            ExecType::Suspended => '9',
            ExecType::PendingNew => 'A',
            ExecType::Calculated => 'B',
            ExecType::Expired => 'C',
            ExecType::Restated => 'D',
            ExecType::PendingReplace => 'E',
            ExecType::Trade => 'F',
            ExecType::TradeCorrect => 'G',
            ExecType::TradeCancel => 'H',
            ExecType::OrderStatus => 'I',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrdStatus {
    New,              // '0'
    PartiallyFilled,  // '1'
    Filled,           // '2'
    DoneForDay,       // '3'
    Canceled,         // '4'
    Replaced,         // '5'
    PendingCancel,    // '6'
    Stopped,          // '7'
    Rejected,         // '8'
    Suspended,        // '9'
    PendingNew,       // 'A'
    Calculated,       // 'B'
    Expired,          // 'C'
    AcceptedForBidding, // 'D'
    PendingReplace,   // 'E'
}

impl OrdStatus {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(OrdStatus::New),
            '1' => Some(OrdStatus::PartiallyFilled),
            '2' => Some(OrdStatus::Filled),
            '3' => Some(OrdStatus::DoneForDay),
            '4' => Some(OrdStatus::Canceled),
            '5' => Some(OrdStatus::Replaced),
            '6' => Some(OrdStatus::PendingCancel),
            '7' => Some(OrdStatus::Stopped),
            '8' => Some(OrdStatus::Rejected),
            '9' => Some(OrdStatus::Suspended),
            'A' => Some(OrdStatus::PendingNew),
            'B' => Some(OrdStatus::Calculated),
            'C' => Some(OrdStatus::Expired),
            'D' => Some(OrdStatus::AcceptedForBidding),
            'E' => Some(OrdStatus::PendingReplace),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            OrdStatus::New => '0',
            OrdStatus::PartiallyFilled => '1',
            OrdStatus::Filled => '2',
            OrdStatus::DoneForDay => '3',
            OrdStatus::Canceled => '4',
            OrdStatus::Replaced => '5',
            OrdStatus::PendingCancel => '6',
            OrdStatus::Stopped => '7',
            OrdStatus::Rejected => '8',
            OrdStatus::Suspended => '9',
            OrdStatus::PendingNew => 'A',
            OrdStatus::Calculated => 'B',
            OrdStatus::Expired => 'C',
            OrdStatus::AcceptedForBidding => 'D',
            OrdStatus::PendingReplace => 'E',
        }
    }
}