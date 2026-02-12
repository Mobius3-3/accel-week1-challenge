use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::VaultConfig;

#[derive(Accounts)]
pub struct InitVaultConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + VaultConfig::INIT_SPACE,
        seeds = [b"vault-config"],
        bump
    )]
    pub vault_config: Account<'info, VaultConfig>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        token::mint = mint,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_ta: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitVaultConfig<'info> {
    pub fn init_vault_config(&mut self, bumps: &InitVaultConfigBumps) -> Result<()> {
        self.vault_config.set_inner(VaultConfig {
            admin: self.admin.key(),
            mint: self.mint.key(),
            vault_ta: self.vault_ta.key(),
            bump: bumps.vault_config,
        });

        Ok(())
    }
}