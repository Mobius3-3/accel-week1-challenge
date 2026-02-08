use std::cell::RefMut;

use anchor_lang::prelude::*;

use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount,
            BaseStateWithExtensions,
            PodStateWithExtensions
        },
        pod::PodAccount
    },
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::VaultState;
use crate::state::WhitelistEntry;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = authority
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
        token::authority = authority
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    ///CHECK: Source token account owner, can be system account or PDA
    pub owner: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub whitelist: Account<'info, WhitelistEntry>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = whitelist,
        associated_token::token_program = token_program
    )]
    pub whitelist_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
