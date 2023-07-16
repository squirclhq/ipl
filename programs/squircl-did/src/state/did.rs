use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

use crate::{
    constants::{
        DID_LENGTH, DISCRIMINATOR_LENGTH, ETH_ADDRESS_LENGTH, ETH_SIGNATURE_LENGTH, I64_LENGTH,
        SOL_ADDRESS_LENGTH, SOL_SIGNATURE_LENGTH, U8_LENGTH,
    },
    errors::SquirclErrorCode,
    utils::{get_ethereum_message_hash, verify_ed25519_ix, verify_secp256k1_ix},
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
        role: Role,
        // nonce: [u8; 32],
    ) -> Self {
        Self {
            address,
            added_at,
            chain: Chain::EVM,
            role,
            // nonce,
        }
    }

    pub fn new_sol(
        address: String,
        added_at: i64,
        role: Role,
        // nonce: [u8; 32],
    ) -> Self {
        Self {
            address,
            added_at,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EthSig {
    pub address_base58: String,
    pub sig_base58: String,
    pub recovery_id: u8,
}

impl EthSig {
    pub fn get_eth_address_vec(&self) -> Vec<u8> {
        bs58::decode(self.address_base58.clone())
            .into_vec()
            .unwrap()
    }

    pub fn get_sig_vec(&self) -> Vec<u8> {
        bs58::decode(self.sig_base58.clone()).into_vec().unwrap()
    }

    pub fn get_eth_address_hex(&self) -> String {
        format!("0x{}", hex::encode(self.get_eth_address_vec().as_slice()))
    }

    pub fn get_sig_hex(&self) -> String {
        format!(
            "0x{}",
            hex::encode([self.get_sig_vec().as_slice(), &[self.recovery_id + 27]].concat())
        )
    }

    pub fn verify(&self, ix: &Instruction, msg_string: String) -> Result<()> {
        let eth_address_binding = self.get_eth_address_vec();
        let eth_address = eth_address_binding.as_slice();
        let sig_binding = self.get_sig_vec();
        let sig = sig_binding.as_slice();
        let msg_binding = get_ethereum_message_hash(msg_string);
        let msg = msg_binding.as_slice();

        match verify_secp256k1_ix(&ix, eth_address, &msg, sig, self.recovery_id) {
            Ok(()) => {
                msg!("signature verified");
            }
            Err(_) => {
                msg!("signature not verified root");
                return Err(SquirclErrorCode::InvalidSignature.into());
            }
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SolSig {
    pub address_base58: String,
    pub sig_base58: String,
}

impl SolSig {
    pub fn get_sol_address_vec(&self) -> Vec<u8> {
        bs58::decode(self.address_base58.clone())
            .into_vec()
            .unwrap()
    }
    pub fn get_sig_vec(&self) -> Vec<u8> {
        bs58::decode(self.sig_base58.clone()).into_vec().unwrap()
    }
    pub fn verify(&self, ix: &Instruction, msg_string: String) -> Result<()> {
        let msg = msg_string.as_bytes();
        let sol_address_binding = self.get_sol_address_vec();
        let sol_address = sol_address_binding.as_slice();
        let sig_binding = self.get_sig_vec();
        let sig = sig_binding.as_slice();

        match verify_ed25519_ix(&ix, sol_address, msg, sig) {
            Ok(()) => {
                msg!("signature verified");
            }
            Err(_) => {
                msg!("signature not verified root");
                return Err(SquirclErrorCode::InvalidSignature.into());
            }
        };

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Sig {
    Eth { eth_sig: EthSig, index: u8 },
    Sol { sol_sig: SolSig, index: u8 },
}
