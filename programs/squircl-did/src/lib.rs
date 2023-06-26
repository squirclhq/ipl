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

    pub fn create_did(ctx: Context<CreateDID>, did_str: String, sig: Sig) -> Result<()> {
        create_did_ix(ctx, did_str, sig)
    }

    pub fn add_addres(
        ctx: Context<AddAddress>,
        _did_str: String,
        new_address_sig: Sig,
        controller_sig: Sig,
    ) -> Result<()> {
        add_address_ix(ctx, new_address_sig, controller_sig)
    }
}
