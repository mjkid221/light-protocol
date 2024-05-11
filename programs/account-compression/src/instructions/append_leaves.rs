use std::cmp;

use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
use light_concurrent_merkle_tree::event::{ChangelogEvent, ChangelogEventV1, Changelogs, PathNode};
use light_macros::heap_neutral;

use crate::{
    emit_indexer_event,
    errors::AccountCompressionErrorCode,
    state::StateMerkleTreeAccount,
    utils::check_registered_or_signer::{check_registered_or_signer, GroupAccess, GroupAccounts},
    RegisteredProgram,
};

const BATCH_SIZE: usize = 7;

#[derive(Accounts)]
pub struct AppendLeaves<'info> {
    #[account(mut)]
    /// Signer used to pay rollover and protocol fees.
    pub fee_payer: Signer<'info>,
    /// CHECK: should only be accessed by a registered program/owner/delegate.
    pub authority: Signer<'info>,
    pub registered_program_pda: Option<Account<'info, RegisteredProgram>>,
    /// CHECK: in event emitting
    pub log_wrapper: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl GroupAccess for StateMerkleTreeAccount {
    fn get_owner(&self) -> &Pubkey {
        &self.owner
    }

    fn get_delegate(&self) -> &Pubkey {
        &self.delegate
    }
}

impl<'info> GroupAccounts<'info> for AppendLeaves<'info> {
    fn get_signing_address(&self) -> &Signer<'info> {
        &self.authority
    }
    fn get_registered_program_pda(&self) -> &Option<Account<'info, RegisteredProgram>> {
        &self.registered_program_pda
    }
}

/// for every leaf one Merkle tree account has to be passed as remaining account
/// for every leaf could be inserted into a different Merkle tree account
/// 1. deduplicate Merkle trees and identify into which tree to insert what leaf
/// 2. iterate over every unique Merkle tree and batch insert leaves
///
#[heap_neutral]
pub fn process_append_leaves_to_merkle_trees<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AppendLeaves<'info>>,
    leaves: &'a [(u8, [u8; 32])],
) -> Result<()> {
    msg!("Processing {} leaves", leaves.len());
    msg!("leaves {:?}", leaves);
    for i in 0..(leaves.len().div_ceil(BATCH_SIZE)) {
        let leaves_start = i * BATCH_SIZE;
        let leaves_to_process = cmp::min(leaves.len().saturating_sub(i * BATCH_SIZE), BATCH_SIZE);
        let leaves_end = leaves_start + leaves_to_process;
        process_batch(&ctx, &leaves[i * BATCH_SIZE..leaves_end])?;
    }

    // for (index, lamports) in fee_vec {
    //     msg!("Transfering {} lamports to account {}", lamports, index);
    //     transfer_lamports_cpi(
    //         &ctx.accounts.fee_payer,
    //         &ctx.remaining_accounts[index as usize].to_account_info(),
    //         lamports,
    //     )?;
    // }
    Ok(())
}

// TODO: refactor to
// for every remaining account which is one Merkle tree
// do function call to iterate over all remaining leaves until the mt index changes
// then append the batch to the Merkle tree
// free heap memory

#[heap_neutral]
#[inline(never)]
fn process_batch<'a, 'c: 'info, 'info>(
    ctx: &Context<'a, '_, 'c, 'info, AppendLeaves<'info>>,
    leaves: &'a [(u8, [u8; 32])],
) -> Result<()> {
    // let mut leaves_in_batch = 0;
    // let mut batch = Vec::with_capacity(BATCH_SIZE);
    // let mut changelog_events = Vec::with_capacity(BATCH_SIZE);
    // init with first Merkle tree account
    // iterate over all leaves
    // if leaf belongs to current Merkle tree account insert into batch
    // if leaf does not belong to current Merkle tree account
    // append batch to Merkle tree
    // insert rollover fee into vector
    // reset batch
    // get next Merkle tree account
    // append leaf of different Merkle tree account to batch
    msg!("Processing batch with {:?} leaves", leaves);
    let len = ctx.remaining_accounts.len();
    for i in 0..len {
        msg!(
            "Processing Merkle tree account {:?}",
            ctx.remaining_accounts[i].key()
        );
        // if leaf.0 as usize == current_mt_index {
        //     batch.push(&leaf.1);
        //     leaves_in_batch += 1;
        // }

        // if leaf.0 as usize != current_mt_index || leaves_in_batch == leaves.len() {
        // Insert leaves to the Merkle tree.
        // msg!("Processing batch with {} leaves", leaves_in_batch);
        let lamports = {
            let start = leaves.iter().position(|x| x.0 as usize == i).unwrap();
            let end = match leaves[start..].iter().position(|x| x.0 as usize != i) {
                Some(pos) => pos + start,
                None => leaves.len(),
            };
            msg!("start {} end {}", start, end);
            let merkle_tree_acc_info = &ctx.remaining_accounts[i];

            let merkle_tree_account =
                AccountLoader::<StateMerkleTreeAccount>::try_from(merkle_tree_acc_info).unwrap();

            let mut merkle_tree_account = merkle_tree_account.load_mut()?;
            let mut lamports = merkle_tree_account.rollover_fee * (end - start) as u64;
            lamports += merkle_tree_account.tip;
            // let index = fee_vec.iter().position(|x| x.0 == current_mt_index as u8);
            // match index {
            //     Some(i) => {
            //         fee_vec[i].1 += merkle_tree_account.rollover_fee * leaves_in_batch as u64;
            //         msg!("Incremented fee {:?}  fee_vec[i]", fee_vec[i]);
            //     }
            //     None => {
            //         fee_vec.push((
            //             current_mt_index as u8,
            //             merkle_tree_account.rollover_fee * leaves_in_batch as u64
            //                 + merkle_tree_account.tip,
            //         ));
            //     }
            // }

            check_registered_or_signer::<AppendLeaves, StateMerkleTreeAccount>(
                ctx,
                &merkle_tree_account,
            )?;

            let merkle_tree = merkle_tree_account.load_merkle_tree_mut()?;
            merkle_tree
                .append_batch(
                    leaves[start..end]
                        .iter()
                        .map(|x| &x.1)
                        .collect::<Vec<&[u8; 32]>>()
                        .as_slice(),
                )
                .map_err(ProgramError::from)?;
            lamports
        };
        transfer_lamports_cpi(
            &ctx.accounts.fee_payer,
            &ctx.remaining_accounts[i].to_account_info(),
            lamports,
        )?;
    }
    // current_mt_index = leaf.0 as usize;
    // batch = Vec::with_capacity(BATCH_SIZE);
    // batch.push(&leaf.1);
    // leaves_in_batch += 1;
    // if leaves_in_batch == BATCH_SIZE {
    //     // We reached the batch limit.
    //     break;
    // }
    // }

    Ok(())
}

pub fn transfer_lamports<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    lamports: u64,
) -> Result<()> {
    let compressed_sol_pda_lamports = from.as_ref().lamports();

    **from.as_ref().try_borrow_mut_lamports()? =
        compressed_sol_pda_lamports.checked_sub(lamports).unwrap();
    let recipient_lamports = to.as_ref().lamports();
    **to.as_ref().try_borrow_mut_lamports()? = recipient_lamports.checked_add(lamports).unwrap();
    Ok(())
}

pub fn transfer_lamports_cpi<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    lamports: u64,
) -> Result<()> {
    let instruction =
        anchor_lang::solana_program::system_instruction::transfer(from.key, to.key, lamports);
    anchor_lang::solana_program::program::invoke(&instruction, &[from.clone(), to.clone()])?;
    Ok(())
}
