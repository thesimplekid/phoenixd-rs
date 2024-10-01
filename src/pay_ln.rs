//! Pay Ln

use anyhow::bail;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{Error, Phoenixd};

/// Pay Invoice Request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceRequest {
    /// Amount in sats
    pub amount_sats: Option<u64>,
    /// Bolt11 Invoice
    pub invoice: String,
}

///Pay bolt12 offer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayBolt12Request {
    /// Amount in sats
    pub amount_sats: Option<u64>,
    /// Bolt12 offer
    pub offer: String,
    /// Message
    pub message: String,
}

/// Pay Invoice Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceResponse {
    /// Amount recipient was paid
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

/// Find Outgoing Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOutgoingInvoiceResponse {
    /// Payment Hash
    pub payment_hash: String,
    /// Preimage
    pub preimage: String,
    /// Paid flag
    pub is_paid: bool,
    /// Amount sent
    pub sent: u64,
    /// Fees
    pub fees: u64,
    /// Invoice
    pub invoice: String,
    /// Completed at
    pub completed_at: Option<u64>,
    /// Time created
    pub created_at: u64,
}

impl Phoenixd {
    /// PayInvoice
    pub async fn pay_bolt11_invoice(
        &self,
        invoice: &str,
        amount_sats: Option<u64>,
    ) -> anyhow::Result<PayInvoiceResponse> {
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

    /// Pay offer
    pub async fn pay_bolt12_offer(
        &self,
        offer: String,
        amount_sats: Option<u64>,
        message: String,
    ) -> anyhow::Result<PayInvoiceResponse> {
        let url = self.api_url.join("/payoffer")?;

        let request = PayBolt12Request {
            amount_sats,
            offer,
            message,
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

    /// Find outgoing invoice
    pub async fn get_outgoing_invoice(
        &self,
        payment_hash: &str,
    ) -> Result<GetOutgoingInvoiceResponse, Error> {
        let url = self
            .api_url
            .join(&format!("payments/outgoing/{}", payment_hash))
            .map_err(|_| Error::InvalidUrl)?;

        let res = match self.make_get(url).await {
            Ok(res) => res,
            Err(err) => {
                if let Error::ReqwestError(err) = &err {
                    if err.status().unwrap_or_default() == StatusCode::NOT_FOUND {
                        return Err(Error::NotFound);
                    }
                }
                return Err(err);
            }
        };

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(err) => {
                log::error!("Api error response getting payment quote");
                log::error!("{}", res);

                Err(err.into())
            }
        }
    }
}
