use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .compile_protos(&[
        "src/takproto/contact.proto", 
        "src/takproto/cotevent.proto",
        "src/takproto/detail.proto",
        "src/takproto/group.proto",
        "src/takproto/precisionlocation.proto",
        "src/takproto/status.proto",
        "src/takproto/takcontrol.proto",
        "src/takproto/takmessage.proto",
        "src/takproto/takv.proto",
        "src/takproto/track.proto"], 
        &["src/takproto/"])?;
    Ok(())
}