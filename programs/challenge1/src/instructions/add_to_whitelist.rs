use anchor_lang::prelude::*;
use crate::{state::{VaultState, WhitelistEntry}, error::VaultError};

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: User to be added to whitelist
    #[account(mut)]
    pub user: UncheckedAccount<'info>,

    #[account(
        init,
        space = WhitelistEntry::INIT_SPACE,
        payer = authority,
        seeds = [b"whitelist", vault_state.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, WhitelistEntry>,


    #[account(
        mut,
        has_one = authority @ VaultError::Unauthorized,
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}


impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist(&mut self, max_amount: u64, bumps: &AddToWhitelistBumps) -> Result<()> {
        let whitelist = &mut self.whitelist;

        whitelist.max_amount = max_amount;
        whitelist.vault = self.vault_state.key();
        whitelist.bump = bumps.whitelist;

        Ok(())
    }
}
