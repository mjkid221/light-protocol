use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
pub mod invoke;
pub use invoke::instruction::*;
pub mod invoke_cpi;
pub use invoke_cpi::{initialize::*, instruction::*};
pub mod constants;
pub mod errors;
pub mod sdk;
pub mod utils;
use errors::SystemProgramError;
use sdk::event::PublicTransactionEvent;

declare_id!("2k4dpbM3XmRUS6BMbhAqb3pBuNsJ1tixK7Ac4gnAYPnn");

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "light_system_program",
    project_url: "lightprotocol.com",
    contacts: "email:security@lightprotocol.com",
    policy: "https://github.com/Lightprotocol/light-protocol/blob/main/SECURITY.md",
    source_code: "https://github.com/Lightprotocol/light-protocol"
}

#[program]
pub mod light_system_program {

    use light_heap::{bench_sbf_end, bench_sbf_start};

    use self::{
        invoke::{processor::process, verify_signer::input_compressed_accounts_signer_check},
        invoke_cpi::processor::process_invoke_cpi,
    };
    use super::*;

    pub fn init_cpi_context_account(_ctx: Context<InitializeCpiContextAccount>) -> Result<()> {
        use account_compression::state_merkle_tree_from_bytes_zero_copy_mut;

        let merkle_tree_account = _ctx.accounts.associated_merkle_tree.to_account_info();
        let mut data = merkle_tree_account.try_borrow_mut_data()?;
        state_merkle_tree_from_bytes_zero_copy_mut(&mut data)?;
        _ctx.accounts
            .cpi_context_account
            .init(_ctx.accounts.associated_merkle_tree.key());
        Ok(())
    }

    pub fn invoke<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InvokeInstruction<'info>>,
        inputs: Vec<u8>,
    ) -> Result<()> {
        let inputs: InstructionDataInvoke =
            InstructionDataInvoke::deserialize(&mut inputs.as_slice())?;

        input_compressed_accounts_signer_check(&inputs, &ctx.accounts.authority.key())?;
        process(inputs, None, ctx)
    }

    pub fn invoke_cpi<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InvokeCpiInstruction<'info>>,
        inputs: Vec<u8>,
    ) -> Result<()> {
        bench_sbf_start!("cpda_deserialize");
        let inputs: InstructionDataInvokeCpi =
            InstructionDataInvokeCpi::deserialize(&mut inputs.as_slice())?;
        bench_sbf_end!("cpda_deserialize");

        process_invoke_cpi(ctx, inputs)
    }

}
