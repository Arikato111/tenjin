use super::{
    events::{Action, FeaturesReqEvent, Payload},
    ofp_header::OfpHeader,
    HelloEvent, Msg, OfpMsgEvent, OpenflowHeader, PacketOutEvent,
};

pub struct Openflow13 {}

impl Openflow13 {
    pub fn new() -> Self {
        Openflow13 {}
    }
}

impl Default for Openflow13 {
    fn default() -> Self {
        Self::new()
    }
}

impl OfpMsgEvent for Openflow13 {
    fn header_parse(&self, bytes: &Vec<u8>) -> Result<OfpHeader, std::io::Error> {
        OfpHeader::parse(bytes)
    }
    fn header_size(&self) -> usize {
        8
    }
    fn hello_event(&self) -> HelloEvent {
        HelloEvent::new()
    }

    fn fetures_req(&self) -> FeaturesReqEvent {
        FeaturesReqEvent::new()
    }
    fn packet_out(
        &self,
        port_id: Option<u32>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent {
        PacketOutEvent::new(port_id, payload, actions)
    }
    fn ofp_version() -> usize {
        0x04
    }
    fn version(&self) -> usize {
        Self::ofp_version()
    }

    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader {
        OfpHeader::new(message, length as usize, xid as usize)
    }

    fn msg_parse(&self, msg: u8) -> Msg {
        Msg::from(msg)
    }

    fn msg_usize(&self, msg: Msg) -> usize {
        msg.to_int() as usize
    }
}
