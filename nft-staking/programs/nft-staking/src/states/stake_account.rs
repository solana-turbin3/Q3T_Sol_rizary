use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccountState {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub staked_at: i64,
    pub bump: u8,
}