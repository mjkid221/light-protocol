/*
 * photon-indexer
 *
 * Solana indexer for general compression
 *
 * The version of the OpenAPI document: 0.15.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignatureInfoList {
    #[serde(rename = "items")]
    pub items: Vec<models::SignatureInfo>,
}

impl SignatureInfoList {
    pub fn new(items: Vec<models::SignatureInfo>) -> SignatureInfoList {
        SignatureInfoList { items }
    }
}
