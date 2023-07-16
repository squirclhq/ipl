use anchor_lang::prelude::*;

#[error_code]
pub enum SquirclErrorCode {
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
    #[msg("Expiry cannot be in the past")]
    ExpiryCannotBeInThePast,
    #[msg("Credential is not mutable")]
    CredentialIsNotMutable,
    #[msg("Credential i not revokable")]
    CredentialIsNotRevokable,
    #[msg("Address does not exist in DID")]
    AddressDoesNotExistInDID,
    #[msg("Nonce expired")]
    NonceExpired,
}
