use anchor_lang::prelude::*;

#[account]
pub struct WhitelistEntry {
    pub owner: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
    pub bump: u8,
}