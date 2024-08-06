//! Handle invoice creation

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::Phoenixd;

/// Invoice Request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRequest {
    /// Correlation ID
    pub external_id: Option<String>,
    /// Invoice description
    pub description: Option<String>,
    /// description Hash
    pub description_hash: Option<String>,
    /// Invoice Amount in sats
    pub amount_sat: u64,
    /// webhook Url
    pub webhook_url: Option<String>,
}

/// Invoice Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceResponse {
    /// Invoice Amount in sat
    pub amount_sat: u64,
    /// Payment Hash
    pub payment_hash: String,
    /// Bolt11
    pub serialized: String,
}

impl Phoenixd {
    /// Create Invoice
    pub async fn create_invoice(&self, invoice_request: InvoiceRequest) -> Result<InvoiceResponse> {
        let url = self.api_url.join("/createinvoice")?;

        let res = self
            .make_post(url, Some(serde_json::to_value(invoice_request)?))
            .await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(_) => {
                log::error!("Api error response on invoice creation");
                log::error!("{}", res);
                bail!("Could not create invoice")
            }
        }
    }

    /// Find Invoice
    pub async fn find_invoice(&self, payment_hash: &str) -> Result<InvoiceResponse> {
        let url = self.api_url.join("/incoming")?.join(payment_hash)?;

        let res = self.make_get(url).await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(_) => {
                log::error!("Api error response on find invoice");
                log::error!("{}", res);
                bail!("Could not find invoice")
            }
        }
    }
}
