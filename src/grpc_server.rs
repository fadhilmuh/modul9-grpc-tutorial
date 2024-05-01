use tonic::{transport::Server, Request, Response, Status};
use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse,
};


pub mod services{
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Default)]
pub struct MyPaymentService{}


#[tonic::async_trait]
impl PaymentService for MyPaymentService{
    async fn process_payment(
        &self,
        request : Request<PaymentRequest>
    ) -> Result<Response<PaymentResponse>, Status>{
        println!("Received payment request: {:?}", request);

        Ok(Response::new (PaymentResponse{success:true}))
    }
}