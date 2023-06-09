use crate::errors::IdentityErrorCode;
use crate::state::{Address, Chain, Did, Role};
use crate::utils::{
    get_default_create_message, get_ethereum_message_hash, verify_ed25519_ix, verify_secp256k1_ix,
};
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
    eth_address_base58: String,
    sig_base58: String,
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

pub fn create_did_sol_ix(
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
