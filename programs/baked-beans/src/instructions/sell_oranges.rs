use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
use anchor_lang::prelude::*;
use solana_program::{program::invoke_signed, system_instruction};
#[derive(Accounts)]
pub struct SellOranges<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be checked with address in global_state
    pub vault: AccountInfo<'info>,

    #[account(mut, address = global_state.treasury)]
    /// CHECK: this should be checked with address in global_state
    pub treasury: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_state: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
}

impl<'info> SellOranges<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<SellOranges>) -> Result<()> {
    let cur_timestamp = Clock::get()?.unix_timestamp as u64;
    let accts = ctx.accounts;
    require!(accts.global_state.is_started == 1, BeanError::NotStarted);

    msg!("SellOranges claimed oranges {}", accts.user_state.claimed_oranges);
    let has_oranges = accts
        .user_state
        .claimed_oranges
        .checked_add(get_oranges_since_last_hatch(
            &accts.user_state,
            cur_timestamp,
            accts.global_state.oranges_per_miner,
        )?)
        .unwrap();

    msg!("SellOranges has_oranges {}", has_oranges);
    let mut orange_value = calculate_oranges_sell(&accts.global_state, has_oranges, accts.vault.lamports())?;

    let fee = dev_fee(&accts.global_state, orange_value)?;
    accts.user_state.claimed_oranges = 0;
    accts.user_state.last_hatch_time = cur_timestamp;
    accts.global_state.market_oranges = accts
        .global_state
        .market_oranges
        .checked_add(has_oranges)
        .unwrap();

    msg!("SellOranges selling orange_value {}", orange_value);
    msg!("SellOranges selling fee {}", fee);
    let real_val = orange_value.checked_sub(fee).unwrap();

    // send fee to treasury
    let bump = ctx.bumps.get("vault").unwrap();
    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.treasury.key(), fee),
        &[
            accts.vault.to_account_info().clone(),
            accts.treasury.clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[*bump]]],
    )?;
    // add vault <- sol_amount - fee
    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.user.key(), real_val),
        &[
            accts.vault.to_account_info().clone(),
            accts.user.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[*bump]]],
    )?;

    // lamports should be bigger than zero to prevent rent exemption
    let rent = Rent::default();
    let required_lamports = rent
        .minimum_balance(0)
        .max(1)
        .saturating_sub(accts.vault.to_account_info().lamports());
    require!(
        **accts.vault.lamports.borrow() > required_lamports,
        BeanError::InsufficientAmount
    );
    Ok(())
}
