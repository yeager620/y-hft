use crate::fix::parser::{FixParser, FixField, RepeatingGroup, GroupDefinitions};
use crate::fix::error::{FixError, ParseError, ValidationError};
use crate::fix::messages::{FixMessage, MessageType, StandardHeader, Header};
use std::collections::HashMap;

pub struct AdvancedFixParser {
    base_parser: FixParser,
    performance_mode: bool,
    strict_validation: bool,
    supported_versions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub message: FixMessage,
    pub header: StandardHeader,
    pub groups: Vec<RepeatingGroup>,
    pub raw_fields: HashMap<u32, FixField>,
    pub parsing_metadata: ParsingMetadata,
}

#[derive(Debug, Clone)]
pub struct ParsingMetadata {
    pub parse_time_nanos: u64,
    pub message_size: usize,
    pub field_count: usize,
    pub group_count: usize,
    pub validation_errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl AdvancedFixParser {
    pub fn new() -> Self {
        Self {
            base_parser: FixParser::new(),
            performance_mode: false,
            strict_validation: true,
            supported_versions: vec!["FIX.4.2".to_string(), "FIX.4.4".to_string(), "FIX.5.0".to_string()],
        }
    }
    
    pub fn with_performance_mode(mut self, enabled: bool) -> Self {
        self.performance_mode = enabled;
        self
    }
    
    pub fn with_strict_validation(mut self, enabled: bool) -> Self {
        self.strict_validation = enabled;
        self
    }
    
    pub fn with_supported_versions(mut self, versions: Vec<String>) -> Self {
        self.supported_versions = versions;
        self
    }
    
    pub fn parse_advanced(&mut self, data: &[u8]) -> Result<ParsedMessage, FixError> {
        let start_time = std::time::Instant::now();
        let mut metadata = ParsingMetadata {
            parse_time_nanos: 0,
            message_size: data.len(),
            field_count: 0,
            group_count: 0,
            validation_errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        
        if self.performance_mode {
            if let Err(e) = self.quick_validate(data) {
                return Err(e);
            }
        }
        
        
        let header_fields = self.base_parser.extract_header_fields(data)?;
        let header = Header::parse(&header_fields)?;
        
        
        if !self.supported_versions.contains(&header.begin_string) {
            if self.strict_validation {
                return Err(ValidationError::InvalidMessageType {
                    msg_type: header.begin_string.clone(),
                }.into());
            } else {
                metadata.warnings.push(format!("Unsupported FIX version: {}", header.begin_string));
            }
        }
        
        
        let message = self.base_parser.parse(data)?;
        
        
        let groups = self.parse_message_groups(data, &header.msg_type)?;
        metadata.group_count = groups.len();
        
        
        let raw_fields = self.extract_all_fields(data)?;
        metadata.field_count = raw_fields.len();
        
        
        if self.strict_validation {
            self.perform_advanced_validation(&message, &header, &groups, &mut metadata)?;
        }
        
        metadata.parse_time_nanos = start_time.elapsed().as_nanos() as u64;
        
        Ok(ParsedMessage {
            message,
            header,
            groups,
            raw_fields,
            parsing_metadata: metadata,
        })
    }
    
    fn quick_validate(&self, data: &[u8]) -> Result<(), FixError> {
        
        if data.len() < 20 {  
            return Err(ParseError::InvalidFormat.into());
        }
        
        
        if !data.starts_with(b"8=") {
            return Err(ParseError::InvalidFormat.into());
        }
        
        if !data.ends_with(b"\x01") {
            return Err(ParseError::MissingSoh.into());
        }
        
        
        if let Some(checksum_pos) = data.windows(3).rposition(|w| w == b"10=") {
            if data.len() - checksum_pos < 7 {  
                return Err(ParseError::InvalidFormat.into());
            }
        } else {
            return Err(ParseError::InvalidFormat.into());
        }
        
        Ok(())
    }
    
    fn parse_message_groups(&mut self, data: &[u8], msg_type: &MessageType) -> Result<Vec<RepeatingGroup>, FixError> {
        let group_defs = self.get_group_definitions_for_message_type(msg_type);
        
        if group_defs.is_empty() {
            return Ok(Vec::new());
        }
        
        self.base_parser.parse_repeating_groups(data, &group_defs)
    }
    
    fn get_group_definitions_for_message_type(&self, msg_type: &MessageType) -> Vec<crate::fix::parser::group_parser::GroupDef> {
        match msg_type {
            MessageType::NewOrderSingle => vec![
                GroupDefinitions::PARTIES_GROUP,
            ],
            MessageType::MarketDataSnapshotFullRefresh | MessageType::MarketDataIncrementalRefresh => vec![
                GroupDefinitions::MD_ENTRIES_GROUP,
            ],
            MessageType::SecurityDefinition => vec![
                GroupDefinitions::SECURITY_ALT_ID_GROUP,
            ],
            _ => Vec::new(),
        }
    }
    
    fn extract_all_fields(&mut self, data: &[u8]) -> Result<HashMap<u32, FixField>, FixError> {
        let raw_fields = self.base_parser.raw_parser.parse(data)?;
        let mut fields = HashMap::new();
        
        for raw_field in raw_fields {
            let field = self.base_parser.field_parser.parse_field(raw_field)?;
            fields.insert(field.tag, field);
        }
        
        Ok(fields)
    }
    
    fn perform_advanced_validation(
        &self,
        message: &FixMessage,
        header: &StandardHeader,
        groups: &[RepeatingGroup],
        metadata: &mut ParsingMetadata,
    ) -> Result<(), FixError> {
        
        self.validate_business_logic(message, metadata)?;
        
        
        self.validate_group_consistency(groups, metadata)?;
        
        
        self.validate_sequence_timing(header, metadata)?;
        
        Ok(())
    }
    
    fn validate_business_logic(&self, message: &FixMessage, metadata: &mut ParsingMetadata) -> Result<(), FixError> {
        match message {
            FixMessage::NewOrderSingle(order) => {
                
                if order.symbol.len() > 32 {
                    metadata.warnings.push("Symbol length exceeds recommended maximum".to_string());
                }
                
                if let Some(price) = order.price {
                    if price <= 0.0 {
                        return Err(ValidationError::InvalidFieldValue {
                            tag: 44,
                            value: price.to_string(),
                        }.into());
                    }
                }
                
                if order.order_qty > 10_000_000 {
                    metadata.warnings.push("Order quantity unusually large".to_string());
                }
            },
            FixMessage::ExecutionReport(exec) => {
                
                if let Some(last_qty) = exec.last_qty {
                    if last_qty > exec.order_qty {
                        return Err(ValidationError::InvalidFieldValue {
                            tag: 32,
                            value: last_qty.to_string(),
                        }.into());
                    }
                }
            },
            _ => {} 
        }
        
        Ok(())
    }
    
    fn validate_group_consistency(&self, groups: &[RepeatingGroup], metadata: &mut ParsingMetadata) -> Result<(), FixError> {
        for group in groups {
            if group.count as usize != group.instances.len() {
                return Err(ValidationError::InvalidFieldValue {
                    tag: 0, 
                    value: format!("Group count mismatch: declared {} but found {} instances", group.count, group.instances.len()),
                }.into());
            }
            
            
            for (i, instance) in group.instances.iter().enumerate() {
                if instance.is_empty() {
                    metadata.warnings.push(format!("Group instance {} is empty", i));
                }
            }
        }
        
        Ok(())
    }
    
    fn validate_sequence_timing(&self, header: &StandardHeader, metadata: &mut ParsingMetadata) -> Result<(), FixError> {
        
        if header.msg_seq_num == 0 {
            return Err(ValidationError::InvalidFieldValue {
                tag: 34,
                value: "0".to_string(),
            }.into());
        }
        
        
        if header.sending_time.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 52 }.into());
        }
        
        
        if let Ok(parsed_time) = self.parse_fix_timestamp(&header.sending_time) {
            let now = std::time::SystemTime::now();
            let five_minutes = std::time::Duration::from_secs(300);
            
            if parsed_time > now + five_minutes {
                metadata.warnings.push("Message timestamp is in the future".to_string());
            }
        }
        
