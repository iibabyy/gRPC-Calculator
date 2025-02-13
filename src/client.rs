use std::error::Error;

use proto::{calculator_client::CalculatorClient, CalculationRequest};

pub mod proto {
	tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let url = "http://localhost:8080";
	let mut client = CalculatorClient::connect(url).await?;

	let request = tonic::Request::new(
		CalculationRequest {
			a: 10,
			b: 2,
		}
	);

	let response = client.divide(request).await?;

	eprintln!("response: {:#?}", response.get_ref().result);

	Ok(())
}