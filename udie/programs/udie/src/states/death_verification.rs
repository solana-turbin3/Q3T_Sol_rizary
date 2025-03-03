use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DeathVerification {
    pub inheritance_plan: Pubkey,
    pub verified_by: Pubkey,
    pub verified_at: i64,
    #[max_len(64)]
    pub death_certificate_hash: String,
    pub bump: u8,
} 