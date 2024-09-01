//! Phoenixd Webhooks

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Phoenixd;

/// Webhook state
#[derive(Debug, Clone)]
pub struct WebhookState {
    /// Sender
    pub sender: tokio::sync::mpsc::Sender<WebhookResponse>,
}

/// Webhook data
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookResponse {
    /// Webhook response type
    #[serde(rename = "type")] pub _type: String,
    /// Amount received
    pub amount_sat: u64,
    /// Payment Hash
    pub payment_hash: String,
    /// External id if one was provided when invoice created
    pub external_id: Option<String>,
}

/*
For Get Info
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    pub state: String,
    pub channel_id: String,
    pub balance_sat: u64,
    pub inbound_liquidity_sat: u64,
    pub capacity_sat: u64,
    pub funding_tx_id: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub node_id: String,
    pub channels: Vec<Channel>,
}

*/

impl Phoenixd {
    /// Create router for webhook
    pub async fn create_invoice_webhook_router(
        &self,
        webhook_endpoint: &str,
        sender: tokio::sync::mpsc::Sender<WebhookResponse>,
    ) -> anyhow::Result<Router> {
        let state = WebhookState { sender };

        let router = Router::new()
            .route(webhook_endpoint, post(handle_invoice))
            .with_state(state);

        Ok(router)
    }
}

async fn handle_invoice(
    State(state): State<WebhookState>,
    Json(payload): Json<Value>,
) -> Result<StatusCode, StatusCode> {
    let webhook_response: WebhookResponse = serde_json::from_value(payload).map_err(|_err| {
        log::warn!("Got an invalid payload on webhook");

        StatusCode::UNPROCESSABLE_ENTITY
    })?;

    log::debug!(
        "Received webhook update for: {}",
        webhook_response.payment_hash
    );

    if let Err(err) = state.sender.send(webhook_response).await {
        log::warn!("Could not send on channel: {}", err);
    }
    Ok(StatusCode::OK)
}
