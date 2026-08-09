use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct IncomingPacket {
    pub from: Option<String>,
    pub payload: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct OutgoingPacket<'a> {
    pub sender_id: &'a str,
    pub target_id: &'a str,
    pub payload: &'a str,
}
