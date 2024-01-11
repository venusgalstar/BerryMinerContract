use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
use anchor_lang::prelude::*;
use solana_program::{program::invoke, system_instruction};

use solana_program::{program::invoke_signed};

use std::mem::size_of;
#[derive(Accounts)]
pub struct SetTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
      has_one = authority
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(ctx: Context<SetTreasury>, treasury_key: Pubkey) -> Result<()> {
    let accts = ctx.accounts;

    accts.global_state.treasury = treasury_key;
    Ok(())
}
