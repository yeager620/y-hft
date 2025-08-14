use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::{StandardHeader, Trailer, Header};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Heartbeat {
    pub header: StandardHeader,
    pub test_req_id: Option<String>, // Tag 112
    pub trailer: Trailer,
}

impl Heartbeat {
    pub fn parse(fields: HashMap<u32, FixField>) -> Result<Heartbeat, FixError> {
        let header = Header::parse(&fields)?;
        let trailer = Trailer::parse(&fields)?;

        let test_req_id = Self::get_optional_string(&fields, 112);

        let heartbeat = Heartbeat {
            header,
            test_req_id,
            trailer,
        };

        heartbeat.validate()?;
        Ok(heartbeat)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.header.validate()?;
        self.trailer.validate()?;
        Ok(())
    }

    fn get_optional_string(fields: &HashMap<u32, FixField>, tag: u32) -> Option<String> {
        fields.get(&tag).and_then(|f| f.as_string()).map(|s| s.to_string())
    }
}