pub mod services {
    tonic::include_proto!("services");
}

use services::{
    chat_service_client::ChatServiceClient, payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient, ChatMessage, PaymentRequest,
    TransactionRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(PaymentRequest {
        amount: 100.0,
        user_id: "user_123".to_string(),
    });

    let response = client.process_payment(request).await?;
    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
