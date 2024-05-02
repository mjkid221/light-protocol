use anchor_lang::{solana_program::pubkey::Pubkey, Result};
use light_utils::hash_to_bn254_field_size_be;
use std::collections::HashMap;

use crate::{errors::CompressedPdaError, NewAddressParams, NewAddressParamsPacked};
pub fn derive_address(merkle_tree_pubkey: &Pubkey, seed: &[u8; 32]) -> Result<[u8; 32]> {
    let hash = match hash_to_bn254_field_size_be(
        [merkle_tree_pubkey.to_bytes(), *seed].concat().as_slice(),
    ) {
        Some(hash) => Ok::<[u8; 32], CompressedPdaError>(hash.0),
        None => return Err(CompressedPdaError::DeriveAddressError.into()),
    }?;

    Ok(hash)
}

// helper function to pack new address params for instruction data in rust clients
pub fn pack_new_address_params(
    new_address_params: &[NewAddressParams],
    remaining_accounts: &mut HashMap<Pubkey, usize>,
) -> Vec<NewAddressParamsPacked> {
    let mut new_address_params_packed = new_address_params
        .iter()
        .map(|x| NewAddressParamsPacked {
            seed: x.seed,
            address_merkle_tree_root_index: x.address_merkle_tree_root_index,
            address_merkle_tree_account_index: 0, // will be assigned later
            address_queue_account_index: 0,       // will be assigned later
        })
        .collect::<Vec<NewAddressParamsPacked>>();
    let len: usize = remaining_accounts.len();
    for (i, params) in new_address_params.iter().enumerate() {
        match remaining_accounts.get(&params.address_merkle_tree_pubkey) {
            Some(_) => {}
            None => {
                remaining_accounts.insert(params.address_merkle_tree_pubkey, i + len);
            }
        };
        new_address_params_packed[i].address_merkle_tree_account_index = *remaining_accounts
            .get(&params.address_merkle_tree_pubkey)
            .unwrap()
            as u8;
    }

    let len: usize = remaining_accounts.len();
    for (i, params) in new_address_params.iter().enumerate() {
        match remaining_accounts.get(&params.address_queue_pubkey) {
            Some(_) => {}
            None => {
                remaining_accounts.insert(params.address_queue_pubkey, i + len);
            }
        };
        new_address_params_packed[i].address_queue_account_index = *remaining_accounts
            .get(&params.address_queue_pubkey)
            .unwrap() as u8;
    }
    new_address_params_packed
}
