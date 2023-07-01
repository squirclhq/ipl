use anchor_lang::prelude::*;

use crate::constants::{BOOL_LENGTH, DISCRIMINATOR_LENGTH, I64_LENGTH, PUBLIC_KEY_LENGTH};

#[account]
pub struct Credential {
    pub issuer_did: String,
    pub subject_did: String,
    pub issued_at: i64,
    pub is_mutable: bool,
    pub is_revokable: bool,
    pub expires_at: Option<i64>,
    pub credential_id: String,
    pub uri: String,
    pub credential_hash: String,
}

impl Credential {
    pub const SEED_PREFIX: &'static str = "credential";

    pub const LEN_BASE: usize = DISCRIMINATOR_LENGTH // discriminator
                                + PUBLIC_KEY_LENGTH // issuer
                                + PUBLIC_KEY_LENGTH // subject
                                + I64_LENGTH // issued_at
                                + BOOL_LENGTH // is_mutable
                                + BOOL_LENGTH // is_revokable
                                + I64_LENGTH; // expires_at
}
