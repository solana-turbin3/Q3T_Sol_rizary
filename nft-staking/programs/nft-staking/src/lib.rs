use anchor_lang::prelude::*;

pub mod context;
pub mod states;
mod error;

pub use context::*;

declare_id!("8a9cYnRrNF5NtjRrBiNGcW4qEHXRzVZitJkQ5msyXPiD");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<InitConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }
}
