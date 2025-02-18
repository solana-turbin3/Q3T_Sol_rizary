use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub seed: u64,
    pub receive_amount: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey, // amount the token b that we want to receive
    pub bump: u8,
}
