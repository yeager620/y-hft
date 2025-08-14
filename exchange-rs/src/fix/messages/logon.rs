use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::{StandardHeader, Trailer, Header};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Logon {
    pub header: StandardHeader,
    pub encrypt_method: char,       
    pub heart_bt_int: u32,         
    pub raw_data_length: Option<u32>, 
    pub raw_data: Option<Vec<u8>>, 
    pub reset_seq_num_flag: Option<bool>, 
    pub next_expected_msg_seq_num: Option<u32>, 
    pub username: Option<String>,   
    pub password: Option<String>,   
    pub trailer: Trailer,
}

impl Logon {
    pub fn parse(fields: HashMap<u32, FixField>) -> Result<Logon, FixError> {
        let header = Header::parse(&fields)?;
        let trailer = Trailer::parse(&fields)?;

        let encrypt_method = Self::get_required_char(&fields, 98, "EncryptMethod")?;
        let heart_bt_int = Self::get_required_int(&fields, 108, "HeartBtInt")? as u32;
        let raw_data_length = Self::get_optional_int(&fields, 95).map(|i| i as u32);
        let raw_data = Self::get_optional_data(&fields, 96);
        let reset_seq_num_flag = Self::get_optional_bool(&fields, 141);
        let next_expected_msg_seq_num = Self::get_optional_int(&fields, 789).map(|i| i as u32);
        let username = Self::get_optional_string(&fields, 553);
        let password = Self::get_optional_string(&fields, 554);

        let logon = Logon {
            header,
            encrypt_method,
            heart_bt_int,
            raw_data_length,
            raw_data,
            reset_seq_num_flag,
            next_expected_msg_seq_num,
            username,
            password,
            trailer,
        };

        logon.validate()?;
        Ok(logon)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.header.validate()?;
        self.trailer.validate()?;

        if !matches!(self.encrypt_method, '0' | '1' | '2' | '3') {
            return Err(ValidationError::InvalidFieldValue {
                tag: 98,
                value: self.encrypt_method.to_string(),
            });
        }

        if self.heart_bt_int == 0 {
            return Err(ValidationError::InvalidFieldValue {
                tag: 108,
                value: self.heart_bt_int.to_string(),
            });
        }

        Ok(())
    }

    fn get_required_char(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<char, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_char())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_required_int(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<i64, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_int())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_optional_string(fields: &HashMap<u32, FixField>, tag: u32) -> Option<String> {
        fields.get(&tag).and_then(|f| f.as_string()).map(|s| s.to_string())
    }

    fn get_optional_int(fields: &HashMap<u32, FixField>, tag: u32) -> Option<i64> {
        fields.get(&tag).and_then(|f| f.as_int())
    }

    fn get_optional_bool(fields: &HashMap<u32, FixField>, tag: u32) -> Option<bool> {
        fields.get(&tag).and_then(|f| f.as_bool())
    }

    fn get_optional_data(fields: &HashMap<u32, FixField>, tag: u32) -> Option<Vec<u8>> {
        fields.get(&tag).and_then(|f| match &f.value {
            crate::fix::parser::field_parser::FieldValue::Data(data) => Some(data.clone()),
            _ => None,
        })
    }
}