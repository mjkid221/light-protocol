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
pub struct AccountData {
    /// A base 64 encoded string.
    #[serde(rename = "data")]
    pub data: String,
    /// A 32-byte hash represented as a base58 string.
    #[serde(rename = "dataHash")]
    pub data_hash: String,
    #[serde(rename = "discriminator")]
    pub discriminator: i32,
}

impl AccountData {
    pub fn new(data: String, data_hash: String, discriminator: i32) -> AccountData {
        AccountData {
            data,
            data_hash,
            discriminator,
        }
    }
}
