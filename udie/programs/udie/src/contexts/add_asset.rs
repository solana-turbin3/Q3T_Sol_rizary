use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::TransferChecked;
use crate::states::{InheritancePlan, Asset};
use crate::errors::UdieError;

#[derive(Accounts)]
pub struct AddAsset<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"inheritance_plan", owner.key().as_ref()],
        bump = inheritance_plan.bump,
        constraint = inheritance_plan.owner == owner.key() @UdieError::InvalidOwner,
        constraint = inheritance_plan.is_active == true @UdieError::PlanLocked,
    )]
    pub inheritance_plan: Account<'info, InheritancePlan>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + Asset::INIT_SPACE,
        seeds = [
            b"asset",
            inheritance_plan.key().as_ref(),
            mint.key().as_ref()
        ],
        bump
    )]
    pub asset: Account<'info, Asset>,
    
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = asset
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddAsset<'info> {
    pub fn add(&mut self, amount: u64, bumps: &AddAssetBumps) -> Result<()> {
        // Initialize asset state
        self.asset.set_inner(Asset {
            mint: self.mint.key(),
            amount,
            inheritance_plan: self.inheritance_plan.key(),
            vault: self.vault.key(),
            bump: bumps.asset,
        });
        
        // Transfer tokens to vault
        let cpi_accounts = TransferChecked {
            from: self.owner_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.owner.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            cpi_accounts
        );
        
        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;
        
        self.inheritance_plan.total_assets += 1;
        
        Ok(())
    }
} 