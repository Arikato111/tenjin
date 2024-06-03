use crate::Controller;
use std::{io::Read, net::TcpListener, thread};

use super::controller_frame::ControllerFrame;
use crate::openflow::ofp10::{traiter::OfpMsgEvent, HelloEvent};

pub fn tcp_listener_handler<OME: OfpMsgEvent>(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Ok(addr) = stream.local_addr() {
                    println!("server has connection from {}", addr);
                }

                thread::spawn(move || {
                    let mut ctrl = Controller::new();
                    ctrl.send_msg(HelloEvent::new(), 0, &mut stream);
                    let ofp_size = ctrl.get_ofp().header_size();
                    // let ofp = controller.lock().unwrap().get_ofp();
                    let mut buffer = vec![0u8; ofp_size];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(v) if v > 0 => {
                                ctrl.request_handler(&mut buffer, &mut stream);
                            }
                            Ok(_) | Err(_) => {
                                println!("cannot read packet");
                                break;
                            }
                        }
                    }
                });
            }
            Err(_) => panic!("Connection failed!"),
        }
    }
}
