use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Arithmetic Overflow")]
    ArithmeticError,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
    #[msg("Required Memo Not Found")]
    MemoNotFound,
    #[msg("Invalid Memo Format")]
    InvalidMemo,
    #[msg("Required Transfer Not Found")]
    TransferNotFound,
}
