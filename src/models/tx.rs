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
pub struct Tx {
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl Tx {
    pub fn new(to: String, data: String, value: String) -> Tx {
        Tx {
            to,
            data,
            value,
        }
    }
}

