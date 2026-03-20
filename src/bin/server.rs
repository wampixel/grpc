use tonic::transport::Server;

use grpc::server::v1::{
    greeter::proto::say_hello_service_server::SayHelloServiceServer,
    greeter::say_hello::SayHelloImpl
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(
            SayHelloServiceServer::new(SayHelloImpl::default())
        )
        .serve(addr)
        .await?;

    Ok(())
}