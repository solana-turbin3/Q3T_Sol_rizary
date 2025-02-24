use anchor_lang::prelude::*;

pub mod contexts;
pub mod state;
pub mod error;

use contexts::*;

declare_id!("717CNihfm7qYnouBCGiK5q8dDaGCvQGJiYCQEXeSgx32");

#[program]
pub mod marketplace {
    use contexts::Purchase;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_listing()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?; 
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault_account()?;
        ctx.accounts.reward_buyer()?;
        Ok(())

    }
}