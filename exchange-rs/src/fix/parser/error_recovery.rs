use crate::fix::error::{FixError, ParseError, ValidationError};
use crate::fix::parser::{FixParser, FixField};
use crate::fix::messages::{FixMessage, MessageType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ErrorRecovery {
    pub allow_partial_parse: bool,
    pub skip_invalid_fields: bool,
    pub max_recovery_attempts: u32,
    pub recover_from_checksum_errors: bool,
}

#[derive(Debug, Clone)]
pub struct RecoveryResult {
    pub message: Option<FixMessage>,
    pub errors: Vec<FixError>,
    pub warnings: Vec<String>,
    pub recovered_fields: Vec<u32>,
    pub skipped_fields: Vec<u32>,
    pub recovery_attempts: u32,
}

pub struct RecoveringParser {
    base_parser: FixParser,
    recovery_config: ErrorRecovery,
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self {
            allow_partial_parse: true,
            skip_invalid_fields: true,
            max_recovery_attempts: 3,
            recover_from_checksum_errors: false,
        }
    }
}

impl RecoveringParser {
    pub fn new(recovery_config: ErrorRecovery) -> Self {
        Self {
            base_parser: FixParser::new(),
            recovery_config,
        }
    }
    
    pub fn parse_with_recovery(&mut self, data: &[u8]) -> RecoveryResult {
        let mut result = RecoveryResult {
            message: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            recovered_fields: Vec::new(),
            skipped_fields: Vec::new(),
            recovery_attempts: 0,
        };
        
        
        match self.base_parser.parse(data) {
            Ok(message) => {
                result.message = Some(message);
                return result;
            }
            Err(error) => {
                result.errors.push(error.clone());
                
                if !self.recovery_config.allow_partial_parse {
                    return result;
                }
                
                
                self.attempt_recovery(data, &error, &mut result);
            }
        }
        
        result
    }
    
    fn attempt_recovery(&mut self, data: &[u8], error: &FixError, result: &mut RecoveryResult) {
        match error {
            FixError::Parse(parse_error) => {
                self.recover_from_parse_error(data, parse_error, result);
            }
            FixError::Validation(validation_error) => {
                self.recover_from_validation_error(data, validation_error, result);
            }
            FixError::Session(_) => {
                result.warnings.push("Session error - attempting message reconstruction".to_string());
                self.attempt_message_reconstruction(data, result);
            }
            FixError::Business(_) => {
                result.warnings.push("Business error - parsing message with relaxed validation".to_string());
                self.parse_with_relaxed_validation(data, result);
            }
        }
    }
    
    fn recover_from_parse_error(&mut self, data: &[u8], error: &ParseError, result: &mut RecoveryResult) {
        match error {
            ParseError::InvalidChecksum { .. } => {
                if self.recovery_config.recover_from_checksum_errors {
                    result.warnings.push("Ignoring checksum validation".to_string());
                    if let Ok(message) = self.parse_ignoring_checksum(data) {
                        result.message = Some(message);
                        result.recovery_attempts += 1;
                    }
                }
            }
            ParseError::InvalidBodyLength { .. } => {
                result.warnings.push("Attempting to parse with corrected body length".to_string());
                if let Ok(message) = self.parse_with_corrected_body_length(data) {
                    result.message = Some(message);
                    result.recovery_attempts += 1;
                }
            }
            ParseError::MissingSoh => {
                result.warnings.push("Attempting to reconstruct SOH delimiters".to_string());
                if let Some(corrected_data) = self.reconstruct_soh_delimiters(data) {
                    if let Ok(message) = self.base_parser.parse(&corrected_data) {
                        result.message = Some(message);
                        result.recovery_attempts += 1;
                    }
                }
            }
            ParseError::InvalidCharacter { position, .. } => {
                result.warnings.push(format!("Skipping invalid character at position {}", position));
                if let Some(cleaned_data) = self.clean_invalid_characters(data) {
                    if let Ok(message) = self.base_parser.parse(&cleaned_data) {
                        result.message = Some(message);
                        result.recovery_attempts += 1;
                    }
                }
            }
            _ => {
                result.warnings.push("Attempting partial message reconstruction".to_string());
                self.attempt_message_reconstruction(data, result);
            }
        }
    }
    
