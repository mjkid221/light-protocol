use crate::{CompressedTokenInstructionDataTransfer, TransferInstruction};
use anchor_lang::{prelude::*, solana_program::account_info::AccountInfo};
use anchor_spl::token::Transfer;
use psp_compressed_pda::append_state::get_seeds;

pub fn de_compress_amount<'a, 'info>(
    inputs: &'a CompressedTokenInstructionDataTransfer,
    ctx: &Context<'_, '_, '_, 'info, TransferInstruction<'info>>,
) -> Result<()> {
    if inputs.is_compress {
        compress_spl_tokens(inputs, ctx)
    } else if inputs.de_compress_amount.is_some() {
        decompress_spl_tokens(inputs, ctx)
    } else {
        Ok(())
    }
}

pub fn decompress_spl_tokens<'a, 'info>(
    inputs: &'a CompressedTokenInstructionDataTransfer,
    ctx: &Context<'_, '_, '_, 'info, TransferInstruction<'info>>,
) -> Result<()> {
    let recipient = match ctx.accounts.decompress_token_account.as_ref() {
        Some(de_compress_recipient) => de_compress_recipient.to_account_info(),
        None => return err!(crate::ErrorCode::DecompressRecipientUndefinedForDecompress),
    };
    let token_pool_pda = match ctx.accounts.token_pool_pda.as_ref() {
        Some(token_pool_pda) => token_pool_pda.to_account_info(),
        None => return err!(crate::ErrorCode::CompressedPdaUndefinedForDecompress),
    };
    let lamports = match inputs.de_compress_amount {
        Some(lamports) => lamports,
        None => return err!(crate::ErrorCode::DeCompressAmountUndefinedForDecompress),
    };
    token_pool_pda.sub_lamports(lamports)?;
    recipient.add_lamports(lamports)?;
    Ok(())
}

pub fn compress_spl_tokens<'a, 'info>(
    inputs: &'a CompressedTokenInstructionDataTransfer,
    ctx: &Context<'_, '_, '_, 'info, TransferInstruction<'info>>,
) -> Result<()> {
    let recipient = match ctx.accounts.token_pool_pda.as_ref() {
        Some(token_pool_pda) => token_pool_pda.to_account_info(),
        None => return err!(crate::ErrorCode::CompressedPdaUndefinedForCompress),
    };
    let lamports = match inputs.de_compress_amount {
        Some(lamports) => lamports,
        None => return err!(crate::ErrorCode::DeCompressAmountUndefinedForCompress),
    };

    transfer(
        &ctx.accounts
            .decompress_token_account
            .as_ref()
            .unwrap()
            .to_account_info(),
        &recipient,
        &ctx.accounts
            .psp_account_compression_authority
            .to_account_info(),
        &ctx.accounts
            .token_program
            .as_ref()
            .unwrap()
            .to_account_info(),
        lamports,
    )
}

pub fn transfer<'info>(
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let (seed, bump) = get_seeds(&crate::ID, &authority.key())?;
    let bump = &[bump];
    let seeds = &[&[b"cpi_authority", seed.as_slice(), bump][..]];

    let accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(token_program.to_account_info(), accounts, seeds);
    anchor_spl::token::transfer(cpi_ctx, amount)
}
