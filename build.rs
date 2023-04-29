fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/project.proto")?;
    tonic_build::compile_protos("proto/project_status.proto")?;
    Ok(())
}
