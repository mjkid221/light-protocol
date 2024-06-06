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
pub struct TokenBalance {
    #[serde(rename = "balance")]
    pub balance: i32,
    /// A Solana public key represented as a base58 string.
    #[serde(rename = "mint")]
    pub mint: String,
}

impl TokenBalance {
    pub fn new(balance: i32, mint: String) -> TokenBalance {
        TokenBalance {
            balance,
            mint,
        }
    }
}

