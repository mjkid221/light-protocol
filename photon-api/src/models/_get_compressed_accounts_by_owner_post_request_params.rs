/*
 * photon-indexer
 *
 * Solana indexer for general compression
 *
 * The version of the OpenAPI document: 0.23.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCompressedAccountsByOwnerPostRequestParams {
    /// A 32-byte hash represented as a base58 string.
    #[serde(
        rename = "cursor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cursor: Option<Option<String>>,
    #[serde(
        rename = "limit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit: Option<Option<i32>>,
    /// A Solana public key represented as a base58 string.
    #[serde(rename = "owner")]
    pub owner: String,
}

impl GetCompressedAccountsByOwnerPostRequestParams {
    pub fn new(owner: String) -> GetCompressedAccountsByOwnerPostRequestParams {
        GetCompressedAccountsByOwnerPostRequestParams {
            cursor: None,
            limit: None,
            owner,
        }
    }
}
