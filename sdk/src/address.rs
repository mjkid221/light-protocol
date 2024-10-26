use anchor_lang::{solana_program::pubkey::Pubkey, AnchorDeserialize, AnchorSerialize};
use light_utils::{hash_to_bn254_field_size_be, hashv_to_bn254_field_size_be};

use crate::merkle_context::{AddressMerkleContext, RemainingAccounts};

#[derive(Debug, PartialEq, Default, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct NewAddressParams {
    pub seed: [u8; 32],
    pub address_queue_pubkey: Pubkey,
    pub address_merkle_tree_pubkey: Pubkey,
    pub address_merkle_tree_root_index: u16,
}

#[derive(Debug, PartialEq, Default, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct NewAddressParamsPacked {
    pub seed: [u8; 32],
    pub address_queue_account_index: u8,
    pub address_merkle_tree_account_index: u8,
    pub address_merkle_tree_root_index: u16,
}

pub struct AddressWithMerkleContext {
    pub address: [u8; 32],
    pub address_merkle_context: AddressMerkleContext,
}

pub fn pack_new_addresses_params(
    addresses_params: &[NewAddressParams],
    remaining_accounts: &mut RemainingAccounts,
) -> Vec<NewAddressParamsPacked> {
    addresses_params
        .iter()
        .map(|x| {
            let address_queue_account_index =
                remaining_accounts.insert_or_get(x.address_queue_pubkey);
            let address_merkle_tree_account_index =
                remaining_accounts.insert_or_get(x.address_merkle_tree_pubkey);
            NewAddressParamsPacked {
                seed: x.seed,
                address_queue_account_index,
                address_merkle_tree_account_index,
                address_merkle_tree_root_index: x.address_merkle_tree_root_index,
            }
        })
        .collect::<Vec<_>>()
}

pub fn pack_new_address_params(
    address_params: NewAddressParams,
    remaining_accounts: &mut RemainingAccounts,
) -> NewAddressParamsPacked {
    pack_new_addresses_params(&[address_params], remaining_accounts)[0]
}

/// Derives a single address seed for a compressed account, based on the
/// provided multiple `seeds`, `program_id` and `merkle_tree_pubkey`.
///
/// # Examples
///
/// ```ignore
/// use light_sdk::{address::derive_address, pubkey};
///
/// let address = derive_address(
///     &[b"my_compressed_account"],
///     &crate::ID,
/// );
/// ```
pub fn derive_address_seed(seeds: &[&[u8]], program_id: &Pubkey) -> [u8; 32] {
    let mut inputs = Vec::with_capacity(seeds.len() + 1);

    let program_id = program_id.to_bytes();
    inputs.push(program_id.as_slice());

    inputs.extend(seeds);

    let address = hashv_to_bn254_field_size_be(inputs.as_slice());
    address
}

/// Derives an address for a compressed account, based on the provided singular
/// `seed` and `address_merkle_context`:
pub fn derive_address(
    address_seed: &[u8; 32],
    address_merkle_context: &AddressMerkleContext,
) -> [u8; 32] {
    let merkle_tree_pubkey = address_merkle_context.address_merkle_tree_pubkey.to_bytes();
    let input = [merkle_tree_pubkey, *address_seed].concat();

    // PANICS: Not being able to find the bump for truncating the hash is
    // practically impossible. Quite frankly, we should just remove that error
    // inside.
    hash_to_bn254_field_size_be(input.as_slice()).unwrap().0
}

#[cfg(test)]
mod test {
    use light_macros::pubkey;

    use super::*;

    #[test]
    fn test_derive_address_seed() {
        let program_id = pubkey!("7yucc7fL3JGbyMwg4neUaenNSdySS39hbAk89Ao3t1Hz");

        let address_seed = derive_address_seed(&[b"foo", b"bar"], &program_id);
        assert_eq!(
            address_seed,
            [
                0, 246, 150, 3, 192, 95, 53, 123, 56, 139, 206, 179, 253, 133, 115, 103, 120, 155,
                251, 72, 250, 47, 117, 217, 118, 59, 174, 207, 49, 101, 201, 110
            ]
        );

        let address_seed = derive_address_seed(&[b"ayy", b"lmao"], &program_id);
        assert_eq!(
            address_seed,
            [
                0, 202, 44, 25, 221, 74, 144, 92, 69, 168, 38, 19, 206, 208, 29, 162, 53, 27, 120,
                214, 152, 116, 15, 107, 212, 168, 33, 121, 187, 10, 76, 233
            ]
        );
    }

    #[test]
    fn test_derive_address() {
        let address_merkle_context = AddressMerkleContext {
            address_merkle_tree_pubkey: pubkey!("11111111111111111111111111111111"),
            address_queue_pubkey: pubkey!("22222222222222222222222222222222222222222222"),
        };
        let program_id = pubkey!("7yucc7fL3JGbyMwg4neUaenNSdySS39hbAk89Ao3t1Hz");

        let address_seed = derive_address_seed(&[b"foo", b"bar"], &program_id);
        let address = derive_address(&address_seed, &address_merkle_context);
        let expected_address = pubkey!("139uhyyBtEh4e1CBDJ68ooK5nCeWoncZf9HPyAfRrukA");
        assert_eq!(address, expected_address.to_bytes());

        let address_seed = derive_address_seed(&[b"ayy", b"lmao"], &program_id);
        let address = derive_address(&address_seed, &address_merkle_context);
        let expected_address = pubkey!("12bhHm6PQjbNmEn3Yu1Gq9k7XwVn2rZpzYokmLwbFazN");
        assert_eq!(address, expected_address.to_bytes());
    }
}
