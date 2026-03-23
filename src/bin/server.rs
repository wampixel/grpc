use tonic::transport::Server;

use grpc::greeter::{
    proto::{
        fact_service_server::FactServiceServer,
        say_hello_service_server::SayHelloServiceServer
    },
    v1::say_hello::server::SayHelloImpl
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(
            SayHelloServiceServer::new(SayHelloImpl::default())
        )
        .add_service(
        )
        .serve(addr)
        .await?;

    Ok(())
}