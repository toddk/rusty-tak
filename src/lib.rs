use std::net::{Ipv4Addr, UdpSocket};
use prost::encoding::decode_varint;
use crate::xml::bullseye::{Detail, DetailItem, Event, Point, TakControl, TakProtocolSupport, TakResponse};
use chrono::Utc;
use wasm_bindgen::prelude::*;

// Include the generated Rust code
pub mod takproto {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/takproto/atakmap.commoncommo.protobuf.v1.rs"));
}

use takproto::TakMessage;
use prost::Message; // Import the Message trait for decoding

mod xml;

enum Marshalling {
    Xml,
    Protobuf(String),
}

#[wasm_bindgen]
pub fn start_server_wasm(multicast_addr: String, port: u16) -> Result<(), JsValue> {
    // Convert String to &str for existing function
    start_server(&multicast_addr, port)
        .map_err(|e| JsValue::from_str(&format!("Failed to start server: {}", e)))
}

pub fn start_server(multicast_addr: &str, port: u16) -> std::io::Result<()> {
    let interface = "0.0.0.0"; // Listen on all interfaces

    let socket = UdpSocket::bind((interface, port))?;
    let multicast_ip: Ipv4Addr = multicast_addr.parse().expect("Invalid multicast address");
    let local_ip: Ipv4Addr = interface.parse().expect("Invalid local interface");

    // Join the multicast group
    socket.join_multicast_v4(&multicast_ip, &local_ip)?;
    println!("Listening for multicast traffic on {}:{}", multicast_addr, port);

    let mut buf = [0u8; 1024];
    let mut marshalling = Marshalling::Xml;

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let data = &buf[..size];
                println!("Received {} bytes from {}", size, src);

                match marshalling {
                    Marshalling::Xml => {
                        let xml = String::from_utf8_lossy(data);
                        if let Ok(event) = quick_xml::de::from_str::<Event>(&xml) {
                            if let Some(detail_item) = event.detail.items.iter().find(|item| match item {
                                DetailItem::TakControl(tc) => tc.tak_request.is_some(),
                                _ => false,
                            }) {
                                if let DetailItem::TakControl(tc) = detail_item {
                                    if let Some(req) = &tc.tak_request {
                                        if req.version == "1" {
                                            println!("Client requested protobuf protocol version 1");
                                            let response = Event {
                                                version: "2.0".to_string(),
                                                uid: "protouid".to_string(),
                                                type_attr: "t-x-takp-r".to_string(),
                                                time: Utc::now().to_rfc3339(),
                                                start: Utc::now().to_rfc3339(),
                                                stale: (Utc::now() + chrono::Duration::minutes(1)).to_rfc3339(),
                                                how: "m-g".to_string(),
                                                point: Point { lat: 0.0, lon: 0.0, hae: 0.0, ce: 999999.0, le: 999999.0 },
                                                detail: Detail {
                                                    items: vec![DetailItem::TakControl(TakControl {
                                                        tak_protocol_support: None,
                                                        tak_request: None,
                                                        tak_response: Some(TakResponse { status: true }),
                                                    })],
                                                },
                                            };
                                            let xml_response = quick_xml::se::to_string(&response).unwrap();
                                            socket.send_to(xml_response.as_bytes(), src)?;
                                            marshalling = Marshalling::Protobuf("1".to_string());
                                            println!("Switched to protobuf protocol version 1");
                                        }
                                    }
                                }
                            }
                        } else {
                            // send protocol support message
                            let support_response = Event {
                                version: "2.0".to_string(),
                                uid: "protouid".to_string(),
                                type_attr: "t-x-takp-v".to_string(),
                                time: Utc::now().to_rfc3339(),
                                start: Utc::now().to_rfc3339(),
                                stale: (Utc::now() + chrono::Duration::minutes(1)).to_rfc3339(),
                                how: "m-g".to_string(),
                                point: Point { lat: 0.0, lon: 0.0, hae: 0.0, ce: 999999.0, le: 999999.0 },
                                detail: Detail {
                                    items: vec![DetailItem::TakControl(TakControl {
                                        tak_protocol_support: Some(TakProtocolSupport { version: "1".to_string() }),
                                        tak_request: None,
                                        tak_response: None,
                                    })],
                                },
                            };
                            let xml_response = quick_xml::se::to_string(&support_response).unwrap();
                            socket.send_to(xml_response.as_bytes(), src)?;
                        }
                    }
                    Marshalling::Protobuf(ref version) => {
                        let magic_byte = data[0];
                        if magic_byte != 0xbf {
                            eprintln!("Invalid magic byte: {:x}", magic_byte);
                            continue;
                        } else {
                            println!("Magic byte is valid");
                        }

                        let offset = 1;
                        let mut data2 = &data[offset..size];
                        let protocol_version = match decode_varint(&mut data2) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Failed to decode protocol version: {}", e);
                                continue;
                            }
                        };

                        if protocol_version.to_string() != *version {
                            eprintln!("Received message with unsupported protocol version: {}", protocol_version);
                            continue;
                        }

                        match TakMessage::decode(&data[3..]) {
                            Ok(deserialized) => {
                                println!("Received from {}: {:?}", src, deserialized);
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize data: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}