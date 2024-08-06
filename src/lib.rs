//! Phoenix API SDK
//! Rust SDK for <https://phoenix.acinq.co>
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]

use std::str::FromStr;

use anyhow::Result;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod invoice;
pub mod pay_ln;
pub mod webhooks;

pub use invoice::*;
pub use pay_ln::*;

/// Phoenixd
#[derive(Debug, Clone)]
pub struct Phoenixd {
    api_password: String,
    api_url: Url,
    client: Client,
    webhook_url: Option<Url>,
}

/// Invoice state
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvoiceState {
    /// Payment Completed
    Completed,
    /// Invoice paid
    Paid,
    /// Invoice unpaid
    Unpaid,
    /// Invoice pending
    Pending,
}

impl Phoenixd {
    /// Create Strike client
    /// # Arguments
    /// * `api_password` - Phoenixd api password
    /// * `url` - Optional Url of nodeless api
    ///
    /// # Example
    /// ```
    /// use phoenixd_rs::Phoenixd;
    /// let client = Phoenixd::new("xxxxxxxxxxx", "https://test.com", None).unwrap();
    /// ```
    pub fn new(api_password: &str, api_url: &str, webhook_url: Option<String>) -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let api_url = Url::from_str(api_url)?;

        let webhook_url = match webhook_url {
            Some(url) => Some(Url::from_str(&url)?),
            None => None,
        };

        Ok(Self {
            api_password: api_password.to_string(),
            api_url,
            client,
            webhook_url,
        })
    }

    async fn make_get(&self, url: Url) -> Result<Value> {
        Ok(self
            .client
            .get(url)
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
            .basic_auth("", Some(&self.api_password))
            .send()
            .await?
            .json::<Value>()
            .await?)
    }

    async fn make_post<T: Serialize>(&self, url: Url, data: Option<T>) -> Result<Value> {
        let value = match data {
            Some(data) => {
                self.client
                    .post(url)
                    .basic_auth("", Some(&self.api_password))
                    .header("Content-Type", "application/json")
                    .header("accept", "application/json")
                    .json(&data)
                    .send()
                    .await?
                    .json::<Value>()
                    .await?
            }
            None => {
                self.client
                    .post(url)
                    .basic_auth("", Some(&self.api_password))
                    .header("accept", "application/json")
                    .send()
                    .await?
                    .json::<Value>()
                    .await?
            }
        };
        Ok(value)
    }
}
