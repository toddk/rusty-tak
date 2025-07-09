use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@uid")]
    pub uid: String,
    #[serde(rename = "@type")]
    pub type_attr: String,
    #[serde(rename = "@time")]
    pub time: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@stale")]
    pub stale: String,
    #[serde(rename = "@how")]
    pub how: String,
    pub point: Point,
    pub detail: Detail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    #[serde(rename = "@lat")]
    pub lat: f64,
    #[serde(rename = "@lon")]
    pub lon: f64,
    #[serde(rename = "@hae")]
    pub hae: f64,
    #[serde(rename = "@ce")]
    pub ce: f64,
    #[serde(rename = "@le")]
    pub le: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    #[serde(rename = "$value", default)]
    pub items: Vec<DetailItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DetailItem {
    Archive,
    Bullseye(Bullseye),
    Contact(Contact),
    Remarks(Remarks),
    Precisionlocation(Precisionlocation),
    Usericon(Usericon),
    Color(Color),
    Link(Link),
    Status(Status),
    TakControl(TakControl),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakControl {
    #[serde(rename = "TakProtocolSupport")]
    pub tak_protocol_support: Option<TakProtocolSupport>,
    #[serde(rename = "TakRequest")]
    pub tak_request: Option<TakRequest>,
    #[serde(rename = "TakResponse")]
    pub tak_response: Option<TakResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakProtocolSupport {
    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakRequest {
    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakResponse {
    #[serde(rename = "@status")]
    pub status: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bullseye {
    #[serde(rename = "@mils")]
    pub mils: bool,
    #[serde(rename = "@distance")]
    pub distance: f64,
    #[serde(rename = "@bearingRef")]
    pub bearing_ref: BearingRef,
    #[serde(rename = "@bullseyeUID")]
    pub bullseye_uid: String,
    #[serde(rename = "@distanceUnits")]
    pub distance_units: String,
    #[serde(rename = "@edgeToCenter")]
    pub edge_to_center: bool,
    #[serde(rename = "@rangeRingVisible")]
    pub range_ring_visible: bool,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@hasRangeRings")]
    pub has_range_rings: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BearingRef {
    T,
    M,
    G,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    #[serde(rename = "@callsign")]
    pub callsign: String,
    #[serde(rename = "@emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "@endpoint")]
    pub endpoint: Option<String>,
    #[serde(rename = "@phone")]
    pub phone: Option<i64>,
    #[serde(rename = "@xmppUsername")]
    pub xmpp_username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Remarks {
    #[serde(rename = "@source")]
    pub source: Option<String>,
    #[serde(rename = "@sourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "@time")]
    pub time: Option<String>,
    #[serde(rename = "@to")]
    pub to: Option<String>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Precisionlocation {
    #[serde(rename = "@geopointsrc")]
    pub geopointsrc: Option<String>,
    #[serde(rename = "@altsrc")]
    pub altsrc: String,
    #[serde(rename = "@PRECISE_IMAGE_FILE")]
    pub precise_image_file: Option<String>,
    #[serde(rename = "@PRECISE_IMAGE_FILE_X")]
    pub precise_image_file_x: Option<f64>,
    #[serde(rename = "@PRECISE_IMAGE_FILE_Y")]
    pub precise_image_file_y: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usericon {
    #[serde(rename = "@iconsetpath")]
    pub iconsetpath: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    #[serde(rename = "@argb")]
    pub argb: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "@battery")]
    pub battery: Option<i64>,
    #[serde(rename = "@readiness")]
    pub readiness: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    #[serde(rename = "@parent_callsign")]
    pub parent_callsign: Option<String>,
    #[serde(rename = "@production_time")]
    pub production_time: Option<String>,
    #[serde(rename = "@relation")]
    pub relation: String,
    #[serde(rename = "@type")]
    pub type_attr: String,
    #[serde(rename = "@uid")]
    pub uid: String,
    #[serde(rename = "@callsign")]
    pub callsign: Option<String>,
    #[serde(rename = "@remarks")]
    pub remarks: Option<String>,
    pub point: LinkPoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkPoint {
    #[serde(rename = "$text")]
    pub value: String,
}