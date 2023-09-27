fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/infrastructure/grpc_client")  // you can change the generated code's location
        .compile(
            &["proto/planbuild/v2/planbuild.proto"],
            &["proto"], // specify the root location to search proto dependencies
        )?;

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/infrastructure/grpc_client")  // you can change the generated code's location
        .compile(
            &["proto/planoutcome/v3/plan.proto"],
            &["proto"], // specify the root location to search proto dependencies
        )?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/infrastructure/grpc_client")  // you can change the generated code's location
        .compile(
            &["proto/plan_outcome_predicting/v1/plan_outcome.proto"],
            &["proto"], // specify the root location to search proto dependencies
        )?;

    Ok(())
}