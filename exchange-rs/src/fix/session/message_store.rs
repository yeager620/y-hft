use crate::fix::error::FixError;
use crate::fix::messages::FixMessage;
use std::collections::HashMap;

pub struct MessageStore {
    outgoing_messages: HashMap<u32, FixMessage>,
    incoming_messages: HashMap<u32, FixMessage>,
}

impl MessageStore {
    pub fn new() -> Self {
        Self {
            outgoing_messages: HashMap::new(),
            incoming_messages: HashMap::new(),
        }
    }

    pub fn store_outgoing_message(&mut self, message: &FixMessage) -> Result<(), FixError> {
        let seq_num = self.extract_seq_num(message)?;
        self.outgoing_messages.insert(seq_num, message.clone());
        Ok(())
    }

    pub fn store_incoming_message(&mut self, message: &FixMessage) -> Result<(), FixError> {
        let seq_num = self.extract_seq_num(message)?;
        self.incoming_messages.insert(seq_num, message.clone());
        Ok(())
    }

    pub fn get_outgoing_message(&self, seq_num: u32) -> Option<&FixMessage> {
        self.outgoing_messages.get(&seq_num)
    }

    pub fn get_incoming_message(&self, seq_num: u32) -> Option<&FixMessage> {
        self.incoming_messages.get(&seq_num)
    }

    pub fn get_outgoing_messages_from(&self, from_seq_num: u32, to_seq_num: u32) -> Vec<&FixMessage> {
        let mut messages = Vec::new();
        for seq_num in from_seq_num..=to_seq_num {
            if let Some(message) = self.outgoing_messages.get(&seq_num) {
                messages.push(message);
            }
        }
        messages
    }

    pub fn clear_old_messages(&mut self, keep_last_n: usize) {
        if self.outgoing_messages.len() > keep_last_n {
            let mut seq_nums: Vec<u32> = self.outgoing_messages.keys().cloned().collect();
            seq_nums.sort();
            
            let remove_count = self.outgoing_messages.len() - keep_last_n;
            for &seq_num in seq_nums.iter().take(remove_count) {
                self.outgoing_messages.remove(&seq_num);
            }
        }

        if self.incoming_messages.len() > keep_last_n {
            let mut seq_nums: Vec<u32> = self.incoming_messages.keys().cloned().collect();
            seq_nums.sort();
            
            let remove_count = self.incoming_messages.len() - keep_last_n;
            for &seq_num in seq_nums.iter().take(remove_count) {
                self.incoming_messages.remove(&seq_num);
            }
        }
    }

    fn extract_seq_num(&self, message: &FixMessage) -> Result<u32, FixError> {
        match message {
            FixMessage::NewOrderSingle(order) => Ok(order.header.msg_seq_num),
            FixMessage::ExecutionReport(report) => Ok(report.header.msg_seq_num),
            FixMessage::OrderCancelRequest(cancel) => Ok(cancel.header.msg_seq_num),
            FixMessage::Heartbeat(heartbeat) => Ok(heartbeat.header.msg_seq_num),
            FixMessage::Logon(logon) => Ok(logon.header.msg_seq_num),
        }
    }
}

impl Default for MessageStore {
    fn default() -> Self {
        Self::new()
    }
}