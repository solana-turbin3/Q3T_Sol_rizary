use anchor_lang::prelude::*;
use crate::errors::UdieError;
#[account]
#[derive(InitSpace)]
pub struct Beneficiary {
    pub wallet: Pubkey,
    #[max_len(32)]
    pub relationship: String,    // max 32 chars
    pub share_percentage: u8,
    pub has_withdrawn: bool,
    pub inheritance_plan: Pubkey,
    pub bump: u8,
}

impl Beneficiary {
    pub fn validate_share(relationship: &str, share: u8) -> Result<()> {
        // Built-in Islamic rules validation
        match relationship.to_lowercase().as_str() {
            "son" => require!(share <= 66, UdieError::InvalidSharePercentage),
            "daughter" => require!(share <= 33, UdieError::InvalidSharePercentage),
            "wife" => require!(share <= 25, UdieError::InvalidSharePercentage),
            "husband" => require!(share <= 50, UdieError::InvalidSharePercentage),
            // Add other relationships...
            _ => return err!(UdieError::InvalidRelationship)
        }
        
        Ok(())
    }
} 