#![allow(unused)]
#![allow(unused_variables)]
use std::{collections::HashMap, net::TcpStream};

use crate::{
    etherparser::ether_type::EtherType,
    openflow::{
        controller_frame::ControllerFrame,
        ofp10::{
            self,
            events::{flow_mod::MatchFields, Action},
            ofp_v1_0::Openflow10,
            FlowModEvent, PacketInEvent,
        },
        traiter::OfpMsgEvent,
    },
};
/**
 * Here is Controller you can modify and write the process or more you need.
 * In production please remove allow unused.
 */

pub struct Controller {
    ofp: Openflow10,
    mac_to_port: HashMap<u64, u16>,
}

impl ControllerFrame<Openflow10> for Controller {
    fn get_ofp(&self) -> &impl OfpMsgEvent {
        &self.ofp
    }
    fn new() -> Self {
        Self {
            ofp: Openflow10::new(),
            mac_to_port: HashMap::new(),
        }
    }
    /**
     * Start here for handle packetIn message.
     */
    fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream) {
        println!("reason {:?}", packetin.reason);
        let pkt = packetin.ether_parse();
        self.mac_to_port.insert(pkt.mac_src, packetin.in_port);

        let mac_dst = pkt.mac_des;
        let mac_src = pkt.mac_src;

        if let EtherType::LLDP = pkt.ether_type {
            return;
        }

        let out_port = match self.mac_to_port.get(&mac_dst) {
            Some(p) => ofp10::PseudoPort::PhysicalPort(*p),
            None => ofp10::PseudoPort::Flood,
        };

        let actions = vec![Action::Oputput(out_port.clone())];

        if let ofp10::PseudoPort::PhysicalPort(_) = out_port {
            let mut match_fields = MatchFields::match_all();
            match_fields.in_port = Some(packetin.in_port);
            match_fields.mac_dest = Some(mac_dst);
            match_fields.mac_src = Some(mac_src);
            if let Some(buf_id) = packetin.buf_id {
                println!("found buf id");
                self.add_flow(xid, 1, match_fields, &actions, Some(buf_id as u32), stream);
                return;
            } else {
                println!("not found buf id");
                self.add_flow(xid, 1, match_fields, &actions, None, stream);
            }
        }
        let packet_out = self
            .ofp
            .packet_out(Some(packetin.in_port), packetin.payload, actions);
        self.send_msg(packet_out, xid, stream);
    }
}

impl Controller {
    fn add_flow(
        &self,
        xid: u32,
        priority: u16,
        flow: MatchFields,
        actions: &Vec<Action>,
        buffer_id: Option<u32>,
        stream: &mut TcpStream,
    ) {
        self.send_msg(
            FlowModEvent::add_flow(10, flow, actions.clone(), buffer_id),
            xid,
            stream,
        )
    }
}
