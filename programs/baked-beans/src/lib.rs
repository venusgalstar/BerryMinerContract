use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("4VUZQ2Tbx3BM9iq6DBH5fTk4oQ3iNZcC9qoUJUBhyyRs");
#[program]
pub mod berry_miner {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
        initialize::handle(ctx, new_authority)
    }

    pub fn buy_oranges(ctx: Context<BuyOranges>, amount: u64) -> Result<()> {
        buy_oranges::handle(ctx, amount)
    }

    pub fn sell_oranges(ctx: Context<SellOranges>) -> Result<()> {
        sell_oranges::handle(ctx)
    }

    pub fn hatch_oranges(ctx: Context<HatchOranges>) -> Result<()> {
        hatch_oranges::handle(ctx)
    }

    pub fn set_config(ctx: Context<SetConfig>, amount: u64) -> Result<()> {
        set_config::handle(ctx, amount)
    }

    pub fn start_mine(ctx: Context<StartMine>) -> Result<()> {
        start_mine::handle(ctx)
    }

    pub fn set_treasury(ctx: Context<SetTreasury>, key: Pubkey) -> Result<()> {
        set_treasury::handle(ctx, key)
    }

    pub fn set_admin(ctx: Context<SetAdmin>, key: Pubkey) -> Result<()> {
        set_admin::handle(ctx, key)
    }
}
