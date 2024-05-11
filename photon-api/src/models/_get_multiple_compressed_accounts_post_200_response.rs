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
pub struct GetMultipleCompressedAccountsPost200Response {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::GetCompressedAccountPost200ResponseError>>,
    /// An ID to identify the response.
    #[serde(rename = "id")]
    pub id: Id,
    /// The version of the JSON-RPC protocol.
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: Jsonrpc,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::GetMultipleCompressedAccountsPost200ResponseResult>>,
}

impl GetMultipleCompressedAccountsPost200Response {
    pub fn new(id: Id, jsonrpc: Jsonrpc) -> GetMultipleCompressedAccountsPost200Response {
        GetMultipleCompressedAccountsPost200Response {
            error: None,
            id,
            jsonrpc,
            result: None,
        }
    }
}
/// An ID to identify the response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Id {
    #[serde(rename = "test-account")]
    TestAccount,
}

impl Default for Id {
    fn default() -> Id {
        Self::TestAccount
    }
}
/// The version of the JSON-RPC protocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Jsonrpc {
    #[serde(rename = "2.0")]
    Variant2Period0,
}

impl Default for Jsonrpc {
    fn default() -> Jsonrpc {
        Self::Variant2Period0
    }
}
