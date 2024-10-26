/*
 * photon-indexer
 *
 * Solana indexer for general compression
 *
 * The version of the OpenAPI document: 0.45.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompressedProof {
    #[serde(rename = "a")]
    pub a: Vec<u8>,
    #[serde(rename = "b")]
    pub b: Vec<u8>,
    #[serde(rename = "c")]
    pub c: Vec<u8>,
}

impl CompressedProof {
    pub fn new(a: Vec<u8>, b: Vec<u8>, c: Vec<u8>) -> CompressedProof {
        CompressedProof { a, b, c }
    }
}
