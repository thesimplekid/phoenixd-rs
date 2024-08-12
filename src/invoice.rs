//! Handle invoice creation

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::Phoenixd;

/// Invoice Request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRequest {
    /// Correlation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Invoice description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// description Hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_hash: Option<String>,
    /// Invoice Amount in sats
    pub amount_sat: u64,
    /// webhook Url
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Find Invoice Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindInvoiceResponse {
    /// Payment Hash
    pub payment_hash: String,
    /// Preimage
    pub preimage: String,
    /// External Id
    pub external_id: Option<String>,
    /// Description
    pub description: String,
    /// Bolt11 invoice
    pub invoice: String,
    /// Paid flag
    pub is_paid: bool,
    /// Sats received
    pub received_sat: u64,
    /// Fees
    pub fees: u64,
    /// Completed at
    pub completed_at: Option<u64>,
    /// Time created
    pub created_at: u64,
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
    pub async fn find_invoice(&self, payment_hash: &str) -> Result<FindInvoiceResponse> {
        let url = self
            .api_url
            .join(&format!("payments/incoimg/{}", payment_hash))?;

        let res = self.make_get(url).await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(err) => {
                log::error!("Api error response on find invoice");
                log::error!("{}", err);
                log::error!("{}", res);
                bail!("Could not find invoice")
            }
        }
    }
}
