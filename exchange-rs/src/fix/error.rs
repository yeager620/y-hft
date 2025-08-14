use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FixError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("Session error: {0}")]
    Session(#[from] SessionError),
    
    #[error("Business error: {0}")]
    Business(#[from] BusinessError),
}

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("Invalid message format")]
    InvalidFormat,
    
    #[error("Invalid checksum: expected {expected}, got {actual}")]
    InvalidChecksum { expected: u8, actual: u8 },
    
    #[error("Invalid body length: expected {expected}, got {actual}")]
    InvalidBodyLength { expected: usize, actual: usize },
    
    #[error("Missing SOH delimiter")]
    MissingSoh,
    
    #[error("Invalid tag format: {tag}")]
    InvalidTag { tag: String },
    
    #[error("Invalid field value for tag {tag}: {value}")]
    InvalidFieldValue { tag: u32, value: String },
}

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Missing required field: {tag}")]
    MissingRequiredField { tag: u32 },
    
    #[error("Invalid message type: {msg_type}")]
    InvalidMessageType { msg_type: String },
    
    #[error("Field {tag} not allowed in message type {msg_type}")]
    FieldNotAllowed { tag: u32, msg_type: String },
    
    #[error("Invalid field length for tag {tag}: {length}")]
    InvalidFieldLength { tag: u32, length: usize },
    
    #[error("Invalid field value for tag {tag}: {value}")]
    InvalidFieldValue { tag: u32, value: String },
}

#[derive(Error, Debug, Clone)]
pub enum SessionError {
    #[error("Invalid sequence number: expected {expected}, got {actual}")]
    InvalidSequenceNumber { expected: u32, actual: u32 },
    
    #[error("Session not logged in")]
    NotLoggedIn,
    
    #[error("Heartbeat timeout")]
    HeartbeatTimeout,
    
    #[error("Invalid session state")]
    InvalidSessionState,
    
    #[error("Duplicate session")]
    DuplicateSession,
}

#[derive(Error, Debug, Clone)]
pub enum BusinessError {
    #[error("Invalid symbol: {symbol}")]
    InvalidSymbol { symbol: String },
    
    #[error("Invalid quantity: {quantity}")]
    InvalidQuantity { quantity: u32 },
    
    #[error("Invalid price: {price}")]
    InvalidPrice { price: u64 },
    
    #[error("Duplicate ClOrdID: {cl_ord_id}")]
    DuplicateClOrdId { cl_ord_id: String },
    
    #[error("Order not found: {cl_ord_id}")]
    OrderNotFound { cl_ord_id: String },
}