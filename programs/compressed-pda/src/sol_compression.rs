use aligned_sized::*;
use anchor_lang::{
    prelude::*,
    solana_program::{account_info::AccountInfo, pubkey::Pubkey},
};

use crate::{append_state::get_seeds, InstructionDataTransfer, TransferInstruction};

#[account]
#[aligned_sized(anchor)]
pub struct CompressedSolPda {}

#[constant]
pub const COMPRESSED_SOL_PDA_SEED: &[u8] = b"compressed_sol_pda";

#[derive(Accounts)]
pub struct InitializeCompressedSolPda<'info> {
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    #[account(
        init,
        payer = fee_payer,
        seeds = [COMPRESSED_SOL_PDA_SEED],
        bump,
        space = CompressedSolPda::LEN,
    )]
    pub compressed_sol_pda: Account<'info, CompressedSolPda>,
    pub system_program: Program<'info, System>,
}

pub fn compression_lamports<'a, 'b, 'c: 'info, 'info>(
    inputs: &'a InstructionDataTransfer,
    ctx: &'a Context<'a, 'b, 'c, 'info, TransferInstruction<'info>>,
) -> Result<()> {
    if inputs.is_compress {
        compress_lamports(inputs, ctx)
    } else if inputs.compression_lamports.is_some() {
        decompress_lamports(inputs, ctx)
    } else {
        Ok(())
    }
}

pub fn decompress_lamports<'a, 'b, 'c: 'info, 'info>(
    inputs: &'a InstructionDataTransfer,
    ctx: &'a Context<'a, 'b, 'c, 'info, TransferInstruction<'info>>,
) -> Result<()> {
    let recipient = match ctx.accounts.compression_recipient.as_ref() {
        Some(compression_recipient) => compression_recipient.to_account_info(),
        None => return err!(crate::ErrorCode::DecompressRecipientUndefinedForDecompressSol),
    };
    let compressed_sol_pda = match ctx.accounts.compressed_sol_pda.as_ref() {
        Some(compressed_sol_pda) => compressed_sol_pda.to_account_info(),
        None => return err!(crate::ErrorCode::CompressedSolPdaUndefinedForDecompressSol),
    };
    let lamports = match inputs.compression_lamports {
        Some(lamports) => lamports,
        None => return err!(crate::ErrorCode::DeCompressLamportsUndefinedForDecompressSol),
    };

    let compressed_sol_pda_lamports = compressed_sol_pda.as_ref().lamports();
    **compressed_sol_pda.as_ref().try_borrow_mut_lamports()? =
        match compressed_sol_pda_lamports.checked_sub(lamports) {
            Some(lamports) => anchor_lang::Result::Ok(lamports),
            None => return err!(crate::ErrorCode::InsufficientLamportsForDecompressSol),
        }?;
    let recipient_lamports = recipient.as_ref().lamports();
    **recipient.as_ref().try_borrow_mut_lamports()? = match recipient_lamports.checked_add(lamports)
    {
        Some(lamports) => anchor_lang::Result::Ok(lamports),
        None => return err!(crate::ErrorCode::AdditionOverflowForDecompressSol),
    }?;
    Ok(())
}

pub fn compress_lamports<'a, 'b, 'c: 'info, 'info>(
    inputs: &'a InstructionDataTransfer,
    ctx: &'a Context<'a, 'b, 'c, 'info, TransferInstruction<'info>>,
) -> Result<()> {
    let recipient = match ctx.accounts.compressed_sol_pda.as_ref() {
        Some(compressed_sol_pda) => compressed_sol_pda.to_account_info(),
        None => return err!(crate::ErrorCode::CompressedSolPdaUndefinedForCompressSol),
    };
    let lamports = match inputs.compression_lamports {
        Some(lamports) => lamports,
        None => return err!(crate::ErrorCode::DeCompressLamportsUndefinedForCompressSol),
    };

    transfer_lamports(
        &ctx.accounts.authority.to_account_info(),
        &recipient,
        &ctx.accounts.account_compression_authority.to_account_info(),
        lamports,
    )
}

pub fn transfer_lamports<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    lamports: u64,
) -> Result<()> {
    msg!("transfer_lamports {}", lamports);
    msg!("from lamports: {}", from.lamports());
    msg!("to lamports: {}", to.lamports());
    let instruction =
        anchor_lang::solana_program::system_instruction::transfer(from.key, to.key, lamports);
    let (seed, bump) = get_seeds(&crate::ID, &authority.key())?;
    let bump = &[bump];
    let seeds = &[&[b"cpi_authority", seed.as_slice(), bump][..]];

    anchor_lang::solana_program::program::invoke_signed(
        &instruction,
        &[authority.clone(), from.clone(), to.clone()],
        seeds,
    )?;
    Ok(())
}
