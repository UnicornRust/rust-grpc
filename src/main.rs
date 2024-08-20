use proto::{
    calculator_service_server::{CalculatorService, CalculatorServiceServer}, 
    CalculatorServiceAddResponse, 
    CalculatorServiceAddRequest
};

use tonic::{async_trait, transport::Server, Request, Response, Status};
use std::{error::Error, result::Result};


mod proto {
    tonic::include_proto!("calculator");
    // 加载在编译文件中生成的描述文件集
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = 
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct Calculator {}


#[async_trait]
impl CalculatorService for Calculator {
    async fn add(
        &self,
        request: Request<CalculatorServiceAddRequest>,
    ) -> Result<Response<CalculatorServiceAddResponse>, Status> {
        println!("Get a request: {:?}", request);
        let input = request.get_ref();
        let response = CalculatorServiceAddResponse {
            result: input.a + input.b
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let calc = Calculator::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    // 构建服务
    Server::builder()
        .add_service(service)
        .add_service(CalculatorServiceServer::new(calc))
        .serve(addr)
        .await?;
    Ok(())
}
