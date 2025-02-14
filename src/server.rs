use std::error::Error;

use proto::{admin_server::{Admin, AdminServer}, calculator_server::{Calculator, CalculatorServer}, CalculationResponse, CounterResponse};
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};


mod proto {
    tonic::include_proto!("calculator");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("calculator_descriptor");
}

// counter state
type State = std::sync::Arc<tokio::sync::RwLock<u32>>;

#[derive(Debug, Default)]
struct CalculatorService {
    state: State,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        eprintln!("Request Count: {}", *count);
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        eprintln!("Request incoming: {request:#?}");
        self.increment_counter().await;

        let input = request.get_ref();

        let response = proto::CalculationResponse {
            result: input.a + input.b,
        };

        Ok(
            tonic::Response::new(response)
        )
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        eprintln!("Request incoming: {request:#?}");
        self.increment_counter().await;

        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("cannot divide by zero"))
        }

        let response = proto::CalculationResponse {
            result: input.a / input.b,
        };

        Ok(
            tonic::Response::new(response)
        )
    }
}


#[derive(Debug, Default)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        request: tonic::Request<proto::GetCountRequest>,
    ) -> Result<tonic::Response<CounterResponse>, tonic::Status> {
        let count = self.state.read().await;

        Ok(tonic::Response::new(CounterResponse {
            count: *count,
        }))
    }
}

fn check_auth(
    req: Request<()>
) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    match req.metadata().get("Authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(tonic::Status::unauthenticated("No valid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080".parse()?;

    let state = State::default();

    let calc = CalculatorService{ state: state.clone() };
    let adm = AdminService{ state: state.clone() };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .add_service(AdminServer::with_interceptor(adm, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}
