use tonic::{Request, Response, Status};

use super::proto::{
    say_hello_service_server::SayHelloService,
    SayHelloRequest,
    SayHelloResponse
};

#[derive(Default)]
pub struct SayHelloImpl {}

#[tonic::async_trait]
impl SayHelloService for SayHelloImpl{
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SayHelloResponse {
            response: format!("Hello {} !", request.into_inner().name)
        };

        Ok(Response::new(reply))
    }
}