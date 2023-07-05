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
    utils::get_default_revoke_credential_message,
};

pub fn revoke_credential_handler(
    ctx: Context<RevokeCredential>,
    credential_id: String,
    issuer_sig: Sig,
) -> Result<()> {
    let issuer_did = &ctx.accounts.issuer_did;
    let subject_did = &ctx.accounts.subject_did;

    match issuer_sig {
        Sig::Eth { eth_sig, index } => {
            let controller_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let issue_credential_message = get_default_revoke_credential_message(
                &credential_id,
                &issuer_did.did,
                &subject_did.did,
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

        Sig::Sol { sol_sig, index } => {
            let controller_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let issue_credential_message = get_default_revoke_credential_message(
                &credential_id,
                &issuer_did.did,
                &subject_did.did,
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

    Ok(())
}

#[derive(Accounts)]
#[instruction(credential_id: String)]
pub struct RevokeCredential<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [Credential::SEED_PREFIX.as_bytes(), &hash::hash(issuer_did.did.as_bytes()).to_bytes(), &hash::hash(subject_did.did.as_bytes()).to_bytes(), credential_id.as_bytes()],
        bump,
        constraint = credential.is_revokable @SquirclErrorCode::CredentialIsNotRevokable,
        close = payer
    )]
    pub credential: Account<'info, Credential>,
    pub issuer_did: Account<'info, Did>,
    pub subject_did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}