    fn recover_from_validation_error(&mut self, data: &[u8], error: &ValidationError, result: &mut RecoveryResult) {
        match error {
            ValidationError::MissingRequiredField { tag } => {
                result.warnings.push(format!("Attempting to provide default value for missing field {}", tag));
                if let Ok(message) = self.parse_with_default_field(data, *tag) {
                    result.message = Some(message);
                    result.recovered_fields.push(*tag);
                    result.recovery_attempts += 1;
                }
            }
            ValidationError::InvalidFieldValue { tag, .. } => {
                if self.recovery_config.skip_invalid_fields {
                    result.warnings.push(format!("Skipping invalid field {}", tag));
                    if let Ok(message) = self.parse_skipping_field(data, *tag) {
                        result.message = Some(message);
                        result.skipped_fields.push(*tag);
                        result.recovery_attempts += 1;
                    }
                }
            }
            ValidationError::FieldOrderingViolation { .. } => {
                result.warnings.push("Attempting to reorder fields".to_string());
                if let Some(reordered_data) = self.reorder_fields(data) {
                    if let Ok(message) = self.base_parser.parse(&reordered_data) {
                        result.message = Some(message);
                        result.recovery_attempts += 1;
                    }
                }
            }
            _ => {
                result.warnings.push("Attempting relaxed validation".to_string());
                self.parse_with_relaxed_validation(data, result);
            }
        }
    }
    
    fn parse_ignoring_checksum(&mut self, data: &[u8]) -> Result<FixMessage, FixError> {
        
        let raw_fields = self.base_parser.raw_parser.parse(data)?;
        let mut fields = HashMap::new();
        
        for raw_field in raw_fields {
            let field = self.base_parser.field_parser.parse_field(raw_field)?;
            fields.insert(field.tag, field);
        }
        
        self.base_parser.message_builder.build_message(fields)
    }
    
    fn parse_with_corrected_body_length(&mut self, data: &[u8]) -> Result<FixMessage, FixError> {
        
        if let Some(corrected_data) = self.calculate_correct_body_length(data) {
            self.base_parser.parse(&corrected_data)
        } else {
            Err(ParseError::InvalidFormat.into())
        }
    }
    
    fn calculate_correct_body_length(&self, data: &[u8]) -> Option<Vec<u8>> {
        
        let data_str = String::from_utf8_lossy(data);
        if let Some(body_length_start) = data_str.find("9=") {
            if let Some(body_length_end) = data_str[body_length_start..].find('\x01') {
                let body_start = body_length_start + body_length_end + 1;
                
                
                if let Some(checksum_start) = data_str.rfind("10=") {
                    let actual_body_length = checksum_start - body_start;
                    let corrected_length_field = format!("9={}", actual_body_length);
                    
                    let mut corrected = Vec::new();
                    corrected.extend_from_slice(&data[..body_length_start]);
                    corrected.extend_from_slice(corrected_length_field.as_bytes());
                    corrected.push(0x01);
                    corrected.extend_from_slice(&data[body_start..]);
                    
                    return Some(corrected);
                }
            }
        }
        None
    }
    
    fn reconstruct_soh_delimiters(&self, data: &[u8]) -> Option<Vec<u8>> {
        let data_str = String::from_utf8_lossy(data);
        
        
        let mut result = Vec::new();
        let mut in_value = false;
        let mut tag_buffer = String::new();
        
        for (i, &byte) in data.iter().enumerate() {
            if byte == b'=' {
                in_value = true;
                result.push(byte);
            } else if in_value && (byte.is_ascii_digit() && data.get(i + 1) == Some(&b'=')) {
                
                result.push(byte);
                result.push(0x01); 
                in_value = false;
            } else {
                result.push(byte);
            }
        }
        
        Some(result)
    }
    
    fn clean_invalid_characters(&self, data: &[u8]) -> Option<Vec<u8>> {
        let mut cleaned = Vec::new();
        
        for &byte in data {
            
            if byte == 0x01 || (byte >= 0x20 && byte <= 0x7E) {
                cleaned.push(byte);
            }
        }
        
        if cleaned.len() != data.len() {
            Some(cleaned)
        } else {
            None
        }
    }
    
