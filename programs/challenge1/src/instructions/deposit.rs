use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};
use anchor_spl::token_2022::spl_token_2022::instruction::TokenInstruction;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use std::str::FromStr;

use crate::{error::VaultError, User, VaultConfig};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        token::mint = mint,
        token::authority = signer,
    )]
    pub source_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault_config,
    )]
    pub vault_ta: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Instructions sysvar for memo introspection
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, nonce: u64) -> Result<()> {
        let expected_memo = format!("deposit:{}:{}:{}", self.signer.key(), amount, nonce);

        Self::verify_transfer_in_transaction(
            &self.instructions,
            self.token_program.key(),
            self.source_ata.key(),
            self.mint.key(),
            self.vault_ta.key(),
            self.signer.key(),
            amount,
        )?;

        Self::verify_memo_in_transaction(&self.instructions, &expected_memo)?;

        self.user.balance = self
            .user
            .balance
            .checked_add(amount)
            .ok_or(VaultError::ArithmeticError)?;

        Ok(())
    }

    fn verify_memo_in_transaction(
        instructions_sysvar: &AccountInfo,
        expected_memo: &str,
    ) -> Result<()> {
        // Current Instruction Index
        let current_index =
            load_current_index_checked(instructions_sysvar).map_err(|_| VaultError::InvalidMemo)?;

        let memo_program_ids = [
            Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")
                .map_err(|_| VaultError::InvalidMemo)?,
            Pubkey::from_str("Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo")
                .map_err(|_| VaultError::InvalidMemo)?,
        ];

        // Check all previous instructions for memo
        for i in 0..current_index {
            let ix = load_instruction_at_checked(i.into(), instructions_sysvar)
                .map_err(|_| VaultError::InvalidMemo)?;

            if memo_program_ids.contains(&ix.program_id) {
                // Parse Memo Data
                let memo_text =
                    std::str::from_utf8(&ix.data).map_err(|_| VaultError::InvalidMemo)?;

                if memo_text == expected_memo {
                    return Ok(());
                }
            }
        }

        Err(VaultError::MemoNotFound.into())
    }

    fn verify_transfer_in_transaction(
        instructions_sysvar: &AccountInfo,
        token_program_id: Pubkey,
        source_ata: Pubkey,
        mint: Pubkey,
        vault_ta: Pubkey,
        signer: Pubkey,
        expected_amount: u64,
    ) -> Result<()> {
        let current_index =
            load_current_index_checked(instructions_sysvar).map_err(|_| VaultError::TransferNotFound)?;

        for i in 0..current_index {
            let ix = load_instruction_at_checked(i.into(), instructions_sysvar)
                .map_err(|_| VaultError::TransferNotFound)?;

            if ix.program_id != token_program_id {
                continue;
            }

            let Ok(token_ix) = TokenInstruction::unpack(&ix.data) else {
                continue;
            };

            if let TokenInstruction::TransferChecked { amount, .. } = token_ix {
                if ix.accounts.len() < 4 {
                    continue;
                }

                let source_matches = ix.accounts[0].pubkey == source_ata;
                let mint_matches = ix.accounts[1].pubkey == mint;
                let destination_matches = ix.accounts[2].pubkey == vault_ta;
                let authority_matches = ix.accounts[3].pubkey == signer;
                let amount_matches = amount == expected_amount;

                if source_matches
                    && mint_matches
                    && destination_matches
                    && authority_matches
                    && amount_matches
                {
                    return Ok(());
                }
            }
        }

        Err(VaultError::TransferNotFound.into())
    }
}
