/*
 * deBridge Liquidity Network (DLN) API
 *
 * A turnkey solution for a high-performance cross-chain trading<br><br><a href=\"https://docs.debridge.finance/dln-the-debridge-liquidity-network-protocol/interacting-with-the-api/quick-start-guide\">Quick Start Guide</a>
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "approximateFulfillmentDelay")]
    pub approximate_fulfillment_delay: f64,
    #[serde(rename = "salt", skip_serializing_if = "Option::is_none")]
    pub salt: Option<f64>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl Order {
    pub fn new(approximate_fulfillment_delay: f64) -> Order {
        Order {
            approximate_fulfillment_delay,
            salt: None,
            metadata: None,
        }
    }
}

