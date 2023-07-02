use anchor_lang::{
    prelude::*,
    solana_program::{
        hash,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

use crate::{
    errors::SquirclErrorCode,
    state::{Address, Chain, Did, Role, Sig},
    utils::{get_default_remove_message_as_address, get_default_remove_message_as_controller},
};

pub fn remove_address_ix(
    ctx: Context<RemoveAddress>,
    address_chain: Chain,
    address: String,
    remover_sig: Sig,
) -> Result<()> {
    let did = &mut ctx.accounts.did;

    let clock: Clock = Clock::get()?;

    match remover_sig {
        Sig::Eth { eth_sig, index } => {
            let remover_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let eth_address = eth_sig.get_eth_address_hex();

            let is_self_remove = address == eth_address;

            if !is_self_remove {
                let mut verified: bool = false;

                did.eth_addresses.iter().for_each(|address| {
                    if address.address == eth_address {
                        if !(matches!(address.role, Role::Controller)
                            || matches!(address.role, Role::Admin))
                        {
                            panic!("{}", SquirclErrorCode::AddressDoesntHaveEnoughPermissions)
                        } else {
                            verified = true;
                        }
                    }
                });

                if !verified {
                    panic!("{}", SquirclErrorCode::AddressDoesNotExistInDID)
                }
            }

            let message = if is_self_remove {
                get_default_remove_message_as_address(eth_address)
            } else {
                get_default_remove_message_as_controller(eth_address, address.clone())
            };

            msg!("message: {}", message);

            msg!("is self remove: {}", is_self_remove);

            eth_sig.verify(&remover_sign_ix, message)?;

            match address_chain {
                Chain::EVM => {
                    let found_address = did
                        .eth_addresses
                        .iter()
                        .find(|addr| addr.address == address);

                    if !found_address.is_some() {
                        panic!("{}", SquirclErrorCode::AddressDoesNotExistInDID)
                    }

                    if matches!(found_address.unwrap().role, Role::Controller) {
                        panic!("{}", SquirclErrorCode::CannotRemoveControllerAddress)
                    }

                    did.remove_address_eth(clock, address)
                }
                Chain::SOL => {
                    let found_address = did
                        .sol_addresses
                        .iter()
                        .find(|addr| addr.address == address);

                    if !found_address.is_some() {
                        panic!("{}", SquirclErrorCode::AddressDoesNotExistInDID)
                    }

                    if matches!(found_address.unwrap().role, Role::Controller) {
                        panic!("{}", SquirclErrorCode::CannotRemoveControllerAddress)
                    }

                    did.remove_address_sol(clock, address)
                }
            }
        }
        Sig::Sol { sol_sig, index } => {
            let remover_sign_ix =
                load_instruction_at_checked(index.try_into().unwrap(), &ctx.accounts.ix_sysvar)?;

            let sol_address = sol_sig.address_base58.clone();

            let is_self_remove = address == sol_address;

            if !is_self_remove {
                let mut verified: bool = false;

                did.sol_addresses.iter().for_each(|address| {
                    if address.address == sol_address {
                        if !(matches!(address.role, Role::Controller)
                            || matches!(address.role, Role::Admin))
                        {
                            panic!("{}", SquirclErrorCode::AddressDoesntHaveEnoughPermissions)
                        }
                    } else {
                        verified = true;
                    }
                });

                if !verified {
                    panic!("{}", SquirclErrorCode::AddressDoesNotExistInDID)
                }
            }

            let message = if is_self_remove {
                get_default_remove_message_as_address(sol_address)
            } else {
                get_default_remove_message_as_controller(sol_address, address.clone())
            };

            msg!("message: {}", message);

            sol_sig.verify(&remover_sign_ix, message)?;

            match address_chain {
                Chain::EVM => {
                    did.eth_addresses.iter().for_each(|addr| {
                        if addr.address == address {
                            if matches!(addr.role, Role::Controller) {
                                panic!("{}", SquirclErrorCode::CannotRemoveControllerAddress)
                            }
                        }
                    });

                    did.remove_address_eth(clock, address)
                }
                Chain::SOL => {
                    did.sol_addresses.iter().for_each(|addr| {
                        if addr.address == address {
                            if matches!(addr.role, Role::Controller) {
                                panic!("{}", SquirclErrorCode::CannotRemoveControllerAddress)
                            }
                        }
                    });

                    did.remove_address_sol(clock, address)
                }
            }
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(did_str: String, address_chain: Chain)]
pub struct RemoveAddress<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [&hash::hash(did_str.as_bytes()).to_bytes()],
        bump,
        realloc = Did::LEN_WITHOUT_ADDRESS + did.sol_addresses.len() as usize * Address::SOL_LEN + did.eth_addresses.len() as usize * Address::ETH_LEN - match address_chain {
           Chain::EVM => Address::ETH_LEN,
           Chain::SOL => Address::SOL_LEN,
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
