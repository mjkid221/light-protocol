use crate::{
    create_account_instruction, create_and_send_transaction,
    create_and_send_transaction_with_event, get_hash_set,
    test_env::COMPRESSED_TOKEN_PROGRAM_PROGRAM_ID,
    test_indexer::{StateMerkleTreeAccounts, TestIndexer, TokenDataWithContext},
    AccountZeroCopy, TransactionParams,
};
use account_compression::{
    initialize_nullifier_queue::NullifierQueueAccount, StateMerkleTreeAccount,
};
use light_compressed_pda::sdk::{compressed_account::MerkleContext, event::PublicTransactionEvent};
use light_compressed_token::{
    get_cpi_authority_pda, get_token_authority_pda, get_token_pool_pda,
    mint_sdk::create_mint_to_instruction, transfer_sdk::create_transfer_instruction,
    TokenTransferOutputData,
};
use light_hasher::Poseidon;
use num_bigint::BigUint;
use num_traits::FromBytes;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::{
    program_pack::Pack,
    pubkey::{Pubkey, PubkeyError},
    signature::{Keypair, Signer},
};
use spl_token::state::Mint;
// TODO: replace with borsh serialize
use anchor_lang::AnchorSerialize;

pub async fn mint_tokens_helper(
    context: &mut ProgramTestContext,
    test_indexer: &mut TestIndexer,
    merkle_tree_pubkey: &Pubkey,
    mint_authority: &Keypair,
    mint: &Pubkey,
    amounts: Vec<u64>,
    recipients: Vec<Pubkey>,
) {
    let payer_pubkey = mint_authority.pubkey();
    let instruction = create_mint_to_instruction(
        &payer_pubkey,
        &payer_pubkey,
        mint,
        merkle_tree_pubkey,
        amounts,
        recipients,
    );
    let event = create_and_send_transaction_with_event::<PublicTransactionEvent>(
        context,
        &[instruction],
        &payer_pubkey,
        &[&mint_authority],
        None,
    )
    .await
    .unwrap()
    .unwrap();
    test_indexer.add_compressed_accounts_with_token_data(event);
}

pub async fn create_mint(
    context: &mut ProgramTestContext,
    payer: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
    freeze_authority: Option<&Pubkey>,
    mint_keypair: Option<&Keypair>,
) -> Pubkey {
    let keypair = Keypair::new();
    let mint_keypair = match mint_keypair {
        Some(mint_keypair) => mint_keypair,
        None => &keypair,
    };
    let mint_pubkey = (*mint_keypair).pubkey();
    let rent = context.banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(Mint::LEN);

    let account_create_ix = crate::create_account_instruction(
        &payer.pubkey(),
        Mint::LEN,
        mint_rent,
        &COMPRESSED_TOKEN_PROGRAM_PROGRAM_ID,
        Some(mint_keypair),
    );

    let create_mint_ix = spl_token::instruction::initialize_mint2(
        &spl_token::id(),
        &mint_pubkey,
        mint_authority,
        freeze_authority,
        decimals,
    )
    .unwrap();
    create_and_send_transaction(
        context,
        &[account_create_ix, create_mint_ix],
        &payer.pubkey(),
        &[payer],
    )
    .await
    .unwrap();
    mint_pubkey
}

