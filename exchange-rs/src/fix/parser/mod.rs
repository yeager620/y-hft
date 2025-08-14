pub mod raw_parser;
pub mod field_parser;
pub mod message_builder;

pub use raw_parser::RawParser;
pub use field_parser::{FieldParser, FixField};
pub use message_builder::MessageBuilder;

use crate::fix::error::{FixError, ParseError};
use crate::fix::messages::FixMessage;
use std::collections::HashMap;

pub struct FixParser {
    raw_parser: RawParser,
    field_parser: FieldParser,
    message_builder: MessageBuilder,
}

impl FixParser {
    pub fn new() -> Self {
        Self {
            raw_parser: RawParser::new(),
            field_parser: FieldParser::new(),
            message_builder: MessageBuilder::new(),
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<FixMessage, FixError> {
        let raw_fields = self.raw_parser.parse(data)?;
        
        let mut fields = HashMap::new();
        for raw_field in raw_fields {
            let field = self.field_parser.parse_field(raw_field)?;
            fields.insert(field.tag, field);
        }
        
        self.message_builder.build_message(fields)
    }

    pub fn validate_checksum(&self, data: &[u8]) -> Result<(), ParseError> {
        self.raw_parser.validate_checksum(data)
    }
}

impl Default for FixParser {
    fn default() -> Self {
        Self::new()
    }
}