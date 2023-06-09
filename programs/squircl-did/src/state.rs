use crate::constants::{
    DID_LENGTH, DISCRIMINATOR_LENGTH, ETH_ADDRESS_LENGTH, ETH_SIGNATURE_LENGTH, I64_LENGTH,
    SOL_ADDRESS_LENGTH, SOL_SIGNATURE_LENGTH, U8_LENGTH,
};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Chain {
    Solana = 0,
    EVM = 1,
}

impl Chain {
    pub const LEN: usize = U8_LENGTH;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Role {
    Controller = 0,     // Controller, can add/remove addresses, cannot be removed
    Admin = 1,          // Controller, can add/remove addresses, can be removed
    Assertion = 2,      // Can be used to decrypt protected credentials, and remove proofs
    Authentication = 3, // Addresses tied to the DID, can only remove themselves :)
}

impl Role {
    pub const LEN: usize = U8_LENGTH;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Address {
    pub address: String,
    pub added_at: i64,
    pub chain: Chain,
    pub signature: String,
    pub role: Role,
    // pub nonce: [u8; 32],
}

impl Address {
    pub const ETH_LEN: usize =
        ETH_ADDRESS_LENGTH + I64_LENGTH + Chain::LEN + ETH_SIGNATURE_LENGTH + Role::LEN;
    // + U8_LENGTH * 32;

    pub const SOL_LEN: usize =
        SOL_ADDRESS_LENGTH + I64_LENGTH + Chain::LEN + SOL_SIGNATURE_LENGTH + Role::LEN;
    // + U8_LENGTH * 32;

    pub fn new_eth(
        address: String,
        added_at: i64,
        signature: String,
        role: Role,
        // nonce: [u8; 32],
    ) -> Self {
        Self {
            address,
            added_at,
            chain: Chain::EVM,
            signature,
            role,
            // nonce,
        }
    }

    pub fn new_sol(
        address: String,
        added_at: i64,
        signature: String,
        role: Role,
        // nonce: [u8; 32],
    ) -> Self {
        Self {
            address,
            added_at,
            chain: Chain::Solana,
            signature,
            role,
            // nonce,
        }
    }
}

#[account]
pub struct Did {
    pub did: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub controller_multisig_threshold: u8, // Default 1
    pub addresses: Vec<Address>,
}

impl Did {
    pub const LEN_WITHOUT_ADDRESS: usize =
        DISCRIMINATOR_LENGTH + DID_LENGTH + I64_LENGTH + I64_LENGTH + U8_LENGTH;

    pub fn new(did: String, clock: Clock, controller: Address) -> Self {
        Self {
            did,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
            controller_multisig_threshold: 1,
            addresses: vec![controller],
        }
    }
}
