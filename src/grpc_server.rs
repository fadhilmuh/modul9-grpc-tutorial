use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc::{Receiver, Sender};
use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse,
transaction_service_server::{TransactionService, TransactionServiceServer}, TransactionRequest, TransactionResponse};


pub mod services{
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service))
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


#[derive(Default)]
pub struct MyTransactionService{}

#[tonic::async_trait]
impl TransactionService for MyTransactionService{
    type GetTransactionHistoryStream = ReceiverStream<Result<TransactionResponse, Status>>;
    async fn get_transaction_history(
        &self, 
        request : Request<TransactionRequest>, 
    ) ->Result<Response<Self::GetTransactionHistoryStream>, Status>{
        println!("Received transaction history request: {:?}", request);
        let (tx,rx): (Sender<Result<TransactionResponse, Status>>, Receiver<Result<TransactionResponse,Status>>) = mpsc::channel(4);

        tokio::spawn(async move{
            for i in 0..30{
                if tx.send (Ok(TransactionResponse{
                    transaction_id : format!("trans_{}",i),
                    status : "Completed".to_string(),
                    amount:100.0,
                    timestamp:"2022-01-01T12:00:00Z".to_string(),
                })).await.is_err(){break;}
                if i%10==9{
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}