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
        }
    }

    fn get_field_type(&self, tag: u32) -> FieldType {
        match tag {
            8 | 35 | 49 | 56 | 11 | 55 | 1 => FieldType::String,
            9 | 34 | 52 | 38 | 60 => FieldType::Int,  
            44 => FieldType::Float,  
            40 | 54 | 21 | 59 => FieldType::Char,  
            _ => FieldType::String,  
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