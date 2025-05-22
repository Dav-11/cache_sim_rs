
/// Represents the types of messages that can be sent or received.
/// YOU MUST DEFINE YOUR ACTUAL MESSAGE TYPES HERE.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MessageType {
    Request(u64),      // Example: Request for address u64
    Response(u64, u32), // Example: Response for address u64 with data u32
    Forward(u64),      // Example: Forwarding a request
    Evict(u64),        // Example: Eviction message
    Nop,               // No operation message, for initial setup or testing
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub requestor_id: u64,
    pub data: Vec<u8>,
    pub address: u32
}


