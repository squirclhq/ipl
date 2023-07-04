use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

declare_id!("AF4ChbnZ2DfGTHdNxHkmdrZkvRBbHYq1WcmnLiFRxwkZ");

#[program]
pub mod squircl_did {

    use super::*;

    pub fn create_did(ctx: Context<CreateDID>, did_str: String, sig: Sig) -> Result<()> {
        create_did_ix(ctx, did_str, sig)
    }

    pub fn add_address(
        ctx: Context<AddAddress>,
        _did_str: String,
        new_address_sig: Sig,
        controller_sig: Sig,
    ) -> Result<()> {
        add_address_ix(ctx, new_address_sig, controller_sig)
    }

    pub fn remove_address(
        ctx: Context<RemoveAddress>,
        _did_str: String,
        address_chain: Chain,
        address: String,
        remover_sig: Sig,
    ) -> Result<()> {
        remove_address_ix(ctx, address_chain, address, remover_sig)
    }

    pub fn issue_credential(
        ctx: Context<IssueCredential>,
        credential_id: String,
        uri: String,
        credential_hash: String,
        is_mutable: bool,
        is_revokable: bool,
        expires_at: Option<i64>,
        issuer_sig: Sig,
    ) -> Result<()> {
        issue_credential_handler(
            ctx,
            credential_id,
            issuer_sig,
            uri,
            credential_hash,
            is_mutable,
            is_revokable,
            expires_at,
        )
    }

    pub fn update_credential(
        ctx: Context<UpdateCredential>,
        credential_id: String,
        uri: String,
        credential_hash: String,
        is_mutable: bool,
        is_revokable: bool,
        expires_at: Option<i64>,
        issuer_sig: Sig,
    ) -> Result<()> {
        update_credential_handler(
            ctx,
            credential_id,
            issuer_sig,
            uri,
            credential_hash,
            is_mutable,
            is_revokable,
            expires_at,
        )
    }

    pub fn revoke_credential(
        _ctx: Context<RevokeCredential>,
        _credential_id: String,
    ) -> Result<()> {
        revoke_credential_handler()
    }
}
