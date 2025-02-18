use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenInterface, TokenAccount, TransferChecked, close_account, transfer_checked, CloseAccount, Mint},
};

use crate::state::EscrowState;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        constraint = (maker.key() == escrow.maker.key()),
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let escrow_seed = self.escrow.seed.to_le_bytes();

        let seeds = &[
            b"escrow",
            self.escrow.maker.as_ref(),
            escrow_seed.as_ref(),
            &[self.escrow.bump],
        ];

        let signer = &[&seeds[..]];
        
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;
        
        msg!("Escrow refunded");
        
        Ok(())
    }
    
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        
        let escrow_seed = self.escrow.seed.to_le_bytes();

        let seeds = &[
            b"escrow",
            self.escrow.maker.as_ref(),
            escrow_seed.as_ref(),
            &[self.escrow.bump],
        ];

        let signer = &[&seeds[..]];
        
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        close_account(cpi_ctx)?;
        
        msg!("Escrow closed");
        
        Ok(())
    }
}