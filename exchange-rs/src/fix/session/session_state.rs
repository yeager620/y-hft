use crate::fix::messages::{StandardHeader, MessageType};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    Disconnected,
    Connected,
    LoggedOn,
    LoggedOut,
    Error,
}

pub struct FixSessionState {
    sender_comp_id: String,
    target_comp_id: String,
    outgoing_seq_num: u32,
    incoming_seq_num: u32,
    status: SessionStatus,
}

impl FixSessionState {
    pub fn new(sender_comp_id: String, target_comp_id: String) -> Self {
        Self {
            sender_comp_id,
            target_comp_id,
            outgoing_seq_num: 1,
            incoming_seq_num: 1,
            status: SessionStatus::Disconnected,
        }
    }

    pub fn create_header(&self, msg_type: MessageType) -> StandardHeader {
        StandardHeader {
            begin_string: "FIX.4.4".to_string(),
            body_length: 0, 
            msg_type,
            sender_comp_id: self.sender_comp_id.clone(),
            target_comp_id: self.target_comp_id.clone(),
            msg_seq_num: self.outgoing_seq_num,
            sending_time: self.get_utc_timestamp(),
            poss_dup_flag: None,
            poss_resend: None,
            secure_data_len: None,
            secure_data: None,
        }
    }

    pub fn increment_outgoing_seq_num(&mut self) {
        self.outgoing_seq_num += 1;
    }

    pub fn increment_incoming_seq_num(&mut self) {
        self.incoming_seq_num += 1;
    }

    pub fn get_outgoing_seq_num(&self) -> u32 {
        self.outgoing_seq_num
    }

    pub fn get_incoming_seq_num(&self) -> u32 {
        self.incoming_seq_num
    }

    pub fn set_outgoing_seq_num(&mut self, seq_num: u32) {
        self.outgoing_seq_num = seq_num;
    }

    pub fn set_incoming_seq_num(&mut self, seq_num: u32) {
        self.incoming_seq_num = seq_num;
    }

    pub fn get_status(&self) -> SessionStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: SessionStatus) {
        self.status = status;
    }

    pub fn is_logged_on(&self) -> bool {
        self.status == SessionStatus::LoggedOn
    }

    pub fn get_sender_comp_id(&self) -> &str {
        &self.sender_comp_id
    }

    pub fn get_target_comp_id(&self) -> &str {
        &self.target_comp_id
    }

    pub fn reset_sequence_numbers(&mut self) {
        self.outgoing_seq_num = 1;
        self.incoming_seq_num = 1;
    }

    fn get_utc_timestamp(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let days_since_epoch = now / 86400;
        let year = 1970 + (days_since_epoch * 4) / 1461; 
        let month = ((days_since_epoch % 365) / 30) + 1;
        let day = (days_since_epoch % 30) + 1;
        
        format!("{:04}{:02}{:02}-{:02}:{:02}:{:02}",
            year, month.min(12), day.min(31),
            (now / 3600) % 24,
            (now / 60) % 60,
            now % 60
        )
    }
}