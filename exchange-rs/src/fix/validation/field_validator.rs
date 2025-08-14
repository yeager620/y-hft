use crate::fix::error::ValidationError;
use crate::fix::parser::field_parser::FieldValue;

pub struct FieldValidator;

impl FieldValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_field(&self, tag: u32, value: &FieldValue) -> Result<(), ValidationError> {
        match tag {
            8 => self.validate_begin_string(value),
            9 => self.validate_body_length(value),
            35 => self.validate_msg_type(value),
            49 | 56 => self.validate_comp_id(value, tag),
            34 => self.validate_seq_num(value),
            52 => self.validate_sending_time(value),
            11 | 37 | 41 => self.validate_order_id(value, tag),
            55 => self.validate_symbol(value),
            54 => self.validate_side(value),
            40 => self.validate_ord_type(value),
            38 => self.validate_quantity(value),
            44 | 99 => self.validate_price(value, tag),
            59 => self.validate_time_in_force(value),
            60 => self.validate_transact_time(value),
            _ => Ok(()),
        }
    }

    fn validate_begin_string(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if s.starts_with("FIX.") && s.len() >= 7 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 8,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 8,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_body_length(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::Int(i) => {
                if *i > 0 && *i <= 1_000_000 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 9,
                        value: i.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 9,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_msg_type(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if s.len() == 1 && (s.chars().next().unwrap().is_alphanumeric()) {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 35,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 35,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_comp_id(&self, value: &FieldValue, tag: u32) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if !s.is_empty() && s.len() <= 64 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_seq_num(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::Int(i) => {
                if *i > 0 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 34,
                        value: i.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 34,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_sending_time(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if s.len() >= 17 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 52,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 52,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_order_id(&self, value: &FieldValue, tag: u32) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if !s.is_empty() && s.len() <= 64 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_symbol(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if !s.is_empty() && s.len() <= 32 && s.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 55,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 55,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_side(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::Char(c) => {
                if matches!(*c, '1' | '2') {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 54,
                        value: c.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 54,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_ord_type(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::Char(c) => {
                if matches!(*c, '1' | '2' | '3' | '4') {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 40,
                        value: c.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 40,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_quantity(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::Int(i) => {
                if *i > 0 && *i <= 1_000_000_000 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 38,
                        value: i.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 38,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_price(&self, value: &FieldValue, tag: u32) -> Result<(), ValidationError> {
        match value {
            FieldValue::Float(f) => {
                if *f > 0.0 && f.is_finite() {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag,
                        value: f.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_time_in_force(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::Char(c) => {
                if matches!(*c, '0' | '1' | '3' | '4') {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 59,
                        value: c.to_string(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 59,
                value: format!("{:?}", value),
            })
        }
    }

    fn validate_transact_time(&self, value: &FieldValue) -> Result<(), ValidationError> {
        match value {
            FieldValue::String(s) => {
                if s.len() >= 17 {
                    Ok(())
                } else {
                    Err(ValidationError::InvalidFieldValue {
                        tag: 60,
                        value: s.clone(),
                    })
                }
            }
            _ => Err(ValidationError::InvalidFieldValue {
                tag: 60,
                value: format!("{:?}", value),
            })
        }
    }
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self::new()
    }
}