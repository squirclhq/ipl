use anchor_lang::prelude::*;

#[error_code]
pub enum IdentityErrorCode {
    #[msg("Invalid signature length")]
    InvalidSignatureLength,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid addres")]
    InvalidAddress,
}