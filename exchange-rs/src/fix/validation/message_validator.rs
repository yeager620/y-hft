use crate::fix::error::ValidationError;
use crate::fix::messages::{FixMessage, MessageType};
use crate::fix::parser::FixField;
use std::collections::HashMap;

pub struct MessageValidator;

impl MessageValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_message(&self, message: &FixMessage) -> Result<(), ValidationError> {
        match message {
            FixMessage::NewOrderSingle(order) => self.validate_new_order_single_fields(order),
            FixMessage::ExecutionReport(report) => self.validate_execution_report_fields(report),
            FixMessage::OrderCancelRequest(cancel) => self.validate_order_cancel_request_fields(cancel),
            FixMessage::Heartbeat(heartbeat) => self.validate_heartbeat_fields(heartbeat),
            FixMessage::Logon(logon) => self.validate_logon_fields(logon),
        }
    }

    pub fn validate_required_fields(&self, msg_type: &MessageType, fields: &HashMap<u32, FixField>) -> Result<(), ValidationError> {
        let required_fields = self.get_required_fields(msg_type);
        
        for &tag in &required_fields {
            if !fields.contains_key(&tag) {
                return Err(ValidationError::MissingRequiredField { tag });
            }
        }

        Ok(())
    }

    pub fn validate_field_presence(&self, msg_type: &MessageType, tag: u32) -> Result<(), ValidationError> {
        let allowed_fields = self.get_allowed_fields(msg_type);
        
        if !allowed_fields.contains(&tag) {
            return Err(ValidationError::FieldNotAllowed {
                tag,
                msg_type: msg_type.as_str().to_string(),
            });
        }

        Ok(())
    }

    fn get_required_fields(&self, msg_type: &MessageType) -> Vec<u32> {
        let standard_header = vec![8, 9, 35, 49, 56, 34, 52];
        let trailer = vec![10];
        
        let mut required = standard_header;
        required.extend(trailer);

        match msg_type {
            MessageType::NewOrderSingle => {
                required.extend(vec![11, 21, 55, 54, 60, 38, 40]);
            }
            MessageType::ExecutionReport => {
                required.extend(vec![37, 11, 17, 150, 39, 55, 54, 38, 40, 151, 14, 60]);
            }
            MessageType::OrderCancelRequest => {
                required.extend(vec![41, 11, 55, 54, 60]);
            }
            MessageType::Heartbeat => {
                
            }
            MessageType::Logon => {
                required.extend(vec![98, 108]);
            }
            _ => {}
        }

        required
    }

    fn get_allowed_fields(&self, msg_type: &MessageType) -> Vec<u32> {
        let standard_header = vec![8, 9, 35, 49, 56, 34, 52, 43, 97, 90, 91];
        let trailer = vec![10];
        
        let mut allowed = standard_header;
        allowed.extend(trailer);

        match msg_type {
            MessageType::NewOrderSingle => {
                allowed.extend(vec![
                    11, 1, 21, 55, 54, 60, 38, 40, 44, 99, 59, 18
                ]);
            }
            MessageType::ExecutionReport => {
                allowed.extend(vec![
                    37, 11, 41, 17, 150, 39, 1, 55, 54, 38, 40, 44, 99, 59,
                    32, 31, 151, 14, 6, 60, 58
                ]);
            }
            MessageType::OrderCancelRequest => {
                allowed.extend(vec![41, 11, 55, 54, 60, 38, 1, 58]);
            }
            MessageType::Heartbeat => {
                allowed.extend(vec![112]);
            }
            MessageType::Logon => {
                allowed.extend(vec![98, 108, 95, 96, 141, 789, 553, 554]);
            }
            _ => {}
        }

        allowed
    }

    fn validate_new_order_single_fields(&self, order: &crate::fix::messages::NewOrderSingle) -> Result<(), ValidationError> {
        if matches!(order.ord_type, '2' | '4') && order.price.is_none() {
            return Err(ValidationError::MissingRequiredField { tag: 44 });
        }

        if matches!(order.ord_type, '3' | '4') && order.stop_px.is_none() {
            return Err(ValidationError::MissingRequiredField { tag: 99 });
        }

        Ok(())
    }

    fn validate_execution_report_fields(&self, _report: &crate::fix::messages::ExecutionReport) -> Result<(), ValidationError> {
        Ok(())
    }

    fn validate_order_cancel_request_fields(&self, _cancel: &crate::fix::messages::OrderCancelRequest) -> Result<(), ValidationError> {
        Ok(())
    }

    fn validate_heartbeat_fields(&self, _heartbeat: &crate::fix::messages::Heartbeat) -> Result<(), ValidationError> {
        Ok(())
    }

    fn validate_logon_fields(&self, logon: &crate::fix::messages::Logon) -> Result<(), ValidationError> {
        if logon.heart_bt_int == 0 {
            return Err(ValidationError::InvalidFieldValue {
                tag: 108,
                value: logon.heart_bt_int.to_string(),
            });
        }

        Ok(())
    }
}

impl Default for MessageValidator {
    fn default() -> Self {
        Self::new()
    }
}