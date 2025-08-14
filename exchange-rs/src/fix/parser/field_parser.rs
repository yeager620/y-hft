use crate::fix::error::ParseError;
use crate::fix::parser::raw_parser::RawField;
use std::str;

#[derive(Debug, Clone)]
pub struct FixField {
    pub tag: u32,
    pub value: FieldValue,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Data(Vec<u8>),
    UTCTimestamp(String), 
    UTCDateOnly(String),
    UTCTimeOnly(String),
}

pub struct FieldParser;

impl FieldParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_field(&self, raw_field: RawField<'_>) -> Result<FixField, ParseError> {
        let tag = self.parse_tag(raw_field.tag)?;
        let value = self.parse_value(tag, raw_field.value)?;
        
        Ok(FixField { tag, value })
    }

    fn parse_tag(&self, tag_bytes: &[u8]) -> Result<u32, ParseError> {
        let tag_str = str::from_utf8(tag_bytes)
            .map_err(|_| ParseError::InvalidTag {
                tag: String::from_utf8_lossy(tag_bytes).to_string(),
            })?;
        
        tag_str.parse::<u32>()
            .map_err(|_| ParseError::InvalidTag {
                tag: tag_str.to_string(),
            })
    }

    fn parse_value(&self, tag: u32, value_bytes: &[u8]) -> Result<FieldValue, ParseError> {
        let value_str = str::from_utf8(value_bytes)
            .map_err(|_| ParseError::InvalidFieldValue {
                tag,
                value: String::from_utf8_lossy(value_bytes).to_string(),
            })?;

        match self.get_field_type(tag) {
            FieldType::String => Ok(FieldValue::String(value_str.to_string())),
            FieldType::Int => {
                let int_val = value_str.parse::<i64>()
                    .map_err(|_| ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    })?;
                Ok(FieldValue::Int(int_val))
            },
            FieldType::Float => {
                let float_val = value_str.parse::<f64>()
                    .map_err(|_| ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    })?;
                Ok(FieldValue::Float(float_val))
            },
            FieldType::Bool => {
                let bool_val = match value_str {
                    "Y" | "y" => true,
                    "N" | "n" => false,
                    _ => return Err(ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    }),
                };
                Ok(FieldValue::Bool(bool_val))
            },
            FieldType::Char => {
                if value_str.len() != 1 {
                    return Err(ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    });
                }
                Ok(FieldValue::Char(value_str.chars().next().unwrap()))
            },
            FieldType::Data => Ok(FieldValue::Data(value_bytes.to_vec())),
            FieldType::UTCTimestamp => {
                
                if self.validate_utc_timestamp(value_str) {
                    Ok(FieldValue::UTCTimestamp(value_str.to_string()))
                } else {
                    Err(ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    })
                }
            },
            FieldType::UTCDateOnly => {
                
                if self.validate_utc_date_only(value_str) {
                    Ok(FieldValue::UTCDateOnly(value_str.to_string()))
                } else {
                    Err(ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    })
                }
            },
            FieldType::UTCTimeOnly => {
                
                if self.validate_utc_time_only(value_str) {
                    Ok(FieldValue::UTCTimeOnly(value_str.to_string()))
                } else {
                    Err(ParseError::InvalidFieldValue {
                        tag,
                        value: value_str.to_string(),
                    })
                }
            },
        }
    }

    fn get_field_type(&self, tag: u32) -> FieldType {
        match tag {
            
            8 | 35 | 49 | 56 | 11 | 55 | 1 | 15 | 22 | 48 | 57 | 142 | 37 | 17 | 20 | 39 => FieldType::String,
            
            9 | 34 | 38 | 90 | 95 | 96 | 123 | 36 | 151 | 14 | 6 | 16 => FieldType::Int,
            
            44 | 31 | 32 | 99 | 423 | 424 => FieldType::Float,
            
            40 | 54 | 21 | 59 | 18 | 98 | 103 | 114 | 139 | 47 => FieldType::Char,
            
            43 | 97 | 141 | 89 => FieldType::Bool,
            
            91 | 212 | 213 => FieldType::Data,
            
            52 | 60 | 122 | 273 => FieldType::UTCTimestamp,
            
            64 | 126 => FieldType::UTCDateOnly,
            
            271 => FieldType::UTCTimeOnly,
            
            _ => FieldType::String,
        }
    }

    fn validate_utc_timestamp(&self, value: &str) -> bool {
        
        if value.len() < 17 || value.len() > 21 {
            return false;
        }
        
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 2 {
            return false;
        }
        
        
        if !self.validate_utc_date_only(parts[0]) {
            return false;
        }
        
        
        self.validate_utc_time_only(parts[1])
    }
    
    fn validate_utc_date_only(&self, value: &str) -> bool {
        
        if value.len() != 8 {
            return false;
        }
        
        if let Ok(year) = value[0..4].parse::<u32>() {
            if let Ok(month) = value[4..6].parse::<u32>() {
                if let Ok(day) = value[6..8].parse::<u32>() {
                    return year >= 1900 && year <= 2100 
                        && month >= 1 && month <= 12
                        && day >= 1 && day <= 31;
                }
            }
        }
        false
    }
    
    fn validate_utc_time_only(&self, value: &str) -> bool {
        
        if value.len() < 8 || value.len() > 12 {
            return false;
        }
        
        let time_parts: Vec<&str> = value.split(':').collect();
        if time_parts.len() != 3 {
            return false;
        }
        
        
        if let Ok(hours) = time_parts[0].parse::<u32>() {
            if hours > 23 {
                return false;
            }
        } else {
            return false;
        }
        
        
        if let Ok(minutes) = time_parts[1].parse::<u32>() {
            if minutes > 59 {
                return false;
            }
        } else {
            return false;
        }
        
        
        let seconds_part = time_parts[2];
        if let Some(dot_pos) = seconds_part.find('.') {
            
            let seconds_str = &seconds_part[0..dot_pos];
            let millis_str = &seconds_part[dot_pos + 1..];
            
            if let Ok(seconds) = seconds_str.parse::<u32>() {
                if seconds > 59 {
                    return false;
                }
            } else {
                return false;
            }
            
            if millis_str.len() > 3 {
                return false;
            }
            
            millis_str.parse::<u32>().is_ok()
        } else {
            
            if let Ok(seconds) = seconds_part.parse::<u32>() {
                seconds <= 59
            } else {
                false
            }
        }
    }
}

