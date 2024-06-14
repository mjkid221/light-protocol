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
pub struct GetCompressionSignaturesForAddressPostRequestParams {
    /// A Solana public key represented as a base58 string.
    #[serde(rename = "address")]
    pub address: String,
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
}

impl GetCompressionSignaturesForAddressPostRequestParams {
    pub fn new(address: String) -> GetCompressionSignaturesForAddressPostRequestParams {
        GetCompressionSignaturesForAddressPostRequestParams {
            address,
            cursor: None,
            limit: None,
        }
    }
}
