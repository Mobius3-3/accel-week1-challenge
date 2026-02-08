use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub vault_state_bump: u8,
    pub vault_authority_bump: u8,
}

impl VaultState {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1 + 1;
}