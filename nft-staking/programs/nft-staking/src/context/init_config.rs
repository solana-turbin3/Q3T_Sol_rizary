use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::states::StakeConfig;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init, 
        payer = admin, 
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::authority = config,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitConfig<'info> {
    pub fn init(&mut self, points_per_stake: u8, max_stake: u8, freeze_period: u32, bumps: &InitConfigBumps) -> Result<()> {
        self.config.set_inner( StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        
        Ok(())
    }
}