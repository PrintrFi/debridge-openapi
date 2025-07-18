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
pub struct DlnOrderStatusResponse {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "status")]
    pub status: Status,
}

impl DlnOrderStatusResponse {
    pub fn new(order_id: String, status: Status) -> DlnOrderStatusResponse {
        DlnOrderStatusResponse {
            order_id,
            status,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Fulfilled")]
    Fulfilled,
    #[serde(rename = "SentUnlock")]
    SentUnlock,
    #[serde(rename = "OrderCancelled")]
    OrderCancelled,
    #[serde(rename = "SentOrderCancel")]
    SentOrderCancel,
    #[serde(rename = "ClaimedUnlock")]
    ClaimedUnlock,
    #[serde(rename = "ClaimedOrderCancel")]
    ClaimedOrderCancel,
}

impl Default for Status {
    fn default() -> Status {
        Self::None
    }
}

