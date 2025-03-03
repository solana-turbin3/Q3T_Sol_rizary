use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub mod contexts;
pub mod states;
pub mod errors;
pub mod events;

use contexts::*;
use states::*;
use events::*;

declare_id!("7Ws6fZivk9af1CPRxFsUyY5wytyZLMFjZqJgYybaNf9j");

#[program]
pub mod udie {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        ctx.accounts.init(fee, &ctx.bumps)?;
        emit!(ConfigInitialized {
            admin: ctx.accounts.admin.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }

    pub fn create_inheritance_plan(ctx: Context<CreatePlan>) -> Result<()> {
        ctx.accounts.create(&ctx.bumps)
    }

    pub fn add_beneficiary(
        ctx: Context<AddBeneficiary>, 
        relationship: String,
        share_percentage: u8
    ) -> Result<()> {
        ctx.accounts.add(relationship, share_percentage, &ctx.bumps)
    }

    pub fn add_asset(
        ctx: Context<AddAsset>,
        amount: u64
    ) -> Result<()> {
        ctx.accounts.add(amount, &ctx.bumps)
    }

    pub fn verify_death(ctx: Context<VerifyDeath>, death_certificate_hash: String) -> Result<()> {
        ctx.accounts.verify(death_certificate_hash, &ctx.bumps)
    }

    pub fn withdraw_asset(ctx: Context<WithdrawAsset>) -> Result<()> {
        ctx.accounts.withdraw()
    }
}