    fn parse_with_default_field(&mut self, data: &[u8], missing_tag: u32) -> Result<FixMessage, FixError> {
        
        match self.base_parser.raw_parser.parse(data) {
            Ok(raw_fields) => {
                let mut fields = HashMap::new();
                
                for raw_field in raw_fields {
                    let field = self.base_parser.field_parser.parse_field(raw_field)?;
                    fields.insert(field.tag, field);
                }
                
                
                if let Some(default_field) = self.create_default_field(missing_tag) {
                    fields.insert(missing_tag, default_field);
                }
                
                self.base_parser.message_builder.build_message(fields)
            }
            Err(e) => Err(FixError::Parse(e))
        }
    }
    
    fn create_default_field(&self, tag: u32) -> Option<FixField> {
        use crate::fix::parser::field_parser::FieldValue;
        
        match tag {
            11 => Some(FixField { tag, value: FieldValue::String("DEFAULT".to_string()) }), 
            21 => Some(FixField { tag, value: FieldValue::Char('1') }), 
            38 => Some(FixField { tag, value: FieldValue::Int(1) }), 
            40 => Some(FixField { tag, value: FieldValue::Char('1') }), 
            54 => Some(FixField { tag, value: FieldValue::Char('1') }), 
            55 => Some(FixField { tag, value: FieldValue::String("UNKNOWN".to_string()) }), 
            _ => None,
        }
    }
    
    fn parse_skipping_field(&mut self, data: &[u8], skip_tag: u32) -> Result<FixMessage, FixError> {
        match self.base_parser.raw_parser.parse(data) {
            Ok(raw_fields) => {
                let mut fields = HashMap::new();
                
                for raw_field in raw_fields {
                    let field = self.base_parser.field_parser.parse_field(raw_field)?;
                    if field.tag != skip_tag {
                        fields.insert(field.tag, field);
                    }
                }
                
                self.base_parser.message_builder.build_message(fields)
            }
            Err(e) => Err(FixError::Parse(e))
        }
    }
    
    fn reorder_fields(&self, data: &[u8]) -> Option<Vec<u8>> {
        
        
        None
    }
    
    fn parse_with_relaxed_validation(&mut self, data: &[u8], result: &mut RecoveryResult) {
        
        if let Ok(raw_fields) = self.base_parser.raw_parser.parse(data) {
            let mut fields = HashMap::new();
            
            for raw_field in raw_fields {
                match self.base_parser.field_parser.parse_field(raw_field) {
                    Ok(field) => {
                        fields.insert(field.tag, field);
                    }
                    Err(e) => {
                        result.warnings.push(format!("Failed to parse field: {}", e));
                    }
                }
            }
            
            
            if let Some(msg_type_field) = fields.get(&35) {
                if let Some(msg_type_str) = msg_type_field.as_string() {
                    if let Some(msg_type) = MessageType::from_str(msg_type_str) {
                        result.warnings.push(format!("Partially parsed {} message", msg_type_str));
                        
                    }
                }
            }
            
            result.recovery_attempts += 1;
        }
    }
    
    fn attempt_message_reconstruction(&mut self, data: &[u8], result: &mut RecoveryResult) {
        result.warnings.push("Attempting message reconstruction from fragments".to_string());
        
        
        let data_str = String::from_utf8_lossy(data);
        
        
        if data_str.contains("35=D") {
            result.warnings.push("Detected New Order Single message".to_string());
        } else if data_str.contains("35=8") {
            result.warnings.push("Detected Execution Report message".to_string());
        } else if data_str.contains("35=0") {
            result.warnings.push("Detected Heartbeat message".to_string());
        }
        
        result.recovery_attempts += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recovery_config() {
        let config = ErrorRecovery::default();
        assert!(config.allow_partial_parse);
        assert!(config.skip_invalid_fields);
        assert_eq!(config.max_recovery_attempts, 3);
    }
    
    #[test]
    fn test_recovering_parser_creation() {
        let config = ErrorRecovery::default();
        let parser = RecoveringParser::new(config);
        
        assert_eq!(parser.recovery_config.max_recovery_attempts, 3);
    }
}