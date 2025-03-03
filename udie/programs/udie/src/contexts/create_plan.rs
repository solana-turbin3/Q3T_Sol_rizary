use anchor_lang::prelude::*;
use crate::states::{AdminConfig, InheritancePlan};

#[derive(Accounts)]
pub struct CreatePlan<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + InheritancePlan::INIT_SPACE,
        seeds = [b"inheritance_plan", owner.key().as_ref()],
        bump
    )]
    pub inheritance_plan: Account<'info, InheritancePlan>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePlan<'info> {
    pub fn create(&mut self, bumps: &CreatePlanBumps) -> Result<()> {
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        
        self.inheritance_plan.set_inner(InheritancePlan {
            owner: self.owner.key(),
            created_at: now,
            last_updated: now,
            last_activity: now,
            is_active: true,
            death_verified: false,
            verification_timestamp: 0,
            total_beneficiaries: 0,
            total_assets: 0,
            freeze_period: 7 * 24 * 60 * 60, // 7 days in seconds
            safety_period: 24 * 60 * 60,     // 1 day in seconds
            bump: bumps.inheritance_plan,
        });
        
        Ok(())
    }
} 