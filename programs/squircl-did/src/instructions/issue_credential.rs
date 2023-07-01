use anchor_lang::prelude::*;

use crate::{
    errors::CredentialErrorCode,
    state::{Credential, Did},
};

pub fn issue_credential_handler(
    ctx: Context<IssueCredential>,
    credential_id: String,
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

    credential.set_inner(Credential {
        issuer_did: ctx.accounts.issuer_did.did.to_string(),
        subject_did: ctx.accounts.subject_did.did.to_string(),
        issued_at: clock.unix_timestamp,
        is_mutable,
        is_revokable,
        expires_at,
        credential_id,
        uri,
        credential_hash,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(credential_id: String, uri: String, credential_hash: String)]
pub struct IssueCredential<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [Credential::SEED_PREFIX.as_bytes(), issuer_did.did.as_bytes(), subject_did.did.as_bytes(), credential_id.as_bytes()],
        bump,
        payer = payer,
        space = Credential::LEN_BASE + (4 + (4 * credential_id.len())) + (4 + (4 * uri.len())) + (4 + (4 * credential_hash.len()))
    )]
    pub credential: Account<'info, Credential>,
    pub issuer_did: Account<'info, Did>,
    pub subject_did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
}
