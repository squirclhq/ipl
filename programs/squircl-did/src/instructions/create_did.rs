use crate::errors::IdentityErrorCode;
use crate::state::{Address, Did, EthSig, Role, SolSig};
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
    ethSig: EthSig,
) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

    // let eth_address_binding = bs58::decode(eth_address_base58).into_vec().unwrap();
    // let eth_address = eth_address_binding.as_slice();
    // let sig_binding = bs58::decode(sig_base58).into_vec().unwrap();
    // let sig = sig_binding.as_slice();

    // msg!("eth_address: {:?}", eth_address);
    // msg!("sig: {:?}", sig);

    // let eth_address_hex = format!("0x{}", hex::encode(eth_address));

    let eth_address_hex = ethSig.get_eth_address_hex();

    let new_did_message = get_default_create_message(eth_address_hex.clone());

    ethSig.verify(&ix, new_did_message)?;

    // msg!("new_did_message: {:?}", new_did_message);

    // let msg_binding = get_ethereum_message_hash(new_did_message);

    // let msg = msg_binding.as_slice();

    // msg!("msg: {:?}", msg);

    // match verify_secp256k1_ix(&ix, eth_address, &msg, sig, recovery_id) {
    //     Ok(()) => {
    //         msg!("signature verified");
    //     }
    //     Err(_) => {
    //         msg!("signature not verified root");
    //         return Err(IdentityErrorCode::InvalidSignature.into());
    //     }
    // }

    // convert the ethereum signature to a hex string (should contain the recovery id)

    let sig_hex = ethSig.get_sig_hex();

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
    solSig: SolSig,
) -> Result<()> {
    let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let new_did_message = get_default_create_message(solSig.address_base58.clone());

    msg!("new_did_message: {:?}", new_did_message);

    solSig.verify(&ix, new_did_message)?;

    did.set_inner(Did::new_sol(
        did_str,
        clock.clone(),
        Address::new_sol(
            solSig.address_base58,
            clock.unix_timestamp,
            solSig.sig_base58,
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
