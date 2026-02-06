use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey,
    pub deposit: u64,
    pub bump: u8,
}