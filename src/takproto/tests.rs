
// tests.rs

use super::takmessage as proto;
use prost::Message;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    #[serde(rename = "@lat")]
    lat: f64,
    #[serde(rename = "@lon")]
    lon: f64,
    #[serde(rename = "@hae")]
    hae: f64,
    #[serde(rename = "@ce")]
    ce: f64,
    #[serde(rename = "@le")]
    le: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "event")]
struct Event {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@type")]
    event_type: String,
    #[serde(rename = "@uid")]
    uid: String,
    #[serde(rename = "@time")]
    time: String,
    #[serde(rename = "@start")]
    start: String,
    #[serde(rename = "@stale")]
    stale: String,
    #[serde(rename = "@how")]
    how: String,
    point: Point,
}

#[test]
fn test_protobuf_roundtrip() {
    let original_cot_event = proto::CotEvent {
        r#type: "a-f-G-U-C".to_string(),
        access: "".to_string(),
        qos: "".to_string(),
        opex: "".to_string(),
        uid: "UID-1234".to_string(),
        send_time: 1672531200000,
        start_time: 1672531200000,
        stale_time: 1672531200000,
        how: "m-g".to_string(),
        lat: 34.0522,
        lon: -118.2437,
        hae: 100.0,
        ce: 10.0,
        le: 5.0,
        detail: None,
    };

    let mut buf = Vec::new();
    original_cot_event.encode(&mut buf).unwrap();

    let decoded_cot_event = proto::CotEvent::decode(&*buf).unwrap();

    assert_eq!(original_cot_event, decoded_cot_event);
}

#[test]
fn test_xml_to_protobuf_and_back() {
    let xml_input = r#"
        <event version="2.0" type="a-f-G-U-C" uid="UID-5678" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T00:00:00Z" how="m-g">
            <point lat="34.0522" lon="-118.2437" hae="100.0" ce="10.0" le="5.0"/>
        </event>
    "#;

    let event_from_xml: Event = from_str(xml_input).unwrap();

    let cot_event_from_xml = proto::CotEvent {
        r#type: event_from_xml.event_type.clone(),
        access: "".to_string(),
        qos: "".to_string(),
        opex: "".to_string(),
        uid: event_from_xml.uid.clone(),
        send_time: 1672531200000, // Corresponds to 2023-01-01T00:00:00Z
        start_time: 1672531200000,
        stale_time: 1672531200000,
        how: event_from_xml.how.clone(),
        lat: event_from_xml.point.lat,
        lon: event_from_xml.point.lon,
        hae: event_from_xml.point.hae,
        ce: event_from_xml.point.ce,
        le: event_from_xml.point.le,
        detail: None,
    };

    let event_for_xml = Event {
        version: "2.0".to_string(),
        event_type: cot_event_from_xml.r#type.clone(),
        uid: cot_event_from_xml.uid.clone(),
        time: "2023-01-01T00:00:00Z".to_string(),
        start: "2023-01-01T00:00:00Z".to_string(),
        stale: "2023-01-01T00:00:00Z".to_string(),
        how: cot_event_from_xml.how.clone(),
        point: Point {
            lat: cot_event_from_xml.lat,
            lon: cot_event_from_xml.lon,
            hae: cot_event_from_xml.hae,
            ce: cot_event_from_xml.ce,
            le: cot_event_from_xml.le,
        },
    };

    assert_eq!(event_from_xml, event_for_xml);
}