        Ok(())
    }
    
    fn parse_fix_timestamp(&self, timestamp: &str) -> Result<std::time::SystemTime, ParseError> {
        
        
        if timestamp.len() < 17 {
            return Err(ParseError::InvalidFormat);
        }
        
        
        
        Ok(std::time::SystemTime::now())
    }
    
    pub fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            performance_mode_enabled: self.performance_mode,
            strict_validation_enabled: self.strict_validation,
            supported_versions: self.supported_versions.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub performance_mode_enabled: bool,
    pub strict_validation_enabled: bool,
    pub supported_versions: Vec<String>,
}

impl Default for AdvancedFixParser {
    fn default() -> Self {
        Self::new()
    }
}


impl AdvancedFixParser {
    pub fn extract_message_type(&mut self, data: &[u8]) -> Result<MessageType, FixError> {
        let header_fields = self.base_parser.extract_header_fields(data)?;
        let header = Header::parse(&header_fields)?;
        Ok(header.msg_type)
    }
    
    pub fn is_administrative_message(&mut self, data: &[u8]) -> Result<bool, FixError> {
        let msg_type = self.extract_message_type(data)?;
        Ok(matches!(msg_type, 
            MessageType::Heartbeat | MessageType::TestRequest | MessageType::ResendRequest |
            MessageType::Reject | MessageType::SequenceReset | MessageType::Logout | 
            MessageType::Logon
        ))
    }
    
    pub fn extract_session_info(&mut self, data: &[u8]) -> Result<SessionInfo, FixError> {
        let header_fields = self.base_parser.extract_header_fields(data)?;
        let header = Header::parse(&header_fields)?;
        
        Ok(SessionInfo {
            sender_comp_id: header.sender_comp_id,
            target_comp_id: header.target_comp_id,
            msg_seq_num: header.msg_seq_num,
            sending_time: header.sending_time,
            msg_type: header.msg_type,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub msg_seq_num: u32,
    pub sending_time: String,
    pub msg_type: MessageType,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_parser_creation() {
        let parser = AdvancedFixParser::new()
            .with_performance_mode(true)
            .with_strict_validation(false);
        
        let stats = parser.get_performance_stats();
        assert!(stats.performance_mode_enabled);
        assert!(!stats.strict_validation_enabled);
    }
    
    #[test]
    fn test_quick_validate() {
        let parser = AdvancedFixParser::new().with_performance_mode(true);
        
        
        let valid_msg = b"8=FIX.4.4\x019=50\x0135=D\x0149=SENDER\x0156=TARGET\x0110=161\x01";
        assert!(parser.quick_validate(valid_msg).is_ok());
        
        
        let invalid_msg = b"invalid message";
        assert!(parser.quick_validate(invalid_msg).is_err());
    }
}