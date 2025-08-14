pub mod raw_parser;
pub mod field_parser;
pub mod message_builder;
pub mod group_parser;
pub mod advanced_parser;
pub mod error_recovery;

pub use raw_parser::RawParser;
pub use field_parser::{FieldParser, FixField};
pub use message_builder::MessageBuilder;
pub use group_parser::{GroupParser, RepeatingGroup, GroupDefinitions};
pub use advanced_parser::{AdvancedFixParser, ParsedMessage, ParsingMetadata, SessionInfo};
pub use error_recovery::{RecoveringParser, ErrorRecovery, RecoveryResult};

use crate::fix::error::{FixError, ParseError};
use crate::fix::messages::FixMessage;
use std::collections::HashMap;

pub struct FixParser {
    raw_parser: RawParser,
    field_parser: FieldParser,
    message_builder: MessageBuilder,
    group_parser: GroupParser,
}

impl FixParser {
    pub fn new() -> Self {
        Self {
            raw_parser: RawParser::new(),
            field_parser: FieldParser::new(),
            message_builder: MessageBuilder::new(),
            group_parser: GroupParser::new(),
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<FixMessage, FixError> {
        
        self.raw_parser.validate_checksum(data)?;
        
        
        self.raw_parser.validate_body_length(data)?;
        
        
        let raw_fields = self.raw_parser.parse(data)?;
        
        
        let mut fields = HashMap::new();
        for raw_field in raw_fields {
            let field = self.field_parser.parse_field(raw_field)?;
            fields.insert(field.tag, field);
        }
        
        
        let message = self.message_builder.build_message(fields)?;
        
        
        self.validate_message(&message)?;
        
        Ok(message)
    }

    pub fn validate_checksum(&self, data: &[u8]) -> Result<(), ParseError> {
        self.raw_parser.validate_checksum(data)
    }
    
    pub fn validate_body_length(&self, data: &[u8]) -> Result<(), ParseError> {
        self.raw_parser.validate_body_length(data)
    }
    
    fn validate_message(&self, message: &FixMessage) -> Result<(), FixError> {
        match message {
            FixMessage::NewOrderSingle(order) => Ok(order.validate()?),
            FixMessage::ExecutionReport(exec) => Ok(exec.validate()?),
            FixMessage::OrderCancelRequest(cancel) => Ok(cancel.validate()?),
            FixMessage::Heartbeat(hb) => Ok(hb.validate()?),
            FixMessage::Logon(logon) => Ok(logon.validate()?),
        }
    }
    
    pub fn extract_header_fields(&mut self, data: &[u8]) -> Result<HashMap<u32, FixField>, FixError> {
        let raw_fields = self.raw_parser.parse(data)?;
        let mut fields = HashMap::new();
        
        
        for raw_field in raw_fields {
            let field = self.field_parser.parse_field(raw_field)?;
            match field.tag {
                8 | 9 | 35 | 49 | 56 | 34 | 52 | 43 | 97 | 90 | 91 => {
                    fields.insert(field.tag, field);
                }
                _ => break, 
            }
        }
        
        Ok(fields)
    }
    
    pub fn parse_repeating_groups(
        &self,
        data: &[u8],
        group_defs: &[group_parser::GroupDef],
    ) -> Result<Vec<RepeatingGroup>, FixError> {
        let raw_fields = self.raw_parser.parse(data)?;
        let mut groups = Vec::new();
        
        for group_def in group_defs {
            if let Some(group) = self.group_parser.parse_repeating_group(
                &raw_fields,
                group_def.count_tag,
                group_def.delimiter_tag,
                group_def.fields,
            )? {
                groups.push(group);
            }
        }
        
        Ok(groups)
    }
}

impl Default for FixParser {
    fn default() -> Self {
        Self::new()
    }
}