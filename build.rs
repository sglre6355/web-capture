fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/web_capture/v1/web_capture.proto")?;
    Ok(())
}
