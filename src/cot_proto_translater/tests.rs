#[cfg(test)]
mod tests {
    use crate::cot_proto_translater::{protobuf_to_xml, xml_to_protobuf};
    use crate::xml::bullseye::Event;

    #[test]
    fn test_xml_to_protobuf_and_back() {
        let xml = r#"
            <event version="2.0" uid="test-uid" type="a-f-G-E-V-A" time="2025-07-09T00:00:00Z" start="2025-07-09T00:00:00Z" stale="2025-07-09T01:00:00Z" how="m-g">
                <point lat="34.0" lon="-118.0" hae="100.0" ce="10.0" le="10.0"/>
                <detail>
                </detail>
            </event>
        "#;

        let proto_bytes = xml_to_protobuf(xml).unwrap();
        let new_xml = protobuf_to_xml(&proto_bytes).unwrap();

        // We can't do a direct string comparison because the timestamp will be different.
        // Instead, we'll parse the new XML and check the fields.
        let event: Event = quick_xml::de::from_str(&new_xml).unwrap();
        assert_eq!(event.uid, "test-uid");
        assert_eq!(event.type_attr, "a-f-G-E-V-A");
        assert_eq!(event.point.lat, 34.0);
        assert_eq!(event.point.lon, -118.0);
    }
}
