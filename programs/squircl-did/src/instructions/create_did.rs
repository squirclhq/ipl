use crate::state::{Address, Did, Role, Sig};
use crate::utils::get_default_create_message;
use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        instruction::Instruction,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

pub fn create_did_ix(ctx: Context<CreateDID>, did_str: String, sig: Sig) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    match sig {
        Sig::Eth { eth_sig, index } => {
            let ix: Instruction =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let eth_address_hex = eth_sig.get_eth_address_hex();

            let new_did_message = get_default_create_message(eth_address_hex.clone());

            eth_sig.verify(&ix, new_did_message)?;

            let sig_hex = eth_sig.get_sig_hex();

            did.set_inner(Did::new_eth(
                did_str,
                clock.clone(),
                Address::new_eth(
                    eth_address_hex,
                    clock.unix_timestamp,
                    sig_hex,
                    Role::Controller,
                ),
            ));
        }

        Sig::Sol { sol_sig, index } => {
            let ix: Instruction =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let new_did_message = get_default_create_message(sol_sig.address_base58.clone());

            sol_sig.verify(&ix, new_did_message)?;

            did.set_inner(Did::new_sol(
                did_str,
                clock.clone(),
                Address::new_sol(
                    sol_sig.address_base58,
                    clock.unix_timestamp,
                    sol_sig.sig_base58,
                    Role::Controller,
                ),
            ));
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(did_str: String)]
pub struct CreateDID<'info> {
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
#[instruction(did_str: String, sig: Sig)]
pub struct CreateDIDSOL<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        payer = payer,
        bump,
        space = Did::LEN_WITHOUT_ADDRESS + match sig {
            Sig::Eth { .. } => Address::ETH_LEN,
            Sig::Sol { .. } => Address::SOL_LEN,
        }
    )]
    pub did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}
