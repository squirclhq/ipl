use anchor_lang::prelude::*;

use crate::{
    errors::CredentialErrorCode,
    state::{Credential, Did},
};

pub fn update_credential_handler(
    ctx: Context<UpdateCredential>,
    uri: String,
    credential_hash: String,
    is_mutable: bool,
    is_revokable: bool,
    expires_at: Option<i64>,
) -> Result<()> {
    let clock: Clock = Clock::get()?;

    let credential = &mut ctx.accounts.credential;

    require!(
        expires_at == None || expires_at.unwrap() > clock.unix_timestamp,
        CredentialErrorCode::ExpiryCannotBeInThePast
    );

    credential.uri = uri;
    credential.credential_hash = credential_hash;
    credential.expires_at = expires_at;
    credential.is_mutable = is_mutable;
    credential.is_revokable = is_revokable;

    Ok(())
}

#[derive(Accounts)]
#[instruction(credential_id: String, uri: String, credential_hash: String)]
pub struct UpdateCredential<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [Credential::SEED_PREFIX.as_bytes(), issuer_did.did.as_bytes(), subject_did.did.as_bytes(), credential_id.as_bytes()],
        bump,
        realloc = Credential::LEN_BASE + (4 + (4 * credential_id.len())) + (4 + (4 * uri.len())) + (4 + (4 * credential_hash.len())),
        realloc::payer = payer,
        realloc::zero = false,
        constraint = credential.is_mutable @CredentialErrorCode::CredentialIsNotMutable,
    )]
    pub credential: Account<'info, Credential>,
    pub issuer_did: Account<'info, Did>,
    pub subject_did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
}
