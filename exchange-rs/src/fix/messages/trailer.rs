use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Trailer {
    pub checksum: u8, // Tag 10
}

impl Trailer {
    pub fn parse(fields: &HashMap<u32, FixField>) -> Result<Trailer, FixError> {
        let checksum = fields.get(&10)
            .and_then(|f| f.as_int())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag: 10 })? as u8;

        Ok(Trailer { checksum })
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        // Checksum is validated during parsing, so this is mostly a placeholder
        Ok(())
    }
}