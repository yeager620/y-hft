pub mod connection;
pub mod session_state;
pub mod message_store;

pub use connection::FixConnection;
pub use session_state::{FixSessionState, SessionStatus};
pub use message_store::MessageStore;

use crate::fix::error::{FixError, SessionError};
use crate::fix::parser::FixParser;
use crate::fix::messages::{FixMessage, MessageType, Heartbeat, Logon};
use crate::fix::bridge::FixOrderBridge;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn, error};

pub struct FixSession {
    session_state: FixSessionState,
    parser: FixParser,
    bridge: FixOrderBridge,
    connection: Option<FixConnection>,
    message_store: MessageStore,
    last_heartbeat: Instant,
    heartbeat_interval: Duration,
}

impl FixSession {
    pub fn new(sender_comp_id: String, target_comp_id: String) -> Self {
        Self {
            session_state: FixSessionState::new(sender_comp_id, target_comp_id),
            parser: FixParser::new(),
            bridge: FixOrderBridge::new(),
            connection: None,
            message_store: MessageStore::new(),
            last_heartbeat: Instant::now(),
            heartbeat_interval: Duration::from_secs(30),
        }
    }

    pub async fn start(&mut self, address: &str) -> Result<(), FixError> {
        info!("Starting FIX session to {}", address);
        
        let connection = FixConnection::connect(address).await?;
        self.connection = Some(connection);
        
        self.send_logon().await?;
        
        self.session_state.set_status(SessionStatus::LoggedOn);
        info!("FIX session established");
        
        Ok(())
    }

    pub async fn process_incoming_message(&mut self, data: &[u8]) -> Result<Option<FixMessage>, FixError> {
        self.parser.validate_checksum(data)?;
        
        let message = self.parser.parse(data)?;
        
        self.session_state.increment_incoming_seq_num();
        self.last_heartbeat = Instant::now();
        
        match &message {
            FixMessage::Heartbeat(heartbeat) => {
                self.handle_heartbeat(heartbeat).await?;
                Ok(None)
            }
            FixMessage::Logon(logon) => {
                self.handle_logon(logon).await?;
                Ok(None)
            }
            FixMessage::NewOrderSingle(_) => {
                if let Some(order) = self.bridge.process_fix_message(message.clone())? {
                    Ok(Some(message))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(Some(message)),
        }
    }

    pub async fn send_heartbeat(&mut self) -> Result<(), FixError> {
        if self.last_heartbeat.elapsed() >= self.heartbeat_interval {
            let heartbeat = self.create_heartbeat(None)?;
            self.send_message(FixMessage::Heartbeat(heartbeat)).await?;
            self.last_heartbeat = Instant::now();
        }
        Ok(())
    }

    pub async fn check_heartbeat_timeout(&self) -> Result<(), SessionError> {
        let timeout_threshold = self.heartbeat_interval * 2;
        if self.last_heartbeat.elapsed() > timeout_threshold {
            return Err(SessionError::HeartbeatTimeout);
        }
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), FixError> {
        info!("Shutting down FIX session");
        
        if let Some(ref mut connection) = self.connection {
            connection.close().await?;
        }
        
        self.session_state.set_status(SessionStatus::Disconnected);
        Ok(())
    }

    async fn send_logon(&mut self) -> Result<(), FixError> {
        let logon = self.create_logon()?;
        self.send_message(FixMessage::Logon(logon)).await
    }

    async fn handle_heartbeat(&mut self, _heartbeat: &Heartbeat) -> Result<(), FixError> {
        Ok(())
    }

    async fn handle_logon(&mut self, logon: &Logon) -> Result<(), FixError> {
        self.heartbeat_interval = Duration::from_secs(logon.heart_bt_int as u64);
        self.session_state.set_status(SessionStatus::LoggedOn);
        info!("Received logon, heartbeat interval: {}s", logon.heart_bt_int);
        Ok(())
    }

    async fn send_message(&mut self, message: FixMessage) -> Result<(), FixError> {
        let message_bytes = self.serialize_message(&message)?;
        
        if let Some(ref mut connection) = self.connection {
            connection.send(&message_bytes).await?;
            
            self.session_state.increment_outgoing_seq_num();
            self.message_store.store_outgoing_message(&message)?;
        }
        Ok(())
    }

    fn create_heartbeat(&self, test_req_id: Option<String>) -> Result<Heartbeat, FixError> {
        let header = self.session_state.create_header(MessageType::Heartbeat);
        let trailer = crate::fix::messages::Trailer { checksum: 0 };

        Ok(Heartbeat {
            header,
            test_req_id,
            trailer,
        })
    }

    fn create_logon(&self) -> Result<Logon, FixError> {
        let header = self.session_state.create_header(MessageType::Logon);
        let trailer = crate::fix::messages::Trailer { checksum: 0 };

        Ok(Logon {
            header,
            encrypt_method: '0',
            heart_bt_int: self.heartbeat_interval.as_secs() as u32,
            raw_data_length: None,
            raw_data: None,
            reset_seq_num_flag: None,
            next_expected_msg_seq_num: None,
            username: None,
            password: None,
            trailer,
        })
    }

    fn serialize_message(&self, _message: &FixMessage) -> Result<Vec<u8>, FixError> {
        Ok(Vec::new())
    }

    pub fn get_session_status(&self) -> SessionStatus {
        self.session_state.get_status()
    }

    pub fn get_outgoing_seq_num(&self) -> u32 {
        self.session_state.get_outgoing_seq_num()
    }

    pub fn get_incoming_seq_num(&self) -> u32 {
        self.session_state.get_incoming_seq_num()
    }
}