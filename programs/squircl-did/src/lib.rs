use crate::utils::verify_secp256k1_ix;
use anchor_lang::{
    prelude::*,
    solana_program::secp256k1_recover::SECP256K1_SIGNATURE_LENGTH,
    solana_program::{
        hash::hash,
        instruction::Instruction,
        secp256k1_program::ID as SECP256K1_ID,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};
use constants::{
    DID_LENGTH, DISCRIMINATOR_LENGTH, ETH_ADDRESS_LENGTH, ETH_SIGNATURE_LENGTH, I64_LENGTH,
    SOL_ADDRESS_LENGTH, SOL_SIGNATURE_LENGTH, U8_LENGTH,
};

pub mod constants;
pub mod utils;

declare_id!("AF4ChbnZ2DfGTHdNxHkmdrZkvRBbHYq1WcmnLiFRxwkZ");

// #[derive(AnchorSerialize, AnchorDeserialize)]
// pub struct Secp256k1RawSignature {
//     pub signature: [u8; 64],
//     pub recovery_id: u8,
// }

#[program]
pub mod squircl_did {

    use crate::utils::verify_secp256k1_ix;

    use super::*;

    pub fn create_did(
        ctx: Context<CreateDID>,
        // did_str: String,
        // eth_address: [u8; 20],
        // msg: [u8; 64],
        // sig: [u8; 64],
        // recovery_id: u8,
        eth_address_base58: String,
        sig_base58: String,
        msg_base58: String,
        recovery_id: u8,
    ) -> Result<()> {
        // let did = &mut ctx.accounts.did;

        // let clock: Clock = Clock::get()?;

        let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

        let eth_address_binding = bs58::decode(eth_address_base58).into_vec().unwrap();
        let eth_address = eth_address_binding.as_slice();
        let msg: Vec<u8> = bs58::decode(msg_base58).into_vec().unwrap();
        let sig_binding = bs58::decode(sig_base58).into_vec().unwrap();
        let sig = sig_binding.as_slice();

        msg!("eth_address: {:?}", eth_address);
        msg!("sig: {:?}", sig);
        msg!("msg: {:?}", msg);

        match verify_secp256k1_ix(&ix, eth_address, &msg, sig, recovery_id) {
            Ok(()) => {
                msg!("signature verified");
            }
            Err(_) => {
                msg!("signature not verified root");
                return Err(IdentityErrorCode::InvalidSignature.into());
            }
        }

        // msg!("signature: {}", signature);

        // let sig_bytes = bs58::decode(signature).into_vec().unwrap();

        // msg!("sig_bytes: {:?}", sig_bytes);

        // // it is an ethereum signature, derive the actual signature and recover id

        // let signature: [u8; 64] = sig_bytes[..SECP256K1_SIGNATURE_LENGTH].try_into().unwrap();
        // let recovery_id = sig_bytes[SECP256K1_SIGNATURE_LENGTH] - 27;

        // msg!("signature: {:?}", signature);
        // msg!("recovery_id: {:?}", recovery_id);

        // match verify_eth_signature(message, signature, public_key) {
        //     Ok(verified) => {
        //         msg!("verified: {}", verified);
        //         if !verified {
        //             return Err(IdentityErrorCode::InvalidSignature.into());
        //         }
        //     }
        //     Err(_) => {
        //         return Err(IdentityErrorCode::InvalidSignature.into());
        //     }
        // }

        // verify the signature based on controller chain

        // match controller_chain {
        //     Chain::Solana => {
        //         // require!(
        //         //     controller_signature.len() == SOL_SIGNATURE_CHARS,
        //         //     IdentityErrorCode::InvalidSignatureLength
        //         // );

        //         msg!("controller_address: {}", controller_address);

        //         // if !(verify_sol_signature(
        //         //     controller_address.clone(),
        //         //     controller_signature.clone(),
        //         //     // controller_nonce,
        //         // )) {
        //         //     return Err(IdentityErrorCode::InvalidSignature.into());
        //         // }

        //         match verify_sol_signature(
        //             controller_address.clone(),
        //             controller_signature.clone(),
        //             // controller_nonce,
        //         ) {
        //             Ok(verified) => {
        //                 if !verified {
        //                     return Err(IdentityErrorCode::InvalidSignature.into());
        //                 }
        //             }
        //             Err(_) => {
        //                 return Err(IdentityErrorCode::InvalidSignature.into());
        //             }
        //         }

        //         msg!("signature verified")
        //     }
        //     Chain::EVM => {
        //         require!(
        //             controller_signature.len() == ETH_SIGNATURE_CHARS,
        //             IdentityErrorCode::InvalidSignatureLength
        //         );

        //         match verify_eth_signature(
        //             controller_address.clone(),
        //             controller_signature.clone(),
        //             // controller_nonce,
        //         ) {
        //             Ok(verified) => {
        //                 if !verified {
        //                     return Err(IdentityErrorCode::InvalidSignature.into());
        //                 }
        //             }
        //             Err(_) => {
        //                 return Err(IdentityErrorCode::InvalidSignature.into());
        //             }
        //         }
        //     }
        // }

        // did.set_inner(DID::new(
        //     did_str,
        //     clock.unix_timestamp,
        //     clock.unix_timestamp,
        //     Address::new(
        //         controller_address,
        //         clock.unix_timestamp,
        //         controller_chain,
        //         controller_signature,
        //         Role::Controller,
        //         // controller_nonce,
        //     ),
        // ));

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateDIDParams {
    pub did_str: String,
    pub controller_address: String,
    pub controller_signature: [u8; 64],
    pub contoller_recovery_id: u8,
    pub controller_chain: Chain,
    // pub controller_nonce: [u8; 32],
}

#[derive(Accounts)]
#[instruction(did_str: String, controller_address: String,  controller_signature: String)]
pub struct CreateDID<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // #[account(
    //     init,
    //     seeds = [b"did".as_ref(), &hash(did_str.as_bytes()).to_bytes()],
    //     payer = payer,
    //     bump,
    //     space = DID::LEN_WITHOUT_ADDRESS + Address::ETH_LEN
    // )]
    // pub did: Account<'info, DID>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}

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

    pub fn new(
        address: String,
        added_at: i64,
        chain: Chain,
        signature: String,
        role: Role,
        // nonce: [u8; 32],
    ) -> Self {
        Self {
            address,
            added_at,
            chain,
            signature,
            role,
            // nonce,
        }
    }
}

#[account]
pub struct DID {
    pub did: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub controller_multisig_threshold: u8, // Default 1
    pub addresses: Vec<Address>,
}

impl DID {
    const LEN_WITHOUT_ADDRESS: usize =
        DISCRIMINATOR_LENGTH + DID_LENGTH + I64_LENGTH + I64_LENGTH + U8_LENGTH;

    pub fn new(did: String, created_at: i64, updated_at: i64, controller: Address) -> Self {
        Self {
            did,
            created_at,
            updated_at,
            controller_multisig_threshold: 1,
            addresses: vec![controller],
        }
    }
}

#[error_code]
pub enum IdentityErrorCode {
    #[msg("Invalid signature length")]
    InvalidSignatureLength,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid addres")]
    InvalidAddress,
}
