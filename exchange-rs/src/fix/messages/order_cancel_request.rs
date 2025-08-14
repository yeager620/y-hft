use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::{StandardHeader, Trailer, Header};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OrderCancelRequest {
    pub header: StandardHeader,
    pub orig_cl_ord_id: String,      // Tag 41
    pub cl_ord_id: String,           // Tag 11
    pub symbol: String,              // Tag 55
    pub side: char,                  // Tag 54
    pub transact_time: String,       // Tag 60
    pub order_qty: Option<u32>,      // Tag 38
    pub account: Option<String>,     // Tag 1
    pub text: Option<String>,        // Tag 58
    pub trailer: Trailer,
}

impl OrderCancelRequest {
    pub fn parse(fields: HashMap<u32, FixField>) -> Result<OrderCancelRequest, FixError> {
        let header = Header::parse(&fields)?;
        let trailer = Trailer::parse(&fields)?;

        let orig_cl_ord_id = Self::get_required_string(&fields, 41, "OrigClOrdID")?;
        let cl_ord_id = Self::get_required_string(&fields, 11, "ClOrdID")?;
        let symbol = Self::get_required_string(&fields, 55, "Symbol")?;
        let side = Self::get_required_char(&fields, 54, "Side")?;
        let transact_time = Self::get_required_string(&fields, 60, "TransactTime")?;
        let order_qty = Self::get_optional_int(&fields, 38).map(|i| i as u32);
        let account = Self::get_optional_string(&fields, 1);
        let text = Self::get_optional_string(&fields, 58);

        let cancel_request = OrderCancelRequest {
            header,
            orig_cl_ord_id,
            cl_ord_id,
            symbol,
            side,
            transact_time,
            order_qty,
            account,
            text,
            trailer,
        };

        cancel_request.validate()?;
        Ok(cancel_request)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.header.validate()?;
        self.trailer.validate()?;

        if self.orig_cl_ord_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 41 });
        }

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

        Ok(())
    }

    fn get_required_string(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<String, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_string())
            .map(|s| s.to_string())
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
}