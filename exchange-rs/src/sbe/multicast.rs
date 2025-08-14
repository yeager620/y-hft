use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

use socket2::{Domain, Protocol, Socket, Type};
use tokio::sync::mpsc;
use tokio::time::timeout;
use thiserror::Error;
use tracing::{debug, error, info, warn};
use bytes::BytesMut;

use crate::sbe::parser::{SbeMessageParser, SbeMessage, SbeParseError};
use crate::sbe::bridge::{SbeBridge, MarketDataUpdate, BridgeError};

#[derive(Error, Debug)]
pub enum MulticastError {
    #[error("Socket creation error: {0}")]
    SocketError(#[from] std::io::Error),
    #[error("SBE parsing error: {0}")]
    SbeError(#[from] SbeParseError),
    #[error("Bridge error: {0}")]
    BridgeError(#[from] BridgeError),
    #[error("Channel send error")]
    ChannelSend,
    #[error("Timeout waiting for data")]
    Timeout,
    #[error("Invalid multicast address: {0}")]
    InvalidAddress(String),
}

#[derive(Debug, Clone)]
pub struct MulticastConfig {
    pub multicast_addr: IpAddr,
    pub port: u16,
    pub interface_addr: Option<IpAddr>,
    pub buffer_size: usize,
    pub read_timeout: Duration,
    pub enable_loopback: bool,
    pub ttl: u32,
}

impl Default for MulticastConfig {
    fn default() -> Self {
        Self {
            multicast_addr: IpAddr::V4(Ipv4Addr::new(224, 0, 1, 1)), 
            port: 8080,
            interface_addr: None,
            buffer_size: 65536,
            read_timeout: Duration::from_millis(100),
            enable_loopback: false,
            ttl: 1,
        }
    }
}

pub struct DeribitMulticastReceiver {
    config: MulticastConfig,
    parser: SbeMessageParser,
    bridge: Arc<SbeBridge>,
    socket: Option<UdpSocket>,
}

impl DeribitMulticastReceiver {
    pub fn new(config: MulticastConfig, bridge: Arc<SbeBridge>) -> Self {
        Self {
            config,
            parser: SbeMessageParser::new(),
            bridge,
            socket: None,
        }
    }

    pub fn start(&mut self) -> Result<mpsc::Receiver<MarketDataUpdate>, MulticastError> {
        info!("Starting Deribit multicast receiver on {}:{}", 
              self.config.multicast_addr, self.config.port);

        self.setup_socket()?;
        
        let (tx, rx) = mpsc::channel(10000); 
        
        let socket = self.socket.take().unwrap();
        let parser = self.parser.clone();
        let bridge = Arc::clone(&self.bridge);
        let config = self.config.clone();

        tokio::spawn(async move {
            Self::receive_loop(socket, parser, bridge, tx, config).await;
        });

        Ok(rx)
    }

    fn setup_socket(&mut self) -> Result<(), MulticastError> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        
        socket.set_reuse_address(true)?;
        #[cfg(any(target_os = "linux", target_os = "android"))]
        socket.set_reuse_port(true)?;
        socket.set_read_timeout(Some(self.config.read_timeout))?;
        socket.set_multicast_loop_v4(self.config.enable_loopback)?;
        socket.set_multicast_ttl_v4(self.config.ttl)?;

        let bind_addr = SocketAddr::new(
            self.config.interface_addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
            self.config.port
        );
        socket.bind(&bind_addr.into())?;

        match self.config.multicast_addr {
            IpAddr::V4(multicast_v4) => {
                let interface = match self.config.interface_addr {
                    Some(IpAddr::V4(addr)) => addr,
                    _ => Ipv4Addr::UNSPECIFIED,
                };
                socket.join_multicast_v4(&multicast_v4, &interface)?;
            }
            IpAddr::V6(_) => {
                return Err(MulticastError::InvalidAddress(
                    "IPv6 multicast not implemented yet".to_string()
                ));
            }
        }

        self.socket = Some(socket.into());
        
        info!("Socket configured for multicast group {}:{}", 
              self.config.multicast_addr, self.config.port);
        
        Ok(())
    }

    async fn receive_loop(
        socket: UdpSocket, 
        parser: SbeMessageParser,
        bridge: Arc<SbeBridge>,
        tx: mpsc::Sender<MarketDataUpdate>,
        config: MulticastConfig
    ) {
        let mut buffer = vec![0u8; config.buffer_size];
        let mut stats_counter = 0u64;
        let mut error_counter = 0u64;
        
        info!("Starting multicast receive loop");

        loop {
            match socket.recv(&mut buffer) {
                Ok(bytes_received) => {
                    if bytes_received == 0 {
                        continue;
                    }

                    stats_counter += 1;

                    if stats_counter % 10000 == 0 {
                        info!("Processed {} messages, {} errors", stats_counter, error_counter);
                    }

                    let message_data = &buffer[..bytes_received];
                    
                    match Self::process_message(&parser, &bridge, message_data).await {
                        Ok(updates) => {
                            for update in updates {
                                if let Err(_) = tx.try_send(update) {
                                    warn!("Market data channel full, dropping update");
                                }
                            }
                        }
                        Err(e) => {
                            error_counter += 1;
                            debug!("Error processing message: {:?}", e);
                            
                            if error_counter % 1000 == 0 {
                                warn!("Total processing errors: {}", error_counter);
                            }
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    error!("Socket receive error: {:?}", e);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    async fn process_message(
        parser: &SbeMessageParser,
        bridge: &SbeBridge,
        data: &[u8]
    ) -> Result<Vec<MarketDataUpdate>, MulticastError> {
        let message = parser.parse_message(data)?;
        
        debug!("Received message: {}", message);

        let updates = bridge.process_message(message)?;
        
        Ok(updates)
    }

    pub fn create_deribit_config() -> MulticastConfig {
        MulticastConfig {
            multicast_addr: IpAddr::V4(Ipv4Addr::new(239, 1, 2, 3)), 
            port: 9999, 
            interface_addr: None,
            buffer_size: 65536,
            read_timeout: Duration::from_millis(50),
            enable_loopback: false,
            ttl: 1,
        }
    }
}

pub struct MulticastManager {
    receivers: Vec<DeribitMulticastReceiver>,
    bridge: Arc<SbeBridge>,
}

impl MulticastManager {
    pub fn new(bridge: Arc<SbeBridge>) -> Self {
        Self {
            receivers: Vec::new(),
            bridge,
        }
    }

    pub fn add_receiver(&mut self, config: MulticastConfig) {
        let receiver = DeribitMulticastReceiver::new(config, Arc::clone(&self.bridge));
        self.receivers.push(receiver);
    }

    pub async fn start_all(&mut self) -> Result<Vec<mpsc::Receiver<MarketDataUpdate>>, MulticastError> {
        let mut channels = Vec::new();
        
        for receiver in &mut self.receivers {
            let rx = receiver.start()?;
            channels.push(rx);
        }
        
        info!("Started {} multicast receivers", channels.len());
        Ok(channels)
    }
}

pub mod deribit {
    use super::*;

    pub fn btc_perpetual_config() -> MulticastConfig {
        MulticastConfig {
            multicast_addr: IpAddr::V4(Ipv4Addr::new(239, 1, 2, 10)),
            port: 10001,
            ..Default::default()
        }
    }

    pub fn eth_perpetual_config() -> MulticastConfig {
        MulticastConfig {
            multicast_addr: IpAddr::V4(Ipv4Addr::new(239, 1, 2, 11)),
            port: 10002,
            ..Default::default()
        }
    }

    pub fn options_config() -> MulticastConfig {
        MulticastConfig {
            multicast_addr: IpAddr::V4(Ipv4Addr::new(239, 1, 2, 20)),
            port: 10010,
            ..Default::default()
        }
    }

    pub fn all_instruments_config() -> Vec<MulticastConfig> {
        vec![
            btc_perpetual_config(),
            eth_perpetual_config(),
            options_config(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_multicast_receiver_creation() {
        let bridge = Arc::new(SbeBridge::default());
        let config = MulticastConfig::default();
        
        let receiver = DeribitMulticastReceiver::new(config, bridge);
        
        assert!(receiver.socket.is_none());
    }

    #[tokio::test]
    async fn test_multicast_manager() {
        let bridge = Arc::new(SbeBridge::default());
        let mut manager = MulticastManager::new(bridge);
        
        manager.add_receiver(MulticastConfig::default());
        
        assert_eq!(manager.receivers.len(), 1);
    }
}

