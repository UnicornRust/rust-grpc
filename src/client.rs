use proto:: {
    calculator_service_client::CalculatorServiceClient,
    CalculatorServiceAddRequest,
};
use std::error::Error;

pub mod proto {
    tonic::include_proto!("calculator");
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorServiceClient::connect(url).await?;
    let req = CalculatorServiceAddRequest { a: 4, b: 5};
    let response = client.add(req).await?;
    println!("Response: {:?}", response.get_ref().result);
    Ok(())
}
