use crate::fix::parser::FixField;
use crate::fix::error::{FixError, ValidationError};
use crate::fix::messages::{
    FixMessage, MessageType, NewOrderSingle, ExecutionReport, 
    OrderCancelRequest, Heartbeat, Logon
};
use std::collections::HashMap;

pub struct MessageBuilder;

impl MessageBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_message(&self, fields: HashMap<u32, FixField>) -> Result<FixMessage, FixError> {
        let msg_type_field = fields.get(&35)
            .ok_or_else(|| ValidationError::MissingRequiredField { tag: 35 })?;
        
        let msg_type_str = msg_type_field.as_string()
            .ok_or_else(|| ValidationError::InvalidFieldValue {
                tag: 35,
                value: format!("{:?}", msg_type_field.value),
            })?;

        let msg_type = MessageType::from_str(msg_type_str)
            .ok_or_else(|| ValidationError::InvalidMessageType {
                msg_type: msg_type_str.to_string(),
            })?;

        match msg_type {
            MessageType::NewOrderSingle => {
                let order = NewOrderSingle::parse(fields)?;
                Ok(FixMessage::NewOrderSingle(order))
            }
            MessageType::ExecutionReport => {
                let exec_report = ExecutionReport::parse(fields)?;
                Ok(FixMessage::ExecutionReport(exec_report))
            }
            MessageType::OrderCancelRequest => {
                let cancel_request = OrderCancelRequest::parse(fields)?;
                Ok(FixMessage::OrderCancelRequest(cancel_request))
            }
            MessageType::Heartbeat => {
                let heartbeat = Heartbeat::parse(fields)?;
                Ok(FixMessage::Heartbeat(heartbeat))
            }
            MessageType::Logon => {
                let logon = Logon::parse(fields)?;
                Ok(FixMessage::Logon(logon))
            }
            _ => Err(FixError::Validation(ValidationError::InvalidMessageType {
                msg_type: msg_type_str.to_string(),
            }))
        }
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}