fn main() -> Result<(), Box<dyn std::errir::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"],
            &["proto"],
        )?;
    Ok(())
}