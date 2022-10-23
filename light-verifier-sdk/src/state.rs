use anchor_lang::prelude::*;
use crate::light_transaction::{Config, Transaction};
use std::marker::PhantomData;
use borsh::{BorshSerialize, BorshDeserialize};

/// Verifier state is a boiler plate struct which should be versatile enough to serve many use cases.
/// For specialized use cases with less
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct VerifierStateTenNF<T: Config> {
    pub signer: Pubkey,
    pub nullifiers: Vec<Vec<u8>>,
    pub leaves: Vec<Vec<u8>>,
    pub public_amount: [u8; 32],
    pub fee_amount: [u8; 32],
    pub mint_pubkey: [u8;32],
    pub merkle_root: [u8; 32],
    pub tx_integrity_hash: [u8; 32],
    pub relayer_fee: u64,
    pub encrypted_utxos: Vec<u8>,
    pub merkle_root_index: u64,
    pub e_phantom: PhantomData<T>
}

impl <T: Config>VerifierStateTenNF<T> {
    pub const LEN: usize = 2048;
}

impl <T: Config>anchor_lang::AccountDeserialize for VerifierStateTenNF<T> {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        match VerifierStateTenNF::deserialize(buf) {
            Ok(v) => Ok(v),
            Err(_) => err!(anchor_lang::error::ErrorCode::AccountDidNotDeserialize),
        }
    }
}

impl <T: Config>anchor_lang::AccountSerialize for VerifierStateTenNF<T> {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        self.serialize(writer).unwrap();
        match self.serialize(writer) {
            Ok(_) => Ok(()),
            Err(_) => err!(anchor_lang::error::ErrorCode::AccountDidNotSerialize),
        }
    }
}

impl <T: Config>anchor_lang::Owner for VerifierStateTenNF<T> {
    fn owner() -> Pubkey {
        Pubkey::new(&T::ID[..])
    }
}

impl <T: Config>From<Transaction<'_, '_, '_, T>> for VerifierStateTenNF<T> {
    fn from(light_tx: Transaction<'_, '_, '_, T>) -> VerifierStateTenNF<T> {
        let mut nullifiers = [[0u8;32]; 10];
        for (i, nf) in light_tx.nullifiers.iter().enumerate() {
                nullifiers[i] = nf.clone().try_into().unwrap();
        }
        let mut leaves = vec![vec![vec![0u8;32]; 2]; 1];
        for (i, leaf) in light_tx.leaves.iter().enumerate() {
                leaves[i] = vec![leaf[0].clone().try_into().unwrap(), leaf[1].clone().try_into().unwrap()];
        }

        VerifierStateTenNF {
            merkle_root_index: <usize as TryInto::<u64>>::try_into(light_tx.merkle_root_index).unwrap(),
            signer: Pubkey::new(&[0u8;32]),
            nullifiers: light_tx.nullifiers,
            leaves: leaves[0].clone(), // vec![vec![0u8;32]; 2], //
            public_amount: light_tx.public_amount.try_into().unwrap(),
            fee_amount: light_tx.fee_amount.try_into().unwrap(),
            mint_pubkey: light_tx.mint_pubkey.try_into().unwrap(),
            relayer_fee: light_tx.relayer_fee.try_into().unwrap(),
            encrypted_utxos: [light_tx.encrypted_utxos.clone(), vec![0u8; 256 - light_tx.encrypted_utxos.len()]].concat().try_into().unwrap(),
            merkle_root: light_tx.merkle_root.try_into().unwrap(),
            tx_integrity_hash: light_tx.tx_integrity_hash.try_into().unwrap(),
            e_phantom: PhantomData,
        }
    }
}

#[test]
fn test_into() {

    // dbg!(person);


}