pub async fn assert_create_mint(
    context: &mut ProgramTestContext,
    authority: &Pubkey,
    mint: &Pubkey,
    pool: &Pubkey,
) {
    let mint_account: spl_token::state::Mint = spl_token::state::Mint::unpack(
        &context
            .banks_client
            .get_account(*mint)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    let mint_authority = get_token_authority_pda(authority, mint).0;
    assert_eq!(mint_account.supply, 0);
    assert_eq!(mint_account.decimals, 2);
    assert_eq!(mint_account.mint_authority.unwrap(), mint_authority);
    assert_eq!(mint_account.freeze_authority, None.into());
    assert!(mint_account.is_initialized);
    let mint_account: spl_token::state::Account = spl_token::state::Account::unpack(
        &context
            .banks_client
            .get_account(*pool)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();

    assert_eq!(mint_account.amount, 0);
    assert_eq!(mint_account.delegate, None.into());
    assert_eq!(mint_account.mint, *mint);
    assert_eq!(mint_account.owner, get_cpi_authority_pda().0);
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct MerkleTreeTestSnapShot {
    pub accounts: StateMerkleTreeAccounts,
    pub root: [u8; 32],
    pub next_index: usize,
    pub num_added_accounts: usize,
}

pub async fn assert_merkle_tree_after_tx(
    context: &mut ProgramTestContext,
    snapshots: &[MerkleTreeTestSnapShot],
    test_indexer: &TestIndexer,
) {
    let mut deduped_snapshots = snapshots.to_vec();
    deduped_snapshots.sort();
    deduped_snapshots.dedup();
    for (i, snapshot) in deduped_snapshots.iter().enumerate() {
        let merkle_tree_account =
            AccountZeroCopy::<StateMerkleTreeAccount>::new(context, snapshot.accounts.merkle_tree)
                .await;
        let merkle_tree = merkle_tree_account
            .deserialized()
            .copy_merkle_tree()
            .unwrap();
        if merkle_tree.root().unwrap() == snapshot.root {
            println!("deduped_snapshots: {:?}", deduped_snapshots);
            println!("i: {:?}", i);
            panic!("merkle tree root update failed");
        }
        assert_eq!(
            merkle_tree.next_index(),
            snapshot.next_index + snapshot.num_added_accounts
        );
        let test_indexer_merkle_tree = test_indexer
            .state_merkle_trees
            .iter()
            .find(|x| x.0.merkle_tree == snapshot.accounts.merkle_tree)
            .expect("merkle tree not found in test indexer");
        assert_eq!(
            merkle_tree.root().unwrap(),
            test_indexer_merkle_tree.1.root(),
            "merkle tree root update failed"
        );
    }
}

pub async fn assert_transfer(
    context: &mut ProgramTestContext,
    test_indexer: &TestIndexer,
    out_compressed_accounts: &[TokenTransferOutputData],
    input_compressed_account_hashes: &[[u8; 32]],
    output_merkle_tree_test_snapshots: &[MerkleTreeTestSnapShot],
    input_merkle_tree_test_snapshots: &[MerkleTreeTestSnapShot],
) {
    // let merkle_tree_account =
    //     AccountZeroCopy::<StateMerkleTreeAccount>::new(context, test_indexer.merkle_tree_pubkey)
    //         .await;
    // let merkle_tree = merkle_tree_account
    //     .deserialized()
    //     .copy_merkle_tree()
    //     .unwrap();
    // assert_eq!(
    //     merkle_tree.root_index(),
    //     old_merkle_tree.root_index() + out_compressed_accounts.len()
    // );

    // assert_eq!(
    //     merkle_tree.root().unwrap(),
    //     test_indexer.merkle_tree.root(),
    //     "merkle tree root update failed"
    // );
    // assert_ne!(
    //     old_merkle_tree.root().unwrap(),
    //     merkle_tree.root().unwrap(),
    //     "merkle tree root update failed"
    // );
    // assert_eq!(
    //     merkle_tree.next_index(),
    //     old_merkle_tree.next_index() + out_compressed_accounts.len()
    // );
    // let next_index_old_mt = old_merkle_tree.next_index();
    // assert that all Merkle trees have been updated
    assert_merkle_tree_after_tx(context, output_merkle_tree_test_snapshots, test_indexer).await;
    println!("out_compressed_accounts: {:?}", out_compressed_accounts);
    let mut tree = Pubkey::default();
    let mut index = 0;
    for (i, out_compressed_account) in out_compressed_accounts.iter().enumerate() {
        if output_merkle_tree_test_snapshots[i].accounts.merkle_tree != tree {
            tree = output_merkle_tree_test_snapshots[i].accounts.merkle_tree;
            index = 0;
        } else {
            index += 1;
        }
        let pos = test_indexer
            .token_compressed_accounts
            .iter()
            .position(|x| {
                x.token_data.owner == out_compressed_account.owner
                    && x.token_data.amount == out_compressed_account.amount
            })
            .expect("transfer recipient compressed account not found in mock indexer");
        let transfer_recipient_token_compressed_account =
            test_indexer.token_compressed_accounts[pos].clone();
        assert_eq!(
            transfer_recipient_token_compressed_account
                .token_data
                .amount,
            out_compressed_account.amount
        );
        assert_eq!(
            transfer_recipient_token_compressed_account.token_data.owner,
            out_compressed_account.owner
        );
        assert_eq!(
            transfer_recipient_token_compressed_account
                .token_data
                .delegate,
            None
        );
        assert_eq!(
            transfer_recipient_token_compressed_account
                .token_data
                .is_native,
            None
        );
        assert_eq!(
            transfer_recipient_token_compressed_account
                .token_data
                .delegated_amount,
            0
        );

        let transfer_recipient_compressed_account = transfer_recipient_token_compressed_account
            .compressed_account
            .clone();
        assert_eq!(
            transfer_recipient_compressed_account
                .compressed_account
                .lamports,
            0
        );
        assert!(transfer_recipient_compressed_account
            .compressed_account
            .data
            .is_some());
        let mut data = Vec::new();
        transfer_recipient_token_compressed_account
            .token_data
            .serialize(&mut data)
            .unwrap();
        assert_eq!(
            transfer_recipient_compressed_account
                .compressed_account
                .data
                .as_ref()
                .unwrap()
                .data,
            data
        );
        assert_eq!(
            transfer_recipient_compressed_account
                .compressed_account
                .owner,
            light_compressed_token::ID
        );
        println!(
            "output_merkle_tree_test_snapshots[i] {:?}",
            output_merkle_tree_test_snapshots[i]
        );
        println!(
            "transfer_recipient_compressed_account
        .merkle_context
        .leaf_index: {:?}",
            transfer_recipient_compressed_account
        );
        // assert_eq!(
        //     transfer_recipient_compressed_account
        //         .merkle_context
        //         .leaf_index as usize,
        //     output_merkle_tree_test_snapshots[i].next_index + i
        // );
        if !test_indexer.token_compressed_accounts.iter().any(|x| {
            x.compressed_account.merkle_context.leaf_index as usize
                == output_merkle_tree_test_snapshots[i].next_index + index
        }) {
            println!(
                "token_compressed_accounts {:?}",
                test_indexer.token_compressed_accounts
            );
            println!("snapshot {:?}", output_merkle_tree_test_snapshots[i]);
            println!("index {:?}", index);
            panic!("transfer recipient compressed account not found in mock indexer");
        };
    }
    assert_nullifiers_exist_in_hash_sets(
        context,
        input_merkle_tree_test_snapshots,
        input_compressed_account_hashes,
    )
    .await;
}

pub async fn assert_nullifiers_exist_in_hash_sets(
    context: &mut ProgramTestContext,
    snapshots: &[MerkleTreeTestSnapShot],
    input_compressed_account_hashes: &[[u8; 32]],
) {
    for (i, hash) in input_compressed_account_hashes.iter().enumerate() {
        let nullifier_queue = unsafe {
            get_hash_set::<u16, NullifierQueueAccount>(
                context,
                snapshots[i].accounts.nullifier_queue,
            )
            .await
        };
        assert!(nullifier_queue
            .contains(&BigUint::from_be_bytes(hash.as_slice()), 0)
            .unwrap());
    }
}

pub async fn assert_mint_to<'a>(
    num_mint_to: usize,
    context: &mut ProgramTestContext,
    test_indexer: &'a TestIndexer,
    recipient_keypair: &Keypair,
    mint: Pubkey,
    amount: u64,
    snapshots: &[MerkleTreeTestSnapShot],
) {
    let token_compressed_account_data = test_indexer.token_compressed_accounts[0].token_data;
    assert_eq!(token_compressed_account_data.amount, amount);
    assert_eq!(
        token_compressed_account_data.owner,
        recipient_keypair.pubkey()
    );
    assert_eq!(token_compressed_account_data.mint, mint);
    assert_eq!(token_compressed_account_data.delegate, None);
    assert_eq!(token_compressed_account_data.is_native, None);
    assert_eq!(token_compressed_account_data.delegated_amount, 0);

    assert_merkle_tree_after_tx(context, snapshots, test_indexer).await;
    let mint_account: spl_token::state::Mint = spl_token::state::Mint::unpack(
        &context
            .banks_client
            .get_account(mint)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    assert_eq!(mint_account.supply, amount * num_mint_to as u64);

    let pool = get_token_pool_pda(&mint);
    let pool_account = spl_token::state::Account::unpack(
        &context
            .banks_client
            .get_account(pool)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    assert_eq!(pool_account.amount, amount * num_mint_to as u64);
}

/// Creates an spl token account and initializes it with the given mint and owner.
/// This function is useful to create token accounts for spl compression and decompression tests.
pub async fn create_token_account(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    account_keypair: &Keypair,
    owner: &Keypair,
) -> Result<(), BanksClientError> {
    let rent = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(anchor_spl::token::TokenAccount::LEN);
    let account_create_ix = create_account_instruction(
        &owner.pubkey(),
        anchor_spl::token::TokenAccount::LEN,
        rent,
        &anchor_spl::token::ID,
        Some(account_keypair),
    );
    let instruction = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &account_keypair.pubkey(),
        mint,
        &owner.pubkey(),
    )
    .unwrap();
    create_and_send_transaction(
        context,
        &[account_create_ix, instruction],
        &owner.pubkey(),
        &[account_keypair, owner],
    )
    .await
    .unwrap();
    Ok(())
}

pub async fn get_merkle_tree_snapshots(
    context: &mut ProgramTestContext,
    test_indexer: &TestIndexer,
    pubkeys: &Vec<Pubkey>,
) -> Vec<MerkleTreeTestSnapShot> {
    let mut snapshots = Vec::new();
    for pubkey in pubkeys.iter() {
        let merkle_tree_account =
            AccountZeroCopy::<StateMerkleTreeAccount>::new(context, *pubkey).await;
        let merkle_tree = merkle_tree_account
            .deserialized()
            .copy_merkle_tree()
            .unwrap();
        let accounts = test_indexer
            .state_merkle_trees
            .iter()
            .find(|x| x.0.merkle_tree == *pubkey)
            .expect("merkle tree not found in test indexer");
        snapshots.push(MerkleTreeTestSnapShot {
            accounts: accounts.0.clone(),
            root: merkle_tree.root().unwrap(),
            next_index: merkle_tree.next_index(),
            num_added_accounts: pubkeys.iter().filter(|x| **x == *pubkey).count(),
        });
    }
    snapshots
}

pub async fn perform_compressed_transfer_test(
    payer: &Keypair,
    context: &mut ProgramTestContext,
    test_indexer: &mut TestIndexer,
    mint: &Pubkey,
    from: &Keypair,
    recipients: &[Pubkey],
    amounts: &[u64],
    input_compressed_accounts: &[TokenDataWithContext],
    output_merkle_tree_pubkeys: &Vec<Pubkey>,
    transaction_params: Option<TransactionParams>,
) {
    if recipients.len() != amounts.len() && amounts.len() != output_merkle_tree_pubkeys.len() {
        panic!("recipients, amounts, and output_merkle_tree_pubkeys must have the same length");
    }
    let mut input_merkle_tree_context = Vec::new();
    let mut input_compressed_account_token_data = Vec::new();
    let mut input_compressed_account_hashes = Vec::new();
    let mut sum_input_amounts = 0;
    for account in input_compressed_accounts {
        let leaf_index = account.compressed_account.merkle_context.leaf_index.clone();
        input_compressed_account_token_data.push(account.token_data);
        input_compressed_account_hashes.push(
            account
                .compressed_account
                .compressed_account
                .hash::<Poseidon>(
                    &account.compressed_account.merkle_context.merkle_tree_pubkey,
                    &leaf_index,
                )
                .unwrap(),
        );
        sum_input_amounts += account.token_data.amount;
        input_merkle_tree_context.push(MerkleContext {
            merkle_tree_pubkey: account.compressed_account.merkle_context.merkle_tree_pubkey,
            nullifier_queue_pubkey: account
                .compressed_account
                .merkle_context
                .nullifier_queue_pubkey,
            leaf_index,
        });
    }
    let mut output_compressed_accounts = Vec::new();
    for (recipient, amount) in recipients.iter().zip(amounts) {
        let account = TokenTransferOutputData {
            amount: *amount,
            owner: *recipient,
            lamports: None,
        };
        sum_input_amounts -= amount;
        output_compressed_accounts.push(account);
    }
    // add change compressed account if tokens are left
    if sum_input_amounts > 0 {
        let account = TokenTransferOutputData {
            amount: sum_input_amounts,
            owner: from.pubkey(),
            lamports: None,
        };
        output_compressed_accounts.push(account);
    }
    let input_merkle_tree_pubkeys: Vec<Pubkey> = input_merkle_tree_context
        .iter()
        .map(|x| x.merkle_tree_pubkey)
        .collect();

    let proof_rpc_result = test_indexer
        .create_proof_for_compressed_accounts(
            Some(&input_compressed_account_hashes),
            &input_merkle_tree_pubkeys,
            None,
            &[],
            context,
        )
        .await;

    let instruction = light_compressed_token::transfer_sdk::create_transfer_instruction(
        &payer.pubkey(),
        &from.pubkey(), // authority
        &input_merkle_tree_context,
        &output_merkle_tree_pubkeys, // output_compressed_account_merkle_tree_pubkeys
        &output_compressed_accounts, // output_compressed_accounts
        &proof_rpc_result.root_indices,
        &Some(proof_rpc_result.proof),
        input_compressed_account_token_data.as_slice(), // input_token_data
        *mint,
        None,  // owner_if_delegate_is_signer
        false, // is_compress
        None,  // compression_amount
        None,  // token_pool_pda
        None,  // decompress_token_account
    )
    .unwrap();

    let snapshots =
        get_merkle_tree_snapshots(context, test_indexer, output_merkle_tree_pubkeys).await;
    let input_snapshots =
        get_merkle_tree_snapshots(context, test_indexer, &input_merkle_tree_pubkeys).await;
    let event = create_and_send_transaction_with_event(
        context,
        &[instruction],
        &payer.pubkey(),
        &[&payer, &from],
        transaction_params,
        // Some(TransactionParams {
        //     num_new_addresses: 0,
        //     num_input_compressed_accounts: input_compressed_account_hashes.len() as u8,
        //     num_output_compressed_accounts: output_compressed_accounts.len() as u8,
        //     compress: 5000, // for second signer
        //     fee_config: crate::FeeConfig::default(),
        // }),
    )
    .await
    .unwrap()
    .unwrap();

    test_indexer.add_compressed_accounts_with_token_data(event);
    println!("snapshots: {:?}", snapshots);
    assert_transfer(
        context,
        &test_indexer,
        &output_compressed_accounts,
        &input_compressed_account_hashes,
        &snapshots,
        &input_snapshots,
    )
    .await;
}

pub async fn decompress_test(
    payer: &Keypair,
    context: &mut ProgramTestContext,
    test_indexer: &mut TestIndexer,
    input_compressed_accounts: Vec<TokenDataWithContext>,
    amount: u64,
    output_merkle_tree_pubkey: &Pubkey,
    recipient_token_account: &Pubkey,
    transaction_params: Option<TransactionParams>,
) {
    let max_amount: u64 = input_compressed_accounts
        .iter()
        .map(|x| x.token_data.amount)
        .sum();
    let change_out_compressed_account = TokenTransferOutputData {
        amount: max_amount - amount,
        owner: payer.pubkey(),
        lamports: None,
    };
    let input_compressed_account_hashes = input_compressed_accounts
        .iter()
        .map(|x| {
            x.compressed_account
                .compressed_account
                .hash::<Poseidon>(
                    &x.compressed_account.merkle_context.merkle_tree_pubkey,
                    &x.compressed_account.merkle_context.leaf_index,
                )
                .unwrap()
        })
        .collect::<Vec<_>>();
    let input_merkle_tree_pubkeys = input_compressed_accounts
        .iter()
        .map(|x| x.compressed_account.merkle_context.merkle_tree_pubkey)
        .collect::<Vec<_>>();

    let proof_rpc_result = test_indexer
        .create_proof_for_compressed_accounts(
            Some(&input_compressed_account_hashes),
            &input_merkle_tree_pubkeys,
            None,
            &[],
            context,
        )
        .await;
    let mint = input_compressed_accounts[0].token_data.mint;
    let output_merkle_tree_pubkeys = vec![*output_merkle_tree_pubkey];
    let instruction = create_transfer_instruction(
        &context.payer.pubkey(),
        &payer.pubkey(), // authority
        &input_compressed_accounts
            .iter()
            .map(|x| x.compressed_account.merkle_context)
            .collect::<Vec<_>>(), // input_compressed_account_merkle_tree_pubkeys
        &output_merkle_tree_pubkeys, // output_cmerkle_contextmerkle_tree_pubkeys
        &[change_out_compressed_account], // output_compressed_accounts
        &proof_rpc_result.root_indices, // root_indices
        &Some(proof_rpc_result.proof),
        input_compressed_accounts
            .iter()
            .map(|x| x.token_data)
            .collect::<Vec<_>>()
            .as_slice(), // input_token_data
        mint,                            // mint
        None,                            // owner_if_delegate_is_signer
        false,                           // is_compress
        Some(amount),                    // compression_amount
        Some(get_token_pool_pda(&mint)), // token_pool_pda
        Some(*recipient_token_account),  // decompress_token_account
    )
    .unwrap();
    let output_merkle_tree_test_snapshots =
        get_merkle_tree_snapshots(context, test_indexer, &output_merkle_tree_pubkeys).await;
    let input_merkle_tree_test_snapshots =
        get_merkle_tree_snapshots(context, test_indexer, &input_merkle_tree_pubkeys).await;
    let recipient_token_account_data_pre = spl_token::state::Account::unpack(
        &context
            .banks_client
            .get_account(*recipient_token_account)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    let context_payer = context.payer.insecure_clone();
    let event = create_and_send_transaction_with_event(
        context,
        &[instruction],
        &payer.pubkey(),
        &[&context_payer, &payer],
        transaction_params,
    )
    .await
    .unwrap()
    .unwrap();

    test_indexer.add_compressed_accounts_with_token_data(event);

    assert_transfer(
        context,
        test_indexer,
        &[change_out_compressed_account],
        input_compressed_account_hashes.as_slice(),
        &output_merkle_tree_test_snapshots,
        &input_merkle_tree_test_snapshots,
    )
    .await;

    let recipient_token_account_data = spl_token::state::Account::unpack(
        &context
            .banks_client
            .get_account(*recipient_token_account)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();
    assert_eq!(
        recipient_token_account_data.amount,
        recipient_token_account_data_pre.amount + amount
    );
}
