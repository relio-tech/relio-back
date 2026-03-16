fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic::build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(
            &["proto/ai_service.proto", "proto/scan_service.proto", "proto/export_service.proto"],
            &["proto/"],
        )?;
    Ok(())
}
