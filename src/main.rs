use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    first_name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn function_handler(
    event: LambdaEvent<Request>
) -> Result<Response, Error> {
    let name = event.payload.first_name;
    let response = Response {
        message: format!(
            "Hello, {}",
            if name == "" { "Serverless World" } else { &name }
        )
    };
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
