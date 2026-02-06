use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Not whitelisted")]
    NotWhitelisted,

    #[msg("Amount exceeds limit")]
    AmountExceedsLimit,
}
