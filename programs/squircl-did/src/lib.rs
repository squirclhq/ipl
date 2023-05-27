use anchor_lang::prelude::*;
use constants::{
    DID_LENGTH, DISCRIMINATOR_LENGTH, ETH_ADDRESS_LENGTH, ETH_SIGNATURE_LENGTH, I64_LENGTH,
    SOL_ADDRESS_LENGTH, SOL_SIGNATURE_LENGTH, U8_LENGTH,
};

pub mod constants;
pub mod utils;

declare_id!("AF4ChbnZ2DfGTHdNxHkmdrZkvRBbHYq1WcmnLiFRxwkZ");

#[program]
pub mod squircl_did {
    use crate::utils::{verify_eth_signature, verify_sol_signature};

    use super::*;

    pub fn create_did(ctx: Context<CreateDID>, params: CreateDIDParams) -> Result<()> {
        let did = &mut ctx.accounts.did;

        let clock: Clock = Clock::get()?;

        // verify the signature based on controller chain

        match params.controller_chain {
            Chain::Solana => {
                require!(
                    params.controller_signature.len() == SOL_SIGNATURE_LENGTH,
                    IdentityErrorCode::InvalidSignatureLength
                );

                if !(verify_sol_signature(
                    params.controller_address.clone(),
                    params.controller_signature.clone(),
                    params.controller_nonce,
                )) {
                    return Err(IdentityErrorCode::InvalidSignature.into());
                }
            }
            Chain::EVM => {
                require!(
                    params.controller_signature.len() == ETH_SIGNATURE_LENGTH,
                    IdentityErrorCode::InvalidSignatureLength
                );

                match verify_eth_signature(
                    params.controller_address.clone(),
                    params.controller_signature.clone(),
                    params.controller_nonce,
                ) {
                    Ok(verified) => {
                        if !verified {
                            return Err(IdentityErrorCode::InvalidSignature.into());
                        }
                    }
                    Err(_) => {
                        return Err(IdentityErrorCode::InvalidSignature.into());
                    }
                }
            }
        }

        did.set_inner(DID::new(
            params.did_str,
            clock.unix_timestamp,
            clock.unix_timestamp,
            Address::new(
                params.controller_address,
                clock.unix_timestamp,
                params.controller_chain,
                params.controller_signature,
                Role::Controller,
                params.controller_nonce,
            ),
        ));

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateDIDParams {
    pub did_str: String,
    pub controller_address: String,
    pub controller_signature: String,
    pub controller_chain: Chain,
    pub controller_nonce: i64,
}

#[derive(Accounts)]
#[instruction(params: CreateDIDParams)]
pub struct CreateDID<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"did".as_ref(), params.did_str.as_ref()],
        payer = payer,
        bump,
        space = DID::LEN_WITHOUT_ADDRESS + (match params.controller_chain {
            Chain::Solana => Address::SOL_LEN,
            Chain::EVM => Address::ETH_LEN
        })
    )]
    pub did: Account<'info, DID>,

    pub system_program: Program<'info, System>,
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
    pub nonce: i64,
}

impl Address {
    pub const ETH_LEN: usize = ETH_ADDRESS_LENGTH
        + I64_LENGTH
        + Chain::LEN
        + ETH_SIGNATURE_LENGTH
        + Role::LEN
        + I64_LENGTH;

    pub const SOL_LEN: usize = SOL_ADDRESS_LENGTH
        + I64_LENGTH
        + Chain::LEN
        + SOL_SIGNATURE_LENGTH
        + Role::LEN
        + I64_LENGTH;

    pub fn new(
        address: String,
        added_at: i64,
        chain: Chain,
        signature: String,
        role: Role,
        nonce: i64,
    ) -> Self {
        Self {
            address,
            added_at,
            chain,
            signature,
            role,
            nonce,
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
}
