use anchor_lang::prelude::*;
use crate::states::{InheritancePlan, DeathVerification};

#[derive(Accounts)]
pub struct VerifyDeath<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"inheritance_plan", inheritance_plan.owner.as_ref()],
        bump = inheritance_plan.bump,
        constraint = inheritance_plan.is_active == true,
        constraint = !inheritance_plan.death_verified
    )]
    pub inheritance_plan: Account<'info, InheritancePlan>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + DeathVerification::INIT_SPACE,
        seeds = [b"death_verification", inheritance_plan.key().as_ref()],
        bump
    )]
    pub verification: Account<'info, DeathVerification>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> VerifyDeath<'info> {
    pub fn verify(&mut self, death_certificate_hash: String, bumps: &VerifyDeathBumps) -> Result<()> {
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        
        // Update verification record
        self.verification.set_inner(DeathVerification {
            inheritance_plan: self.inheritance_plan.key(),
            verified_by: self.authority.key(),
            verified_at: now,
            death_certificate_hash,
            bump: bumps.verification,
        });
        
        // Update inheritance plan
        self.inheritance_plan.death_verified = true;
        self.inheritance_plan.verification_timestamp = now;
        self.inheritance_plan.last_updated = now;
        
        Ok(())
    }
} 