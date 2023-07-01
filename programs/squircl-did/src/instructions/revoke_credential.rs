use anchor_lang::prelude::*;

use crate::{
    errors::CredentialErrorCode,
    state::{Credential, Did},
};

pub fn revoke_credential_handler() -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(credential_id: String)]
pub struct RevokeCredential<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: The subject of the credential
    pub subject: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [Credential::SEED_PREFIX.as_bytes(), issuer_did.did.as_bytes(), subject_did.did.as_bytes(), credential_id.as_bytes()],
        bump,
        constraint = credential.is_revokable @CredentialErrorCode::CredentialIsNotRevokable,
        close = payer
    )]
    pub credential: Account<'info, Credential>,
    pub issuer_did: Account<'info, Did>,
    pub subject_did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
}
