use std::net::{Ipv4Addr, UdpSocket};
use converter::{convert, ConversionType};

fn main() -> std::io::Result<()> {
    let multicast_addr = "224.0.0.1";
    let port = 8080;
    let interface = "0.0.0.0";

    let socket = UdpSocket::bind((interface, port))?;
    let multicast_ip: Ipv4Addr = multicast_addr.parse().expect("Invalid multicast address");
    let local_ip: Ipv4Addr = interface.parse().expect("Invalid local interface");

    socket.join_multicast_v4(&multicast_ip, &local_ip)?;
    println!("Listening for multicast and unicast traffic on {}:{}", multicast_addr, port);

    let mut buf = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let data = &buf[..size];
                println!("Received {} bytes from {}", size, src);

                // Simple check to guess the content type
                let is_xml = data.starts_with(b"<?xml");

                let conversion_type = if is_xml {
                    ConversionType::XmlToProtobuf
                } else {
                    ConversionType::ProtobufToXml
                };

                match convert(data, conversion_type) {
                    Ok(converted_data) => {
                        println!("Conversion successful. Converted data: {:?}", converted_data);
                        // Optionally, send the converted data back or handle it as needed
                    }
                    Err(e) => {
                        eprintln!("Conversion failed: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}
