use prac::first_controller_server::{FirstController, FirstControllerServer};
use prac::{HelloRequest, HelloResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod prac {
    tonic::include_proto!("prac");
}

#[derive(Debug, Default)]
pub struct ImplFirstController {}

#[tonic::async_trait]
impl FirstController for ImplFirstController {
    async fn hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let message = req.into_inner().message;
        println!("Got a request {:?}", &message);

        Ok(Response::new(HelloResponse {
            message: format!("Got a request: {:?}", &message),
        }))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let first_controller = ImplFirstController::default();

    println!("LISTENING ON {:?}", addr);
    Server::builder()
        .add_service(FirstControllerServer::new(first_controller))
        .serve(addr)
        .await?;

    Ok(())
}
