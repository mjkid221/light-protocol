use std::{fmt, marker::PhantomData, mem, pin::Pin, sync::Arc};

use anchor_lang::{
    solana_program::{pubkey::Pubkey, system_instruction},
    AnchorDeserialize,
};
use light_hash_set::{HashSet, HashSetCell};
use num_bigint::ToBigUint;
use num_traits::{Bounded, CheckedAdd, CheckedSub, Unsigned};
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use tokio::sync::Mutex;

pub mod spl;
pub mod test_env;

pub struct AccountZeroCopy<'a, T> {
    pub account: Pin<Box<Account>>,
    deserialized: *const T,
    _phantom_data: PhantomData<&'a T>,
}

impl<'a, T> AccountZeroCopy<'a, T> {
    pub async fn new(context: &mut ProgramTestContext, address: Pubkey) -> AccountZeroCopy<'a, T> {
        let account = Box::pin(
            context
                .banks_client
                .get_account(address)
                .await
                .unwrap()
                .unwrap(),
        );
        let deserialized = account.data[8..].as_ptr() as *const T;

        Self {
            account,
            deserialized,
            _phantom_data: PhantomData,
        }
    }

    // Safe method to access `deserialized` ensuring the lifetime is respected
    pub fn deserialized(&self) -> &'a T {
        unsafe { &*self.deserialized }
    }
}

pub async fn get_account<T: AnchorDeserialize>(
    context: &mut ProgramTestContext,
    pubkey: Pubkey,
) -> T {
    let account = context
        .banks_client
        .get_account(pubkey)
        .await
        .unwrap()
        .unwrap();
    T::deserialize(&mut &account.data[8..]).unwrap()
}

/// Fetches the given account, then copies and serializes it as a `HashSet`.
///
/// # Safety
///
/// This is highly unsafe. Ensuring that:
///
/// * The correct account is used.
/// * The account has enough space to be treated as a HashSet with specified
///   parameters.
/// * The account data is aligned.
///
/// Is the caller's responsibility.
pub async unsafe fn get_hash_set<I, T>(
    context: &mut ProgramTestContext,
    pubkey: Pubkey,
) -> Arc<Mutex<HashSet<I>>>
where
    I: Bounded
        + CheckedAdd
        + CheckedSub
        + Clone
        + Copy
        + fmt::Display
        + From<u8>
        + PartialEq
        + PartialOrd
        + ToBigUint
        + TryFrom<u64>
        + TryFrom<usize>
        + Unsigned,
    f64: From<I>,
    u64: TryFrom<I>,
    usize: TryFrom<I>,
    <usize as TryFrom<I>>::Error: fmt::Debug,
{
    let mut account = context
        .banks_client
        .get_account(pubkey)
        .await
        .unwrap()
        .unwrap();

    // Check if the data is correct.
    let values_offset = 27464_usize;
    let values = account.data[8 + mem::size_of::<T>()..]
        .as_mut_ptr()
        .add(values_offset) as *mut Option<HashSetCell>;
    for i in 0..50 {
        let value = std::ptr::read(values.add(i));
        println!("VALUE: {value:?}");
    }

    Arc::new(Mutex::new(
        HashSet::from_bytes_copy(&mut account.data[8 + mem::size_of::<T>()..]).unwrap(),
    ))
}

pub async fn airdrop_lamports(
    banks_client: &mut ProgramTestContext,
    destination_pubkey: &Pubkey,
    lamports: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a transfer instruction
    let transfer_instruction =
        system_instruction::transfer(&banks_client.payer.pubkey(), destination_pubkey, lamports);

    // Create and sign a transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&banks_client.payer.pubkey()),
        &vec![&banks_client.payer],
        banks_client.last_blockhash,
    );

    // Send the transaction
    banks_client
        .banks_client
        .process_transaction(transaction)
        .await?;

    Ok(())
}

pub async fn create_and_send_transaction(
    context: &mut ProgramTestContext,
    instruction: &[Instruction],
    payer: &Pubkey,
    signers: &[&Keypair],
) -> Result<Signature, BanksClientError> {
    let transaction = Transaction::new_signed_with_payer(
        instruction,
        Some(payer),
        &signers.to_vec(),
        context.last_blockhash,
    );
    let signature = transaction.signatures[0];
    context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(signature)
}

pub fn create_account_instruction(
    payer: &Pubkey,
    size: usize,
    rent: u64,
    id: &Pubkey,
    keypair: Option<&Keypair>,
) -> Instruction {
    let keypair = match keypair {
        Some(keypair) => keypair.insecure_clone(),
        None => Keypair::new(),
    };
    system_instruction::create_account(payer, &keypair.pubkey(), rent, size as u64, id)
}
