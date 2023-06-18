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

    pub fn create_did_evm(
        ctx: Context<CreateDIDEVM>,
        did_str: String,
        eth_sig: EthSig,
    ) -> Result<()> {
        create_did_evm_ix(ctx, did_str, eth_sig)
    }

    pub fn create_did_sol(
        ctx: Context<CreateDIDSOL>,
        did_str: String,
        sol_sig: SolSig,
    ) -> Result<()> {
        create_did_sol_ix(ctx, did_str, sol_sig)
    }

    pub fn add_address_evm(
        ctx: Context<AddAddressEVM>,
        _did_str: String,
        new_address_sig: EthSig,
        controller_sig: Sig,
    ) -> Result<()> {
        add_address_evm_ix(ctx, new_address_sig, controller_sig)
    }

    pub fn add_address_sol(
        ctx: Context<AddAddressSol>,
        _did_str: String,
        new_address_sig: SolSig,
        controller_sig: Sig,
    ) -> Result<()> {
        add_address_sol_ix(ctx, new_address_sig, controller_sig)
    }
}
