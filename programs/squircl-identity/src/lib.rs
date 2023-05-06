use anchor_lang::prelude::*;

declare_id!("GQKH5unUGcEffQQCNNgQWhuijza6N62aQ1hTMu6VbAox");

#[program]
pub mod squircl_identity {
    use super::*;

    pub fn issue_credential(
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
            IdentityErrorCode::ExpiryCannotBeInThePast
        );

        credential.set_inner(Credential {
            issuer: *ctx.accounts.issuer.key,
            subject: *ctx.accounts.subject.key,
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

    pub fn update_credential(
        ctx: Context<UpdateCredential>,
        _credential_id: String,
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
            IdentityErrorCode::ExpiryCannotBeInThePast
        );

        credential.uri = uri;
        credential.credential_hash = credential_hash;
        credential.expires_at = expires_at;
        credential.is_mutable = is_mutable;
        credential.is_revokable = is_revokable;

        Ok(())
    }

    pub fn revoke_credential(
        _ctx: Context<RevokeCredential>,
        _credential_id: String,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(credential_id: String, uri: String, credential_hash: String)]
pub struct IssueCredential<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    /// CHECK: The subject of the credential
    pub subject: AccountInfo<'info>,
    #[account(
        init,
        seeds = [Credential::SEED_PREFIX.as_bytes(), issuer.key.as_ref(), subject.key.as_ref(), credential_id.as_bytes()],
        bump,
        payer = issuer,
        space = Credential::LEN_BASE + (4 + (4 *credential_id.len())) + (4 + (4 * uri.len())) + (4 + (4 * credential_hash.len()))
    )]
    pub credential: Account<'info, Credential>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(credential_id: String, uri: String, credential_hash: String)]
pub struct UpdateCredential<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    /// CHECK: The subject of the credential
    pub subject: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [Credential::SEED_PREFIX.as_bytes(), issuer.key.as_ref(), subject.key.as_ref(), credential_id.as_bytes()],
        bump,
        has_one = issuer,
        has_one = subject,
        constraint = credential.is_mutable && credential.issuer == *issuer.key @IdentityErrorCode::CredentialIsNotMutable,
    )]
    pub credential: Account<'info, Credential>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(credential_id: String)]
pub struct RevokeCredential<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    /// CHECK: The subject of the credential
    pub subject: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [Credential::SEED_PREFIX.as_bytes(), issuer.key.as_ref(), subject.key.as_ref(), credential_id.as_bytes()],
        bump,
        has_one = issuer,
        has_one = subject,
        constraint = credential.is_revokable && credential.issuer == *issuer.key @IdentityErrorCode::CredentialIsNotRevokable,
        close = issuer
    )]
    pub credential: Account<'info, Credential>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Credential {
    pub issuer: Pubkey,
    pub subject: Pubkey,
    pub issued_at: i64,
    pub is_mutable: bool,
    pub is_revokable: bool,
    pub expires_at: Option<i64>,
    pub credential_id: String,
    pub uri: String,
    pub credential_hash: String,
}

impl Credential {
    const SEED_PREFIX: &'static str = "credential";

    const LEN_BASE: usize = 8 // discriminator
                            + 32 // issuer
                            + 32 // subject
                            + 8 // issued_at
                            + 1 // is_mutable
                            + 1 // is_revokable
                            + 8; // expires_at
}

#[error_code]
pub enum IdentityErrorCode {
    #[msg("Expiry cannot be in the past")]
    ExpiryCannotBeInThePast,
    #[msg("Credential is not mutable")]
    CredentialIsNotMutable,
    #[msg("Credential is not revokable")]
    CredentialIsNotRevokable,
}
