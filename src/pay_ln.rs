//! Pay Ln

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::Phoenixd;

/// Pay Invoice Request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceRequest {
    /// Amount in sats
    pub amount_sats: Option<u64>,
    /// Bolt11 Invoice
    pub invoice: String,
}

/// Pay Invoice Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceResponse {
    /// Amount recipient was payed
    pub recipient_amount_sat: u64,
    /// Routing fee paid
    pub routing_fee_sat: u64,
    /// Payment Id
    pub payment_id: String,
    /// Payment hash
    pub payment_hash: String,
    /// Payment preimage
    pub payment_preimage: String,
}

impl Phoenixd {
    /// PayInvoice
    pub async fn pay_bolt11_invoice(
        &self,
        invoice: &str,
        amount_sats: Option<u64>,
    ) -> Result<PayInvoiceResponse> {
        let url = self.api_url.join("/payinvoice")?;

        let request = PayInvoiceRequest {
            amount_sats,
            invoice: invoice.to_string(),
        };

        let res = self.make_post(url, Some(request)).await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(_) => {
                log::error!("Api error response on payment quote execution");
                log::error!("{}", res);
                bail!("Could not execute payment quote")
            }
        }
    }
}
