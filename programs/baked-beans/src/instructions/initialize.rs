use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use solana_program::{program::invoke, system_instruction};
use std::mem::size_of;

use std::str::FromStr;
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<GlobalState>(),
        payer = authority,
    )]
    pub global_state: Account<'info, GlobalState>,

    /// CHECK: this should be set by admin
    pub treasury: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        if self.global_state.is_initialized == 1 {
            require!(
                self.global_state.authority.eq(&self.authority.key()),
                BeanError::NotAllowedAuthority
            )
        }
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {

    /*require!(
        ctx.accounts.authority.key().eq(&Pubkey::from_str("87nyxMKv8THrMWbjETtrNTS6RC5ksGL5P7cG6L3RRZky").unwrap()),
        BeanError::NotAllowedAuthority
    );*/

    let accts = ctx.accounts;
    accts.global_state.is_initialized = 1;
    accts.global_state.authority = new_authority;
    accts.global_state.vault = accts.vault.key();
    accts.global_state.treasury = accts.treasury.key();

    accts.global_state.market_oranges = 108000000000;
    accts.global_state.dev_fee = 500; // means 5%
    accts.global_state.psn = 10000;
    accts.global_state.psnh = 5000;
    accts.global_state.oranges_per_miner = 1080000;
    accts.global_state.is_started = 0;

    let rent = Rent::default();
    let required_lamports = rent
        .minimum_balance(0)
        .max(1)
        .saturating_sub(accts.vault.to_account_info().lamports());
    msg!("required lamports = {:?}", required_lamports);
    invoke(
        &system_instruction::transfer(
            &accts.authority.key(),
            &accts.vault.key(),
            required_lamports,
        ),
        &[
            accts.authority.to_account_info().clone(),
            accts.vault.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;
    //Err(BeanError::NotAllowedAuthority.into())
    Ok(())
}
