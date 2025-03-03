use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint, TokenAccount, TokenInterface,
        transfer_checked, TransferChecked,
    },
};
use crate::states::{InheritancePlan, Asset, Beneficiary};
use crate::errors::UdieError;

#[derive(Accounts)]
pub struct WithdrawAsset<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            b"inheritance_plan", 
            inheritance_plan.owner.as_ref()
        ],
        bump = inheritance_plan.bump,
        constraint = inheritance_plan.is_active == true,
        constraint = inheritance_plan.death_verified
    )]
    pub inheritance_plan: Account<'info, InheritancePlan>,
    
    #[account(
        mut,
        seeds = [
            b"beneficiary",
            inheritance_plan.key().as_ref(),
            beneficiary.key().as_ref()
        ],
        bump = beneficiary_account.bump,
        constraint = beneficiary_account.is_verified == true,
        constraint = !beneficiary_account.has_withdrawn,
        constraint = beneficiary_account.wallet == beneficiary.key()
    )]
    pub beneficiary_account: Account<'info, Beneficiary>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        seeds = [
            b"asset",
            inheritance_plan.key().as_ref(),
            mint.key().as_ref()
        ],
        bump = asset.bump
    )]
    pub asset: Account<'info, Asset>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = asset
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary
    )]
    pub beneficiary_ata: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawAsset<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        // Calculate beneficiary's share
        let share_amount = self.asset.calculate_share(
            self.beneficiary_account.share_percentage
        )?;
        
        // Transfer tokens from vault to beneficiary
        let inheritance_plan_key = self.inheritance_plan.key();
        let mint_key = self.mint.key();
        let bump = [self.asset.bump];
        
        let seeds = [
            b"asset".as_ref(),
            inheritance_plan_key.as_ref(),
            mint_key.as_ref(),
            bump.as_ref(),
        ];
        
        let signer_seeds = &[&seeds[..]];
        
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.beneficiary_ata.to_account_info(),
            authority: self.asset.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds
        );
        
        transfer_checked(cpi_ctx, share_amount, self.mint.decimals)?;
        
        // Update states
        self.asset.amount = self.asset.amount
            .checked_sub(share_amount)
            .ok_or(UdieError::Overflow)?;
            
        self.beneficiary_account.has_withdrawn = true;
        
        // Update inheritance plan if asset is fully withdrawn
        if self.asset.amount == 0 {
            self.inheritance_plan.total_assets = self.inheritance_plan.total_assets
                .checked_sub(1)
                .ok_or(UdieError::Overflow)?;
        }
        
        Ok(())
    }
} 