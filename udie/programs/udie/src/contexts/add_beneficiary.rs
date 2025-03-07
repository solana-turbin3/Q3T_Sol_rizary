use anchor_lang::prelude::*;
use crate::states::{InheritancePlan, Beneficiary};
use crate::errors::UdieError;

#[derive(Accounts)]
pub struct AddBeneficiary<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"inheritance_plan", owner.key().as_ref()],
        bump = inheritance_plan.bump,
        constraint = inheritance_plan.owner == owner.key() @UdieError::InvalidOwner,
        constraint = inheritance_plan.is_active == true @UdieError::PlanLocked,
        constraint = !inheritance_plan.death_verified @UdieError::DeathNotVerified
    )]
    pub inheritance_plan: Account<'info, InheritancePlan>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + Beneficiary::INIT_SPACE,
        seeds = [
            b"beneficiary", 
            inheritance_plan.key().as_ref(),
            beneficiary_wallet.key().as_ref()
        ],
        bump
    )]
    pub beneficiary: Account<'info, Beneficiary>,
    
    /// CHECK: Wallet address of beneficiary
    pub beneficiary_wallet: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> AddBeneficiary<'info> {
    pub fn add(
        &mut self,
        relationship: String,
        share_percentage: u8,
        bumps: &AddBeneficiaryBumps
    ) -> Result<()> {
        // Validate share based on Islamic rules
        Beneficiary::validate_share(&relationship, share_percentage)?;
        
        // Ensure total share doesn't exceed 100%
        let total_share = self.inheritance_plan.total_beneficiaries
            .checked_mul(share_percentage)
            .ok_or(UdieError::Overflow)?;
            
        require!(
            total_share <= 100,
            UdieError::ShareExceeds100Percent
        );
        
        self.beneficiary.set_inner(Beneficiary {
            wallet: self.beneficiary_wallet.key(),
            relationship,
            share_percentage,
            has_withdrawn: false,
            inheritance_plan: self.inheritance_plan.key(),
            bump: bumps.beneficiary,
        });
        
        self.inheritance_plan.total_beneficiaries += 1;
        
        Ok(())
    }
} 