
pub mod tests;

use crate::takproto::{CotEvent, TakMessage};
use crate::xml::bullseye::{Detail, Event, Point};
use prost::Message;
use chrono::{DateTime, Utc};

pub fn xml_to_protobuf(xml: &str) -> Result<Vec<u8>, String> {
    let event: Event = quick_xml::de::from_str(xml)
        .map_err(|e| format!("Failed to parse XML: {}", e))?;

    let cot_event = CotEvent {
        r#type: event.type_attr,
        access: "".to_string(),
        qos: "".to_string(),
        opex: "".to_string(),
        uid: event.uid,
        send_time: Utc::now().timestamp_millis() as u64,
        start_time: DateTime::parse_from_rfc3339(&event.start).unwrap().timestamp_millis() as u64,
        stale_time: DateTime::parse_from_rfc3339(&event.stale).unwrap().timestamp_millis() as u64,
        how: event.how,
        lat: event.point.lat,
        lon: event.point.lon,
        hae: event.point.hae,
        ce: event.point.ce,
        le: event.point.le,
        detail: None,
    };

    let tak_message = TakMessage {
        tak_control: None,
        cot_event: Some(cot_event),
    };

    let mut buf = Vec::new();
    tak_message.encode(&mut buf)
        .map_err(|e| format!("Failed to encode protobuf: {}", e))?;
    Ok(buf)
}

pub fn protobuf_to_xml(bytes: &[u8]) -> Result<String, String> {
    let tak_message = TakMessage::decode(bytes)
        .map_err(|e| format!("Failed to decode protobuf: {}", e))?;

    if let Some(cot_event) = tak_message.cot_event {
        let event = Event {
            version: "2.0".to_string(),
            uid: cot_event.uid,
            type_attr: cot_event.r#type,
            time: Utc::now().to_rfc3339(),
            start: DateTime::from_timestamp_millis(cot_event.start_time as i64).unwrap().to_rfc3339(),
            stale: DateTime::from_timestamp_millis(cot_event.stale_time as i64).unwrap().to_rfc3339(),
            how: cot_event.how,
            point: Point {
                lat: cot_event.lat,
                lon: cot_event.lon,
                hae: cot_event.hae,
                ce: cot_event.ce,
                le: cot_event.le,
            },
            detail: Detail { items: vec![] },
        };

        quick_xml::se::to_string(&event)
            .map_err(|e| format!("Failed to serialize XML: {}", e))
    } else {
        Err("No CotEvent in TakMessage".to_string())
    }
}
