use anchor_lang::prelude::*;

use crate::User;

#[derive(Accounts)]
#[instruction(user_key: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
      init_if_needed,
      payer = admin,
      space = 8 + User::INIT_SPACE,
      seeds = [b"user", user_key.as_ref()],
      bump,
  )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self, _user_key: Pubkey, bumps:WhitelistOperationsBumps) -> Result<()> {
        self.user.bump = bumps.user;
        self.user.whitelisted = true;
        Ok(())
    }

    pub fn remove_from_whitelist(&mut self, _user_key: Pubkey) -> Result<()> {
        self.user.whitelisted = false;
        Ok(())
    }
}
