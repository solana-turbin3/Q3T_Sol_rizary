use anchor_lang::prelude::*;
mod instructions;
mod state;

use instructions::*;

declare_id!("HAMntPt4n26ME8z13Hn68zwNA8vaqgsioohE2AxHJ9dE");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.make(seed, receive_amount, ctx.bumps)?;
        ctx.accounts.deposit(receive_amount)?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }
}