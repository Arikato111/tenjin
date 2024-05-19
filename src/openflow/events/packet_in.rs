use crate::etherparser::ethernet::EthernetFrame;

use super::Payload;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufRead, Cursor};

pub enum PacketInReason {
    NoMatch,
    Action,
    Unknown,
}

pub struct PacketInEvent {
    pub buf_id: Option<u32>,
    pub total_len: u16,
    pub in_port: u16,
    pub reason: PacketInReason,
    pub table_id: u8,
    pub payload: Payload,
}

impl PacketInEvent {
    pub fn ether_parse(&self) -> EthernetFrame {
        match &self.payload {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => EthernetFrame::parse(&p),
        }
    }
    pub fn parse(payload: &Vec<u8>) -> PacketInEvent {
        let mut bytes = Cursor::new(payload.to_vec());
        let buf_id = match bytes.read_i32::<BigEndian>().unwrap() {
            -1 => None,
            n => Some(n as u32),
        };
        let total_len = bytes.read_u16::<BigEndian>().unwrap();
        let in_port = bytes.read_u16::<BigEndian>().unwrap();
        let reason = match bytes.read_u8().unwrap() {
            1 => PacketInReason::NoMatch,
            2 => PacketInReason::Action,
            _ => PacketInReason::Unknown,
        };
        let table_id = bytes.read_u8().unwrap();
        let packet = bytes.fill_buf().unwrap().to_vec();
        let payload = match buf_id {
            Some(n) => Payload::Buffered(n as u32, packet),
            None => Payload::NoBuffered(packet),
        };
        PacketInEvent {
            buf_id,
            total_len,
            in_port,
            reason,
            table_id,
            payload,
        }
    }
}
