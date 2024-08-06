use std::env;

use dotenvy::dotenv;
use phoenixd_rs::Phoenixd;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let phoenixd = Phoenixd::new(
        &env::var("API_KEY").expect("API key not set"),
        &env::var("API_URL").expect("Api url not set"),
    )
    .unwrap();

    let invoice = "lnbc100n1pnfhjd8pp5vssdjgseqjfs5av4sqymk7ns0u3ldj2904npwue3na2yr0k379kqdq2f38xy6t5wvcqzzsxqrpcgsp58qn6n6f5pj5leuh28f6gz32kgmyzl987htduzatj69nypmdddlxs9qxpqysgqwv48q7ypza0wryu854h9y0ffude4pu857ksu5wa3dt9kn557tsrhx38lzjaece44gfner9rwhsw5cj2e7pt5ckse84t5865m2gczfdsqvtukva";

    let pay_response = phoenixd.pay_bolt11_invoice(invoice, None).await.unwrap();

    println!("{:?}", pay_response);
}
