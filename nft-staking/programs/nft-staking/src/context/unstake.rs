use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount
    },
    token::{Revoke, revoke, Mint, Token, TokenAccount},
};

use crate::states::{UserAccountState, StakeAccountState, StakeConfig};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>, 
    #[account(
        seeds = [
            b"metadata",
            nft_mint.key().as_ref(),
            metadata_program.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )] 
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadta",
            nft_mint.key().as_ref(),
            metadata_program.key().as_ref(),
            b"edition",
        ],
        bump,
        seeds::program = metadata_program.key(),
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"stake_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccountState>,

    #[account(
        mut,
        close = user,
        seeds = [b"stake_account", nft_mint.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccountState>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        self.user_account.amount_staked -= 1;

        let cpi_program = self.metadata_program.to_account_info();

        let cpi_accounts = ThawDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            token_account: &self.nft_mint_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.nft_mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        let seeds = [ 
            b"stake_account",
            self.nft_mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        ThawDelegatedAccountCpi::new(
            &cpi_program,
            cpi_accounts
        ).invoke_signed(signer_seeds)?;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Revoke {
            source: self.nft_mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        revoke(cpi_ctx)?;

        Ok(())
    }
}