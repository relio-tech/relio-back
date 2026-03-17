fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic-prost-build 0.14 — configure() moved from tonic-build to tonic-prost-build
    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(
            &["proto/ai_service.proto", "proto/scan_service.proto", "proto/export_service.proto"],
            &["proto/"],
        )?;
    Ok(())
}