#[derive(Debug, Clone)]
enum FieldType {
    String,
    Int,
    Float,
    Bool,
    Char,
    Data,
    UTCTimestamp,
    UTCDateOnly,
    UTCTimeOnly,
}

impl FixField {
    pub fn as_string(&self) -> Option<&str> {
        match &self.value {
            FieldValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self.value {
            FieldValue::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self.value {
            FieldValue::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_char(&self) -> Option<char> {
        match self.value {
            FieldValue::Char(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.value {
            FieldValue::Bool(b) => Some(b),
            _ => None,
        }
    }
    
    pub fn as_utc_timestamp(&self) -> Option<&str> {
        match &self.value {
            FieldValue::UTCTimestamp(ts) => Some(ts),
            _ => None,
        }
    }
    
    pub fn as_utc_date_only(&self) -> Option<&str> {
        match &self.value {
            FieldValue::UTCDateOnly(date) => Some(date),
            _ => None,
        }
    }
    
    pub fn as_utc_time_only(&self) -> Option<&str> {
        match &self.value {
            FieldValue::UTCTimeOnly(time) => Some(time),
            _ => None,
        }
    }
    
    pub fn as_data(&self) -> Option<&[u8]> {
        match &self.value {
            FieldValue::Data(data) => Some(data),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fix::parser::raw_parser::RawField;

    #[test]
    fn test_parse_string_field() {
        let parser = FieldParser::new();
        let raw_field = RawField {
            tag: b"8",
            value: b"FIX.4.4",
        };
        
        let field = parser.parse_field(raw_field).unwrap();
        assert_eq!(field.tag, 8);
        assert_eq!(field.as_string(), Some("FIX.4.4"));
    }

    #[test]
    fn test_parse_int_field() {
        let parser = FieldParser::new();
        let raw_field = RawField {
            tag: b"9",
            value: b"178",
        };
        
        let field = parser.parse_field(raw_field).unwrap();
        assert_eq!(field.tag, 9);
        assert_eq!(field.as_int(), Some(178));
    }

    #[test]
    fn test_parse_float_field() {
        let parser = FieldParser::new();
        let raw_field = RawField {
            tag: b"44",
            value: b"15.75",
        };
        
        let field = parser.parse_field(raw_field).unwrap();
        assert_eq!(field.tag, 44);
        assert_eq!(field.as_float(), Some(15.75));
    }
}