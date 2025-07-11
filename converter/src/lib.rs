use wasm_bindgen::prelude::*;

// Include the generated Rust code
pub mod takproto {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/takproto/atakmap.commoncommo.protobuf.v1.rs"));
}

pub mod xml;
pub mod cot_proto_translater;

pub use cot_proto_translater::{xml_to_protobuf, protobuf_to_xml};

#[wasm_bindgen]
pub enum ConversionType {
    XmlToProtobuf,
    ProtobufToXml,
}

#[wasm_bindgen]
pub fn convert(input: &[u8], conversion_type: ConversionType) -> Result<Vec<u8>, JsValue> {
    match conversion_type {
        ConversionType::XmlToProtobuf => {
            let xml_input = String::from_utf8(input.to_vec())
                .map_err(|e| JsValue::from_str(&format!("Invalid UTF-8 sequence: {}", e)))?;
            xml_to_protobuf(&xml_input)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
        ConversionType::ProtobufToXml => {
            protobuf_to_xml(input)
                .map(|s| s.as_bytes().to_vec())
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
