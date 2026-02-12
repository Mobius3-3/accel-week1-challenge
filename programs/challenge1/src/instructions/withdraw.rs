use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
 Mint, TokenAccount, TokenInterface, 
};

use crate::{error::VaultError, User, VaultConfig};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", signer.key().as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,

    #[account(
        seeds = [b"vault-config"],
        bump = vault_config.bump
    )]
    pub vault_config: Account<'info, VaultConfig>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = signer,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault_config,
    )]
    pub vault_ta: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require_gte!(
            self.user.balance,
            amount,
            VaultError::InsufficientBalance
        );

        self.user.balance = self
            .user
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::ArithmeticError)?;

        Ok(())
    }
}
