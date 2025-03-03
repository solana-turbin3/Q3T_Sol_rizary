use anchor_lang::prelude::*;
use crate::errors::UdieError;

#[account]
#[derive(InitSpace)]
pub struct Asset {
    pub mint: Pubkey,
    pub amount: u64,
    pub inheritance_plan: Pubkey,
    pub vault: Pubkey,
    pub bump: u8,
}

impl Asset {
    pub fn calculate_share(&self, share_percentage: u8) -> Result<u64> {
        // Calculate beneficiary's share
        let share_amount = (self.amount
            .checked_mul(share_percentage as u64)
            .ok_or(UdieError::Overflow)?)
            .checked_div(100)
            .ok_or(UdieError::DivisionByZero)?;
            
        require!(share_amount > 0, UdieError::InvalidAmount);
        
        Ok(share_amount)
    }
} 