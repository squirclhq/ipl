use anchor_lang::prelude::*;

#[error_code]
pub enum IdentityErrorCode {
    #[msg("Invalid signature length")]
    InvalidSignatureLength,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid addres")]
    InvalidAddress,
    #[msg("Address already exists")]
    AddressAlreadyExists,
    #[msg("Address doesn't have enough permissions")]
    AddressDoesntHaveEnoughPermissions,
    #[msg("Cannot remove controller address")]
    CannotRemoveControllerAddress,
}
