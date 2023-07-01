use anchor_lang::prelude::*;

use crate::constants::{
    DID_LENGTH, DISCRIMINATOR_LENGTH, ETH_ADDRESS_LENGTH, ETH_SIGNATURE_LENGTH, I64_LENGTH,
    SOL_ADDRESS_LENGTH, SOL_SIGNATURE_LENGTH, U8_LENGTH,
};

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
pub enum Chain {
    EVM = 0,
    SOL = 1,
}

impl Chain {
    pub const LEN: usize = U8_LENGTH;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Address {
    pub address: String,
    pub added_at: i64,
    pub signature: String,
    pub chain: Chain,
    pub role: Role,
    // pub nonce: [u8; 32],
}

impl Address {
    pub const ETH_LEN: usize = ETH_ADDRESS_LENGTH + I64_LENGTH + ETH_SIGNATURE_LENGTH + Role::LEN;
    // + U8_LENGTH * 32;

    pub const SOL_LEN: usize = SOL_ADDRESS_LENGTH + I64_LENGTH + SOL_SIGNATURE_LENGTH + Role::LEN;
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
            signature,
            chain: Chain::EVM,
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
            signature,
            chain: Chain::SOL,
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
    pub eth_addresses: Vec<Address>,
    pub sol_addresses: Vec<Address>,
}

impl Did {
    pub const LEN_WITHOUT_ADDRESS: usize =
        DISCRIMINATOR_LENGTH + DID_LENGTH + I64_LENGTH + I64_LENGTH + U8_LENGTH;

    pub fn new_eth(did: String, clock: Clock, controller: Address) -> Self {
        Self {
            did,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
            eth_addresses: vec![controller],
            sol_addresses: vec![],
        }
    }

    pub fn new_sol(did: String, clock: Clock, controller: Address) -> Self {
        Self {
            did,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
            eth_addresses: vec![],
            sol_addresses: vec![controller],
        }
    }

    pub fn add_address_sol(&mut self, clock: Clock, address: Address) {
        self.updated_at = clock.unix_timestamp;
        self.sol_addresses.push(address);
    }

    pub fn add_address_eth(&mut self, clock: Clock, address: Address) {
        self.updated_at = clock.unix_timestamp;
        self.eth_addresses.push(address);
    }

    pub fn remove_address_sol(&mut self, clock: Clock, address: String) {
        self.updated_at = clock.unix_timestamp;
        self.sol_addresses.retain(|a| a.address != address);
    }

    pub fn remove_address_eth(&mut self, clock: Clock, address: String) {
        self.updated_at = clock.unix_timestamp;
        self.eth_addresses.retain(|a| a.address != address);
    }
}
