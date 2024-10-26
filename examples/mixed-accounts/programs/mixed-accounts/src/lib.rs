use anchor_lang::prelude::*;
use light_sdk::{
    compressed_account::LightAccount, light_account, light_accounts, light_program,
    merkle_context::PackedAddressMerkleContext,
};

declare_id!("7yucc7fL3JGbyMwg4neUaenNSdySS39hbAk89Ao3t1Hz");

#[light_program]
#[program]
pub mod mixed_accounts {
    use super::*;

    pub fn with_compressed_account<'info>(
        ctx: LightContext<'_, '_, '_, 'info, WithCompressedAccount<'info>>,
        name: String,
    ) -> Result<()> {
        ctx.light_accounts.my_compressed_account.name = name;
        Ok(())
    }

    pub fn without_compressed_account<'info>(
        ctx: Context<'_, '_, '_, 'info, WithoutCompressedAccount<'info>>,
        name: String,
    ) -> Result<()> {
        ctx.accounts.my_regular_account.name = name;
        Ok(())
    }
}

#[light_account]
#[derive(Clone, Debug, Default)]
pub struct MyCompressedAccount {
    name: String,
}

#[account]
pub struct MyRegularAccount {
    name: String,
}

#[light_accounts]
#[instruction(name: String)]
pub struct WithCompressedAccount<'info> {
    #[account(mut)]
    #[fee_payer]
    pub signer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::MixedAccounts>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signed: AccountInfo<'info>,

    #[light_account(
        init,
        seeds = [b"compressed".as_slice(), name.as_bytes()],
    )]
    pub my_compressed_account: LightAccount<MyCompressedAccount>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct WithoutCompressedAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds = [b"compressed".as_slice(), name.as_bytes()],
        bump,
        payer = signer,
        space = 8 + 8,
    )]
    pub my_regular_account: Account<'info, MyRegularAccount>,
    pub system_program: Program<'info, System>,
}
