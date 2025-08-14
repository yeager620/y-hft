use crate::fix::error::{FixError, SessionError};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

pub struct FixConnection {
    stream: TcpStream,
}

impl FixConnection {
    pub async fn connect(address: &str) -> Result<Self, FixError> {
        let stream = TcpStream::connect(address).await
            .map_err(|e| SessionError::InvalidSessionState)?;
            
        Ok(Self { stream })
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<(), FixError> {
        self.stream.write_all(data).await
            .map_err(|_| SessionError::InvalidSessionState)?;
        Ok(())
    }

    pub async fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, FixError> {
        let bytes_read = self.stream.read(buffer).await
            .map_err(|_| SessionError::InvalidSessionState)?;
        Ok(bytes_read)
    }

    pub async fn close(&mut self) -> Result<(), FixError> {
        self.stream.shutdown().await
            .map_err(|_| SessionError::InvalidSessionState)?;
        Ok(())
    }
}