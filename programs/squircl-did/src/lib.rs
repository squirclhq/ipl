use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("AF4ChbnZ2DfGTHdNxHkmdrZkvRBbHYq1WcmnLiFRxwkZ");

#[program]
pub mod squircl_did {

    use super::*;

    pub fn create_did_evm(
        ctx: Context<CreateDIDEVM>,
        did_str: String,
        eth_address_base58: String,
        sig_base58: String,
        recovery_id: u8,
    ) -> Result<()> {
        create_did_evm_ix(ctx, did_str, eth_address_base58, sig_base58, recovery_id)
    }

    pub fn create_did_sol(
        ctx: Context<CreateDIDSOL>,
        did_str: String,
        sol_address_base58: String,
        sig_base58: String,
        msg_base58: String,
    ) -> Result<()> {
        create_did_sol_ix(ctx, did_str, sol_address_base58, sig_base58, msg_base58)
    }
}
