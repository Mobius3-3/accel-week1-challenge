use anchor_lang::prelude::*;

#[account]
pub struct WhitelistEntry {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub max_amount: u64,
    pub bump: u8,
}

impl WhitelistEntry {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}