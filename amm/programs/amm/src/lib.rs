use anchor_lang::prelude::*;

mod state;
mod contexts;
mod errors;

pub use contexts::*;

declare_id!("4zLs23czYYEdHUGxGbZUcwWBFWEV21wma4fzpEdRob1h");

#[program]
pub mod amm_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64,fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, lp_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(lp_amount, max_x, max_y)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_amount, min_x, min_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
        ctx.accounts.swap(args)?;
        Ok(())
    }
}
