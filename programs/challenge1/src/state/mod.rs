use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub balance: u64,
    pub whitelisted: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VaultConfig {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub vault_ta: Pubkey,
    pub bump: u8,
}
