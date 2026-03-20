use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_service_server::{GreeterService, GreeterServiceServer};
use hello_world::{SayHelloResponse, SayHelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct HWGreeter {}

#[tonic::async_trait]
impl GreeterService for HWGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SayHelloResponse {
            response: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = HWGreeter::default();

    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}