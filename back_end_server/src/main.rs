use tonic::{transport::Server, Request, Response, Status};

use hello_v1::hello_service_server::{HelloService, HelloServiceServer};
use hello_v1::{SayHelloRequest, SayHelloResponse};

#[derive(Default)]
struct Hello;

pub mod hello_v1 {
    include!("../../proto/output/hello.v1.rs");
}

#[tonic::async_trait]
impl HelloService for Hello {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Incoming request: {:?}", request);

        let reply = SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:9090".parse().unwrap();
    let hello = Hello::default();

    println!("Hello gRPC listening on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(hello))
        .serve(addr)
        .await?;

    Ok(())
}
