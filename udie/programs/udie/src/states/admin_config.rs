use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
}

impl AdminConfig {
    pub const LEN: usize = 32 + 2 + 1; // Pubkey (32) + u16 (2) + u8 (1)
} 