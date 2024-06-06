/*
 * photon-indexer
 *
 * Solana indexer for general compression
 *
 * The version of the OpenAPI document: 0.19.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCompressedTokenAccountsByDelegatePost200ResponseResult {
    #[serde(rename = "context")]
    pub context: Box<models::Context>,
    #[serde(rename = "value")]
    pub value: Box<models::TokenAccountList>,
}

impl GetCompressedTokenAccountsByDelegatePost200ResponseResult {
    pub fn new(context: models::Context, value: models::TokenAccountList) -> GetCompressedTokenAccountsByDelegatePost200ResponseResult {
        GetCompressedTokenAccountsByDelegatePost200ResponseResult {
            context: Box::new(context),
            value: Box::new(value),
        }
    }
}

