use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::MessageType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StandardHeader {
    pub begin_string: String,     
    pub body_length: u32,         
    pub msg_type: MessageType,    
    pub sender_comp_id: String,   
    pub target_comp_id: String,   
    pub msg_seq_num: u32,         
    pub sending_time: String,     
    pub poss_dup_flag: Option<bool>, 
    pub poss_resend: Option<bool>,   
    pub secure_data_len: Option<u32>, 
    pub secure_data: Option<Vec<u8>>, 
}

pub struct Header;

impl Header {
    pub fn parse(fields: &HashMap<u32, FixField>) -> Result<StandardHeader, FixError> {
        let begin_string = Self::get_required_string(fields, 8, "BeginString")?;
        let body_length = Self::get_required_int(fields, 9, "BodyLength")? as u32;
        
        let msg_type_str = Self::get_required_string(fields, 35, "MsgType")?;
        let msg_type = MessageType::from_str(&msg_type_str)
            .ok_or_else(|| ValidationError::InvalidMessageType {
                msg_type: msg_type_str.clone(),
            })?;
            
        let sender_comp_id = Self::get_required_string(fields, 49, "SenderCompID")?;
        let target_comp_id = Self::get_required_string(fields, 56, "TargetCompID")?;
        let msg_seq_num = Self::get_required_int(fields, 34, "MsgSeqNum")? as u32;
        let sending_time = Self::get_required_string(fields, 52, "SendingTime")?;
        
        let poss_dup_flag = Self::get_optional_bool(fields, 43);
        let poss_resend = Self::get_optional_bool(fields, 97);
        let secure_data_len = Self::get_optional_int(fields, 90).map(|i| i as u32);
        let secure_data = Self::get_optional_data(fields, 91);

        Ok(StandardHeader {
            begin_string,
            body_length,
            msg_type,
            sender_comp_id,
            target_comp_id,
            msg_seq_num,
            sending_time,
            poss_dup_flag,
            poss_resend,
            secure_data_len,
            secure_data,
        })
    }

    fn get_required_string(fields: &HashMap<u32, FixField>, tag: u32, _name: &str) -> Result<String, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_string())
            .map(|s| s.to_string())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_required_int(fields: &HashMap<u32, FixField>, tag: u32, name: &str) -> Result<i64, ValidationError> {
        fields.get(&tag)
            .and_then(|f| f.as_int())
            .ok_or_else(|| ValidationError::MissingRequiredField { tag })
    }

    fn get_optional_bool(fields: &HashMap<u32, FixField>, tag: u32) -> Option<bool> {
        fields.get(&tag).and_then(|f| f.as_bool())
    }

    fn get_optional_int(fields: &HashMap<u32, FixField>, tag: u32) -> Option<i64> {
        fields.get(&tag).and_then(|f| f.as_int())
    }

    fn get_optional_data(fields: &HashMap<u32, FixField>, tag: u32) -> Option<Vec<u8>> {
        fields.get(&tag).and_then(|f| match &f.value {
            crate::fix::parser::field_parser::FieldValue::Data(data) => Some(data.clone()),
            _ => None,
        })
    }
}

impl StandardHeader {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.begin_string != "FIX.4.4" {
            return Err(ValidationError::InvalidMessageType {
                msg_type: self.begin_string.clone(),
            });
        }

        if self.sender_comp_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 49 });
        }

        if self.target_comp_id.is_empty() {
            return Err(ValidationError::MissingRequiredField { tag: 56 });
        }

        if self.msg_seq_num == 0 {
            return Err(ValidationError::MissingRequiredField { tag: 34 });
        }

        Ok(())
    }
}