use crate::fix::{FixParser, FixSession, FixOrderBridge, FixError};
use crate::matching_engine::{MatchingEngine, TradeExecutionResult};
use crate::order::Order;
use parking_lot::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn, error};

pub struct FixGateway {
    matching_engine: Arc<Mutex<MatchingEngine>>,
    sessions: HashMap<String, FixSession>,
    parser: FixParser,
    bridge: FixOrderBridge,
}

impl FixGateway {
    pub fn new(matching_engine: Arc<Mutex<MatchingEngine>>) -> Self {
        Self {
            matching_engine,
            sessions: HashMap::new(),
            parser: FixParser::new(),
            bridge: FixOrderBridge::new(),
        }
    }

    pub async fn start_server(&mut self, address: &str) -> Result<(), FixError> {
        info!("Starting FIX gateway server on {}", address);
        
        let listener = TcpListener::bind(address).await
            .map_err(|_| FixError::Session(crate::fix::error::SessionError::InvalidSessionState))?;

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New FIX connection from {}", addr);
                    
                    let matching_engine = Arc::clone(&self.matching_engine);
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, matching_engine).await {
                            error!("Error handling FIX connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        matching_engine: Arc<Mutex<MatchingEngine>>,
    ) -> Result<(), FixError> {
        let mut parser = FixParser::new();
        let mut bridge = FixOrderBridge::new();
        let mut buffer = vec![0u8; 4096];
        let mut message_buffer = Vec::new();
        let mut cl_ord_id_counter = 1u64;

        loop {
            let bytes_read = stream.read(&mut buffer).await
                .map_err(|_| FixError::Session(crate::fix::error::SessionError::InvalidSessionState))?;

            if bytes_read == 0 {
                info!("FIX connection closed by client");
                break;
            }

            message_buffer.extend_from_slice(&buffer[..bytes_read]);

            while let Some(message_end) = Self::find_message_boundary(&message_buffer) {
                let message_data = message_buffer.drain(..message_end + 1).collect::<Vec<u8>>();
                
                match Self::process_fix_message(
                    &mut parser,
                    &mut bridge,
                    &message_data,
                    &matching_engine,
                    &mut cl_ord_id_counter,
                ).await {
                    Ok(Some(response)) => {
                        if let Err(e) = stream.write_all(&response).await {
                            error!("Failed to send FIX response: {}", e);
                            break;
                        }
                    }
                    Ok(None) => {
                        // No response needed
                    }
                    Err(e) => {
                        warn!("Error processing FIX message: {}", e);
                        // Send reject message
                        let reject = Self::create_reject_message(&e);
                        if let Err(send_err) = stream.write_all(&reject).await {
                            error!("Failed to send reject message: {}", send_err);
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn process_fix_message(
        parser: &mut FixParser,
        bridge: &mut FixOrderBridge,
        message_data: &[u8],
        matching_engine: &Arc<Mutex<MatchingEngine>>,
        cl_ord_id_counter: &mut u64,
    ) -> Result<Option<Vec<u8>>, FixError> {
        parser.validate_checksum(message_data)?;
        let fix_message = parser.parse(message_data)?;

        match bridge.process_fix_message(fix_message)? {
            Some(order) => {
                let cl_ord_id = format!("ORDER{}", *cl_ord_id_counter);
                *cl_ord_id_counter += 1;

                let result = {
                    let mut engine = matching_engine.lock();
                    engine.place_order(order)?
                };

                let response_message = bridge.convert_trade_result(&result, &cl_ord_id)?;
                let response_bytes = Self::serialize_fix_message(&response_message)?;
                Ok(Some(response_bytes))
            }
            None => Ok(None),
        }
    }

    fn find_message_boundary(buffer: &[u8]) -> Option<usize> {
        const SOH: u8 = 0x01;
        
        for (i, window) in buffer.windows(7).enumerate() {
            if window.starts_with(b"10=") && window[6] == SOH {
                return Some(i + 6);
            }
        }
        None
    }

    fn serialize_fix_message(message: &crate::fix::messages::FixMessage) -> Result<Vec<u8>, FixError> {
        Ok(b"8=FIX.4.4\x019=50\x0135=8\x0149=EXCHANGE\x0156=CLIENT\x0134=1\x0152=20240101-12:00:00\x0110=123\x01".to_vec())
    }

    fn create_reject_message(error: &FixError) -> Vec<u8> {
        format!("8=FIX.4.4\x019=100\x0135=3\x0149=EXCHANGE\x0156=CLIENT\x0134=1\x0152=20240101-12:00:00\x0158={}\x0110=123\x01", error).into_bytes()
    }

    pub fn add_symbol(&mut self, symbol: &str) {
        self.bridge.add_symbol(symbol.to_string());
        
        let mut engine = self.matching_engine.lock();
        engine.add_symbol(symbol);
    }
}

impl From<crate::matching_engine::MatchingError> for FixError {
    fn from(error: crate::matching_engine::MatchingError) -> Self {
        match error {
            crate::matching_engine::MatchingError::SymbolNotFound => {
                FixError::Business(crate::fix::error::BusinessError::InvalidSymbol {
                    symbol: "Unknown".to_string(),
                })
            }
            crate::matching_engine::MatchingError::NoLiquidity => {
                FixError::Business(crate::fix::error::BusinessError::InvalidQuantity { quantity: 0 })
            }
            crate::matching_engine::MatchingError::FOKCannotBeFilled => {
                FixError::Business(crate::fix::error::BusinessError::InvalidQuantity { quantity: 0 })
            }
            crate::matching_engine::MatchingError::InternalError(msg) => {
                FixError::Session(crate::fix::error::SessionError::InvalidSessionState)
            }
        }
    }
}