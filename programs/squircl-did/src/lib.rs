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
        ethSig: EthSig,
    ) -> Result<()> {
        create_did_evm_ix(ctx, did_str, ethSig)
    }

    pub fn create_did_sol(
        ctx: Context<CreateDIDSOL>,
        did_str: String,
        solSig: SolSig,
    ) -> Result<()> {
        create_did_sol_ix(ctx, did_str, solSig)
    }
}
