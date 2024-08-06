use std::env;

use dotenvy::dotenv;
use phoenixd_rs::{InvoiceRequest, Phoenixd};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let strike = Phoenixd::new(
        &env::var("API_KEY").expect("API key not set"),
        &env::var("API_URL").expect("Apit url needs to be set"),
    )
    .unwrap();

    let invoice_request = InvoiceRequest {
        external_id: None,
        description: Some("some invoice".to_string()),
        webhook_url: None,
        amount_sat: 100,
        description_hash: None,
    };

    let create_invoice = strike.create_invoice(invoice_request).await.unwrap();
    println!("{:?}", create_invoice);

    let invoice = strike
        .find_invoice(&create_invoice.payment_hash.clone())
        .await
        .unwrap();
    println!("{:?}", invoice);
}
