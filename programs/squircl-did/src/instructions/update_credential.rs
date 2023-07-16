use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

use crate::{
    errors::SquirclErrorCode,
    state::{Credential, Did, Sig},
    utils::get_default_update_credential_message,
};

pub fn update_credential_handler(
    ctx: Context<UpdateCredential>,
    credential_id: String,
    issuer_sig: Sig,
    uri: String,
    credential_hash: String,
    is_mutable: bool,
    is_revokable: bool,
    expires_at: Option<i64>,
) -> Result<()> {
    let clock: Clock = Clock::get()?;

    let timestamp_1_hour_ago = clock.unix_timestamp - 3600;

    let credential = &mut ctx.accounts.credential;
    let issuer_did = &ctx.accounts.issuer_did;
    let subject_did = &ctx.accounts.subject_did;

    require!(
        expires_at == None || expires_at.unwrap() > clock.unix_timestamp,
        SquirclErrorCode::ExpiryCannotBeInThePast
    );

    match issuer_sig {
        Sig::Eth {
            eth_sig,
            index,
            nonce,
        } => {
            require!(nonce > timestamp_1_hour_ago, SquirclErrorCode::NonceExpired);

            let controller_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let issue_credential_message = get_default_update_credential_message(
                &credential_id,
                &issuer_did.did,
                &subject_did.did,
                &uri,
                &credential_hash,
                nonce,
            );

            eth_sig.verify(&controller_address_sign_ix, issue_credential_message)?;

            let found_address = issuer_did
                .eth_addresses
                .iter()
                .find(|address| address.address == eth_sig.get_eth_address_hex());

            if !found_address.is_some() {
                return Err(SquirclErrorCode::AddressDoesNotExistInDID.into());
            }
        }

        Sig::Sol {
            sol_sig,
            index,
            nonce,
        } => {
            require!(nonce > timestamp_1_hour_ago, SquirclErrorCode::NonceExpired);

            let controller_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let issue_credential_message = get_default_update_credential_message(
                &credential_id,
                &issuer_did.did,
                &subject_did.did,
                &uri,
                &credential_hash,
                nonce,
            );

            sol_sig.verify(&controller_address_sign_ix, issue_credential_message)?;

            let found_address = issuer_did
                .sol_addresses
                .iter()
                .find(|address| address.address == sol_sig.address_base58);

            if !found_address.is_some() {
                return Err(SquirclErrorCode::AddressDoesNotExistInDID.into());
            }
        }
    }

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
        seeds = [Credential::SEED_PREFIX.as_bytes(), &hash::hash(issuer_did.did.as_bytes()).to_bytes(), &hash::hash(subject_did.did.as_bytes()).to_bytes(), credential_id.as_bytes()],
        bump,
        realloc = Credential::LEN_BASE + (4 + (4 * credential_id.len())) + (4 + (4 * uri.len())) + (4 + (4 * credential_hash.len())),
        realloc::payer = payer,
        realloc::zero = false,
        constraint = credential.is_mutable @SquirclErrorCode::CredentialIsNotMutable,
    )]
    pub credential: Account<'info, Credential>,
    pub issuer_did: Account<'info, Did>,
    pub subject_did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}
