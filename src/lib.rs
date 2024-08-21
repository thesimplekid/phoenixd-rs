//! Phoenix API SDK
//! Rust SDK for <https://phoenix.acinq.co>
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]

use std::str::FromStr;

use anyhow::Result;
use reqwest::{Client, IntoUrl, Url};
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
    /// let client = Phoenixd::new("xxxxxxxxxxx", "https://test.com").unwrap();
    /// ```
    pub fn new(api_password: &str, api_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let api_url = Url::from_str(api_url)?;

        Ok(Self {
            api_password: api_password.to_string(),
            api_url,
            client,
        })
    }

    async fn make_get<U>(&self, url: U) -> Result<Value>
    where
        U: IntoUrl,
    {
        Ok(self
            .client
            .get(url)
            .basic_auth("", Some(&self.api_password))
            .send()
            .await?
            .json::<Value>()
            .await?)
    }

    async fn make_post<U, T>(&self, url: U, data: Option<T>) -> Result<Value>
    where
        U: IntoUrl,
        T: Serialize,
    {
        let value = match data {
            Some(data) => {
                self.client
                    .post(url)
                    .basic_auth("", Some(&self.api_password))
                    .form(&data)
                    .send()
                    .await?
                    .json::<Value>()
                    .await?
            }
            None => {
                self.client
                    .post(url)
                    .basic_auth("", Some(&self.api_password))
                    .send()
                    .await?
                    .json::<Value>()
                    .await?
            }
        };
        Ok(value)
    }
}
