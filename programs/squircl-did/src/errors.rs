use anchor_lang::prelude::*;

#[error_code]
pub enum DidErrorCode {
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

#[error_code]
pub enum CredentialErrorCode {
    #[msg("Expiry cannot be in the past")]
    ExpiryCannotBeInThePast,
    #[msg("Credential is not mutable")]
    CredentialIsNotMutable,
    #[msg("Credential is not revokable")]
    CredentialIsNotRevokable,
}
