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
pub struct GetMultipleCompressedAccountProofsPost200ResponseResult {
    #[serde(rename = "context")]
    pub context: Box<models::Context>,
    #[serde(rename = "value")]
    pub value: Vec<models::MerkleProofWithContext>,
}

impl GetMultipleCompressedAccountProofsPost200ResponseResult {
    pub fn new(context: models::Context, value: Vec<models::MerkleProofWithContext>) -> GetMultipleCompressedAccountProofsPost200ResponseResult {
        GetMultipleCompressedAccountProofsPost200ResponseResult {
            context: Box::new(context),
            value,
        }
    }
}

