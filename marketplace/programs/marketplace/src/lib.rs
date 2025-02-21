use anchor_lang::prelude::*;

pub mod contexts;
pub mod state;
pub mod error;

declare_id!("717CNihfm7qYnouBCGiK5q8dDaGCvQGJiYCQEXeSgx32");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    
}

#[derive(Accounts)]
pub struct Initialize {}
