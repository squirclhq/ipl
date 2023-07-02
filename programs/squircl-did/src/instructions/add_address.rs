use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

use crate::{
    errors::SquirclErrorCode,
    state::{Address, Did, Role, Sig},
    utils::{get_default_add_message_as_controller, get_default_add_message_as_new_address},
};

pub fn add_address_ix(
    ctx: Context<AddAddress>,
    new_address_sig: Sig,
    controller_sig: Sig,
) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    let new_address = if let Sig::Eth { eth_sig, index: _ } = &new_address_sig {
        Address::new_eth(
            eth_sig.get_eth_address_hex(),
            clock.unix_timestamp,
            eth_sig.get_sig_hex(),
            Role::Admin,
        )
    } else if let Sig::Sol { sol_sig, index: _ } = &new_address_sig {
        Address::new_sol(
            sol_sig.address_base58.clone(),
            clock.unix_timestamp,
            sol_sig.sig_base58.clone(),
            Role::Admin,
        )
    } else {
        return Err(SquirclErrorCode::InvalidSignature.into());
    };

    match controller_sig {
        Sig::Eth { eth_sig, index } => {
            let controller_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let add_message_as_controller = get_default_add_message_as_controller(
                eth_sig.get_eth_address_hex(),
                new_address.address.clone(),
            );

            eth_sig.verify(&controller_address_sign_ix, add_message_as_controller)?;

            did.eth_addresses.iter().for_each(|address| {
                if address.address == eth_sig.get_eth_address_hex() {
                    if !(matches!(address.role, Role::Controller)
                        || matches!(address.role, Role::Admin))
                    {
                        panic!("{}", SquirclErrorCode::AddressDoesntHaveEnoughPermissions)
                    }
                }
            });
        }
        Sig::Sol { sol_sig, index } => {
            let controller_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let add_message_as_controller = get_default_add_message_as_controller(
                sol_sig.address_base58.clone(),
                new_address.address.clone(),
            );

            sol_sig.verify(&controller_address_sign_ix, add_message_as_controller)?;

            did.sol_addresses.iter().for_each(|address| {
                if address.address == sol_sig.address_base58 {
                    if !(matches!(address.role, Role::Controller)
                        || matches!(address.role, Role::Admin))
                    {
                        panic!("{}", SquirclErrorCode::AddressDoesntHaveEnoughPermissions)
                    }
                }
            });
        }
    }

    match new_address_sig {
        Sig::Eth { eth_sig, index } => {
            let new_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let new_address_hex = eth_sig.get_eth_address_hex();

            let add_message_as_new_adress =
                get_default_add_message_as_new_address(new_address_hex.clone());

            eth_sig.verify(&new_address_sign_ix, add_message_as_new_adress)?;

            did.eth_addresses.iter().for_each(|address| {
                if address.address == new_address.address {
                    panic!("{}", SquirclErrorCode::AddressAlreadyExists)
                }
            });

            did.add_address_eth(clock.clone(), new_address);
        }
        Sig::Sol { sol_sig, index } => {
            let new_address_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let new_address_hex = sol_sig.address_base58.clone();

            let add_message_as_new_adress =
                get_default_add_message_as_new_address(new_address_hex.clone());

            sol_sig.verify(&new_address_sign_ix, add_message_as_new_adress)?;

            did.sol_addresses.iter().for_each(|address| {
                if address.address == new_address.address {
                    panic!("{}", SquirclErrorCode::AddressAlreadyExists)
                }
            });

            did.add_address_sol(clock.clone(), new_address);
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(did_str: String, new_address_sig: Sig)]
pub struct AddAddress<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        bump,
        realloc = Did::LEN_WITHOUT_ADDRESS + did.sol_addresses.len() as usize * Address::SOL_LEN + did.eth_addresses.len() as usize * Address::ETH_LEN + match new_address_sig {
            Sig::Eth { .. } => Address::ETH_LEN,
            Sig::Sol { .. } => Address::SOL_LEN,
        },
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub did: Account<'info, Did>,
    pub system_program: Program<'info, System>,
    /// CHECK: we make sure the sysvar is the actual sysvar account
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}
