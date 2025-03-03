use anchor_lang::prelude::*;
use crate::states::AdminConfig;
use crate::errors::UdieError;
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + AdminConfig::LEN,
        seeds = [b"admin_config", admin.key().as_ref()],
        bump
    )]
    pub admin_config: Account<'info, AdminConfig>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        // Verify the admin is the signer
        require!(
            self.admin.is_signer,
            UdieError::Unauthorized
        );

        self.admin_config.set_inner(AdminConfig {
            admin: self.admin.key(),
            fee,
            bump: bumps.admin_config,
        });
        
        Ok(())
    }
} 