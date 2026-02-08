use anchor_lang::prelude::*;
use crate::{state::{VaultState, WhitelistEntry}, error::VaultError};

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: user being removed
    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = authority @ VaultError::Unauthorized
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [
            b"whitelist",
            vault_state.key().as_ref(),
            user.key().as_ref()
        ],
        bump = whitelist.bump,
        close = authority
    )]
    pub whitelist: Account<'info, WhitelistEntry>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        // Nothing needed here.
        Ok(())
    }
}