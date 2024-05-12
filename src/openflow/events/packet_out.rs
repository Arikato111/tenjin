use std::io::{BufRead, Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::openflow::{OfpPort, PseudoPort};

use super::{flow_mod::SizeCheck, FlowAction, Payload};

pub struct PacketOutEvent {
    pub payload: Payload,
    pub port_id: Option<u16>,
    pub actions: Vec<FlowAction>,
}

impl PacketOutEvent {
    pub fn parse(buf: &Vec<u8>) -> Self {
        let mut bytes = Cursor::new(buf);
        let buf_id = match bytes
            .read_i32::<BigEndian>()
            .expect("cannot parse buf id in packetout")
        {
            -1 => None,
            n => Some(n),
        };
        let in_port = bytes.read_u16::<BigEndian>().unwrap();
        let action_len = bytes.read_u16::<BigEndian>().unwrap();
        let mut actions_buf = vec![0; action_len as usize];
        let _ = bytes.read_exact(&mut actions_buf);
        let mut action_bytes = Cursor::new(actions_buf);
        let actions = FlowAction::parse_sequence(&mut action_bytes);
        Self {
            payload: match buf_id {
                None => Payload::NoBuffered(bytes.fill_buf().unwrap().to_vec()),
                Some(n) => {
                    Payload::Buffered(n as u32, bytes.fill_buf().unwrap().to_ascii_lowercase())
                }
            },
            port_id: {
                if in_port == OfpPort::None as u16 {
                    None
                } else {
                    Some(in_port)
                }
            },
            actions,
        }
    }

    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_i32::<BigEndian>(match self.payload {
            Payload::Buffered(n, _) => n as i32,
            Payload::NoBuffered(_) => -1,
        });
        match self.port_id {
            Some(id) => {
                PseudoPort::PhysicalPort(id).marshal(bytes);
            }
            None => {
                let _ = bytes.write_u16::<BigEndian>(OfpPort::None as u16);
            }
        }
        let _ = bytes.write_u16::<BigEndian>(self.actions.size_of_sequence() as u16);
        for act in self.actions.move_controller_last() {
            act.marshal(bytes);
        }
        self.payload.marshal(bytes);
    }
}
