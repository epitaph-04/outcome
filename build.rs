fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/infrastructure/proto_client")  // you can change the generated code's location
        .compile(
            &["proto/planbuild/v2/planbuild.proto"],
            &["proto"], // specify the root location to search proto dependencies
        )?;
    Ok(())
}