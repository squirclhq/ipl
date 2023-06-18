use crate::state::{Address, Did, EthSig, Role, SolSig};
use crate::utils::get_default_create_message;
use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        instruction::Instruction,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

pub fn create_did_evm_ix(
    ctx: Context<CreateDIDEVM>,
    did_str: String,
    eth_sig: EthSig,
) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

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

    Ok(())
}

pub fn create_did_sol_ix(
    ctx: Context<CreateDIDSOL>,
    did_str: String,
    sol_sig: SolSig,
) -> Result<()> {
    let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let new_did_message = get_default_create_message(sol_sig.address_base58.clone());

    msg!("new_did_message: {:?}", new_did_message);

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

    Ok(())
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
