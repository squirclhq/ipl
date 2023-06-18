use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

use crate::{
    state::{Address, Did, EthSig, Role, SolSig},
    utils::{get_default_add_message_as_controller, get_default_add_message_as_new_address},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Sig {
    Eth { ethSig: EthSig },
    Sol { solSig: SolSig },
}

pub fn add_address_evm_ix(
    ctx: Context<AddAddressEVM>,
    new_address_sig: EthSig,
    controller_sig: Sig,
) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let new_address_sign_ix = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

    let new_address_hex = new_address_sig.get_eth_address_hex();

    let add_message_as_new_adress = get_default_add_message_as_new_address(new_address_hex.clone());

    new_address_sig.verify(&new_address_sign_ix, add_message_as_new_adress)?;

    match controller_sig {
        Sig::Eth { ethSig } => {
            let controller_address_sign_ix =
                load_instruction_at_checked(1, &ctx.accounts.ix_sysvar)?;

            let add_message_as_controller = get_default_add_message_as_controller(
                ethSig.get_eth_address_hex(),
                new_address_hex.clone(),
            );

            ethSig.verify(&controller_address_sign_ix, add_message_as_controller)?;
        }
        Sig::Sol { solSig } => {
            let controller_address_sign_ix =
                load_instruction_at_checked(1, &ctx.accounts.ix_sysvar)?;

            let add_message_as_controller = get_default_add_message_as_controller(
                solSig.address_base58.clone(),
                new_address_hex.clone(),
            );

            solSig.verify(&controller_address_sign_ix, add_message_as_controller)?;
        }
    }

    did.add_address_eth(
        clock.clone(),
        Address::new_eth(
            new_address_hex,
            clock.unix_timestamp,
            new_address_sig.get_sig_hex(),
            Role::Admin,
        ),
    );

    Ok(())
}

pub fn add_address_sol_ix(
    ctx: Context<AddAddressSol>,
    new_address_sig: SolSig,
    controller_sig: Sig,
) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let new_address_sign_ix = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

    let add_message_as_new_adress =
        get_default_add_message_as_new_address(new_address_sig.address_base58.clone());

    new_address_sig.verify(&new_address_sign_ix, add_message_as_new_adress)?;

    match controller_sig {
        Sig::Eth { ethSig } => {
            let add_message_as_controller = get_default_add_message_as_controller(
                ethSig.get_eth_address_hex(),
                new_address_sig.address_base58.clone(),
            );

            ethSig.verify(&new_address_sign_ix, add_message_as_controller)?;
        }
        Sig::Sol { solSig } => {
            let add_message_as_controller = get_default_add_message_as_controller(
                solSig.address_base58.clone(),
                new_address_sig.address_base58.clone(),
            );

            solSig.verify(&new_address_sign_ix, add_message_as_controller)?;
        }
    }

    did.add_address_sol(
        clock.clone(),
        Address::new_sol(
            new_address_sig.address_base58.clone(),
            clock.unix_timestamp,
            new_address_sig.sig_base58,
            Role::Admin,
        ),
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(did_str: String)]
pub struct AddAddressEVM<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        bump,
        realloc = Did::LEN_WITHOUT_ADDRESS + did.sol_addresses.len() as usize * Address::SOL_LEN + did.eth_addresses.len() as usize * Address::ETH_LEN + Address::ETH_LEN,
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(did_str: String)]
pub struct AddAddressSol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        bump,
        realloc = Did::LEN_WITHOUT_ADDRESS + did.sol_addresses.len() as usize * Address::SOL_LEN + did.eth_addresses.len() as usize * Address::ETH_LEN + Address::SOL_LEN,
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}
