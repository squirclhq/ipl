use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

use crate::{
    errors::DidErrorCode,
    utils::{get_ethereum_message_hash, verify_ed25519_ix, verify_secp256k1_ix},
};

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
                return Err(DidErrorCode::InvalidSignature.into());
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
                return Err(DidErrorCode::InvalidSignature.into());
            }
        };

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Sig {
    Eth { eth_sig: EthSig, index: u8 },
    Sol { sol_sig: SolSig, index: u8 },
}
