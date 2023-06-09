use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        instruction::Instruction,
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

#[program]
pub mod squircl_did {

    use crate::{
        constants::{get_default_create_message, get_ethereum_message_hash},
        utils::{verify_ed25519_ix, verify_secp256k1_ix},
    };

    use super::*;

    pub fn create_did_evm(
        ctx: Context<CreateDIDEVM>,
        did_str: String,
        eth_address_base58: String,
        sig_base58: String,
        msg_base58: String,
        recovery_id: u8,
    ) -> Result<()> {
        let did = &mut ctx.accounts.did;

        let clock: Clock = Clock::get()?;

        let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

        let eth_address_binding = bs58::decode(eth_address_base58).into_vec().unwrap();
        let eth_address = eth_address_binding.as_slice();
        let sig_binding = bs58::decode(sig_base58).into_vec().unwrap();
        let sig = sig_binding.as_slice();

        // msg!("eth_address: {:?}", eth_address);
        // msg!("sig: {:?}", sig);

        let eth_address_hex = format!("0x{}", hex::encode(eth_address));

        let new_did_message = get_default_create_message(eth_address_hex.clone());

        // msg!("new_did_message: {:?}", new_did_message);

        let msg_binding = get_ethereum_message_hash(new_did_message);

        let msg = msg_binding.as_slice();

        // msg!("msg: {:?}", msg);

        match verify_secp256k1_ix(&ix, eth_address, &msg, sig, recovery_id) {
            Ok(()) => {
                msg!("signature verified");
            }
            Err(_) => {
                msg!("signature not verified root");
                return Err(IdentityErrorCode::InvalidSignature.into());
            }
        }

        // convert the ethereum signature to a hex string (should contain the recovery id)

        let sig_hex = format!("0x{}", hex::encode([sig, &[recovery_id + 27]].concat()));

        did.set_inner(Did::new(
            did_str,
            clock.clone(),
            Address::new_eth(
                eth_address_hex,
                clock.unix_timestamp,
                sig_hex,
                Role::Controller,
            ),
        ));

        Ok(())
    }

    pub fn create_did_sol(
        ctx: Context<CreateDIDSOL>,
        did_str: String,
        sol_address_base58: String,
        sig_base58: String,
        msg_base58: String,
    ) -> Result<()> {
        let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

        let did = &mut ctx.accounts.did;

        let clock: Clock = Clock::get()?;

        let sol_address_binding = bs58::decode(sol_address_base58.clone()).into_vec().unwrap();
        let sol_address = sol_address_binding.as_slice();
        // let msg_binding: Vec<u8> = bs58::decode(msg_base58).into_vec().unwrap();
        // let msg = msg_binding.as_slice();
        let sig_binding = bs58::decode(sig_base58.clone()).into_vec().unwrap();
        let sig = sig_binding.as_slice();

        let new_did_message = get_default_create_message(sol_address_base58.clone());

        msg!("new_did_message: {:?}", new_did_message);

        let msg: &[u8] = new_did_message.as_bytes();

        msg!("sol_address: {:?}", sol_address);
        msg!("sig: {:?}", sig);
        msg!("msg: {:?}", msg);

        match verify_ed25519_ix(&ix, sol_address, msg, sig) {
            Ok(()) => {
                msg!("signature verified");
            }
            Err(_) => {
                msg!("signature not verified root");
                return Err(IdentityErrorCode::InvalidSignature.into());
            }
        }

        did.set_inner(Did::new(
            did_str,
            clock.clone(),
            Address::new_sol(
                sol_address_base58,
                clock.unix_timestamp,
                sig_base58,
                Role::Controller,
            ),
        ));

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
#[instruction(did_str: String)]
pub struct CreateDIDEVM<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        payer = payer,
        bump,
        space = Did::LEN_WITHOUT_ADDRESS + Address::ETH_LEN
    )]
    pub did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(did_str: String)]
pub struct CreateDIDSOL<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        payer = payer,
        bump,
        space = Did::LEN_WITHOUT_ADDRESS + Address::ETH_LEN
    )]
    pub did: Account<'info, Did>,
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
    const LEN_WITHOUT_ADDRESS: usize =
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

#[error_code]
pub enum IdentityErrorCode {
    #[msg("Invalid signature length")]
    InvalidSignatureLength,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid addres")]
    InvalidAddress,
}
