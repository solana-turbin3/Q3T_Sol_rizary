use anchor_lang::prelude::*;
use crate::errors::UdieError;
#[account]
#[derive(InitSpace)]
pub struct InheritancePlan {
    pub owner: Pubkey,
    pub created_at: i64,
    pub last_updated: i64,
    pub last_activity: i64,
    pub is_active: bool,
    pub death_verified: bool,
    pub verification_timestamp: i64,
    pub total_beneficiaries: u8,
    pub total_assets: u32,
    pub freeze_period: i64,      // Timelock period after death verification
    pub safety_period: i64,      // Cool-down period between major changes
    pub bump: u8,
}

impl InheritancePlan {
    pub fn can_modify(&self) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        // Check safety period
        require!(
            current_time >= self.last_activity + self.safety_period,
            UdieError::SafetyPeriodActive
        );

        // Check freeze period if death verified
        if self.death_verified {
            require!(
                current_time >= self.verification_timestamp + self.freeze_period,
                UdieError::FreezePeriodActive
            );
        }

        Ok(())
    }
} 