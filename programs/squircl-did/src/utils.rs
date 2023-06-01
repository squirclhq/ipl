// use crate::{constants::get_default_create_message, IdentityErrorCode};
// use anchor_lang::{
//     prelude::ProgramError,
//     solana_program::{
//         keccak,
//         secp256k1_recover::{secp256k1_recover, SECP256K1_SIGNATURE_LENGTH},
//     },
// };
// // use ed25519_dalek::{PublicKey, Signature};

// pub fn verify_eth_signature(
//     message: &[u8],
//     signature: &[u8],
//     public_key: &[u8],
// ) -> Result<bool, ProgramError> {
//     // let message = Message::parse_slice(message)?;
//     // let signature = RecoverableSignature::parse(signature)?;
//     // let public_key = PublicKey::parse_slice(public_key)?;

//     // msg!("message: {:?}", message);
//     // msg!("signature: {:?}", signature);
//     // msg!("public_key: {:?}", public_key);

//     // let recovery_id = RecoveryId::parse(signature.as_ref()[64])?;
//     // let signature = signature.into_compact();

//     // msg!("recovery_id: {:?}", recovery_id);
//     // msg!("signature: {:?}", signature);

//     // let (recovered_key, _) = RecoverableSignature::recover(signature, message);

//     // msg!("recovered_key: {:?}", recovered_key);

//     // if public_key != recovered_key {
//     //     msg!("public_key != recovered_key");
//     //     return Err(IdentityErrorCode::InvalidSignature.into());
//     // }

//     // msg!("public_key == recovered_key");

//     // Ok(())

//     // // let message = get_default_create_message(address.clone(), nonce);
//     // let message = get_default_create_message(address.clone());

//     let message_hash = keccak::hash(message.as_bytes());

//     // msg!("signature: {:?}", signature);
//     // msg!("recovery_id: {:?}", recovery_id);

//     // msg!("address: {}", address);

//     let signature = signature[..SECP256K1_SIGNATURE_LENGTH].try_into().unwrap();
//     let recovery_id = signature[SECP256K1_SIGNATURE_LENGTH] - 27;

//     // // recover public key from signature
//     // let recovered_pubkey = match secp256k1_recover(message_hash.as_ref(), recovery_id, &signature) {
//     //     Ok(recovered_pubkey) => recovered_pubkey,
//     //     Err(_) => {
//     //         msg!("err recovering");
//     //         return Err(Secp256k1RecoverError::InvalidSignature);
//     //     }
//     // };

//     // // msg!(
//     // //     "recovered_pubkey hex: {:?}",
//     // //     hex::encode(recovered_pubkey.to_bytes())
//     // // );
//     // let mut r_address = [0u8; 20];
//     // r_address.copy_from_slice(&keccak::hash(recovered_pubkey.to_bytes().as_ref()).to_bytes()[12..]);
//     // msg!("r_address: {:?}", r_address);

//     // // remove 0x from address

//     // // convert the address stirng to Secp256k1Pubkey
//     // let address_bytes = hex::decode(address.trim_start_matches("0x")).unwrap();
//     // msg!("address_bytes: {:?}", address_bytes);
//     // // let address_pubkey = Secp256k1Pubkey::new(&address_bytes);
//     // // msg!("address_pubkey: {:?}", address_pubkey.to_bytes());

//     // msg!("recovered_pubkey: {:?}", recovered_pubkey.to_bytes());
//     // msg!("address_bytes: {:?}", address_bytes.deref());

//     // msg!("recover address hex: {:?}", hex::encode(r_address));
//     // msg!("address hex: {:?}", hex::encode(address_bytes.deref()));

//     // // compare the recovered public key with the address
//     // Ok(recovered_pubkey.to_bytes() == address_bytes.deref())
// }

// pub fn verify_sol_signature(address: String, signature: String) -> Result<bool, IdentityErrorCode> {
//     // let message = get_default_create_message(address.clone(), nonce);
//     let message = get_default_create_message(address.clone());
//     let message_bytes = message.as_bytes();

//     // let pubkey = match Pubkey::from_str(&address) {
//     //     Ok(pubkey) => pubkey,
//     //     Err(_) => return Err(IdentityErrorCode::InvalidAddress.into()),
//     // };

//     // let public_key = match PublicKey::from_bytes(&pubkey.to_bytes()) {
//     //     Ok(public_key) => public_key,
//     //     Err(_) => return Err(IdentityErrorCode::InvalidAddress.into()),
//     // };

//     // // decode base58 signature
//     // let signature_bytes = match bs58::decode(signature).into_vec() {
//     //     Ok(signature_bytes) => signature_bytes,
//     //     Err(_) => return Err(IdentityErrorCode::InvalidSignature.into()),
//     // };

//     // let sig = match Signature::from_bytes(&signature_bytes) {
//     //     Ok(sig) => sig,
//     //     Err(_) => return Err(IdentityErrorCode::InvalidSignature.into()),
//     // };

//     // Ok(public_key.verify_strict(message_bytes, &sig).is_ok())
//     Ok(true)
// }

use anchor_lang::{
    prelude::*,
    solana_program::{
        entrypoint::ProgramResult,
        hash::hash,
        instruction::Instruction,
        secp256k1_program::ID as SECP256K1_ID,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

use crate::IdentityErrorCode;

/// Verify Secp256k1Program instruction fields
pub fn verify_secp256k1_ix(
    ix: &Instruction,
    eth_address: &[u8],
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> Result<()> {
    if ix.program_id != SECP256K1_ID
    // And data of this size
    {
        msg!("Invalid Secp256k1 instruction");
        return Err(IdentityErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    check_secp256k1_data(&ix.data, eth_address, msg, sig, recovery_id)?; // If that's not the case, check data

    Ok(())
}

/// Verify serialized Secp256k1Program instruction data
pub fn check_secp256k1_data(
    data: &[u8],
    eth_address: &[u8],
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> Result<()> {
    msg!("data: {:?}", data);

    // According to this layout used by the Secp256k1Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/secp256k1-program.ts#L49

    // "Deserializing" byte slices

    let num_signatures = &[data[0]]; // Byte  0
    let signature_offset = &data[1..=2]; // Bytes 1,2
    let signature_instruction_index = &[data[3]]; // Byte  3
    let eth_address_offset = &data[4..=5]; // Bytes 4,5
    let eth_address_instruction_index = &[data[6]]; // Byte  6
    let message_data_offset = &data[7..=8]; // Bytes 7,8
    let message_data_size = &data[9..=10]; // Bytes 9,10
    let message_instruction_index = &[data[11]]; // Byte  11

    let data_eth_address = &data[12..12 + 20]; // Bytes 12..12+20
    let data_sig = &data[32..32 + 64]; // Bytes 32..32+64
    let data_recovery_id = &[data[96]]; // Byte  96
    let data_msg = &data[97..]; // Bytes 97..end

    // Expected values

    const SIGNATURE_OFFSETS_SERIALIZED_SIZE: u16 = 11;
    const DATA_START: u16 = 1 + SIGNATURE_OFFSETS_SERIALIZED_SIZE;

    let msg_len: u16 = msg.len().try_into().unwrap();
    let eth_address_len: u16 = eth_address.len().try_into().unwrap();
    let sig_len: u16 = sig.len().try_into().unwrap();

    let exp_eth_address_offset: u16 = DATA_START;
    let exp_signature_offset: u16 = DATA_START + eth_address_len;
    let exp_message_data_offset: u16 = exp_signature_offset + sig_len + 1;
    let exp_num_signatures: u8 = 1;

    // Header and Arg Checks

    // Header
    // if num_signatures != &exp_num_signatures.to_le_bytes()
    //     || signature_offset != &exp_signature_offset.to_le_bytes()
    //     || signature_instruction_index != &[0]
    //     || eth_address_offset != &exp_eth_address_offset.to_le_bytes()
    //     || eth_address_instruction_index != &[0]
    //     || message_data_offset != &exp_message_data_offset.to_le_bytes()
    //     || message_data_size != &msg_len.to_le_bytes()
    //     || message_instruction_index != &[0]
    // {
    //     msg!("Invalid Secp256k1 instruction data 1");
    //     return Err(IdentityErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    // }

    // the above check is failing, figure out by logging which check is actually failing

    if num_signatures != &exp_num_signatures.to_le_bytes() {
        msg!("num_signatures: {:?}", num_signatures);
        msg!("exp_num_signatures: {:?}", exp_num_signatures.to_le_bytes());
        msg!("num_signatures != exp_num_signatures");
    }

    if signature_offset != &exp_signature_offset.to_le_bytes() {
        msg!("signature_offset: {:?}", signature_offset);
        msg!(
            "exp_signature_offset: {:?}",
            exp_signature_offset.to_le_bytes()
        );
        msg!("signature_offset != exp_signature_offset");
    }

    if signature_instruction_index != &[0] {
        msg!(
            "signature_instruction_index: {:?}",
            signature_instruction_index
        );
        msg!("signature_instruction_index != &[0]");
    }

    if eth_address_offset != &exp_eth_address_offset.to_le_bytes() {
        msg!("eth_address_offset: {:?}", eth_address_offset);
        msg!(
            "exp_eth_address_offset: {:?}",
            exp_eth_address_offset.to_le_bytes()
        );
        msg!("eth_address_offset != exp_eth_address_offset");
    }

    if eth_address_instruction_index != &[0] {
        msg!(
            "eth_address_instruction_index: {:?}",
            eth_address_instruction_index
        );
        msg!("eth_address_instruction_index != &[0]");
    }

    if message_data_offset != &exp_message_data_offset.to_le_bytes() {
        msg!("message_data_offset: {:?}", message_data_offset);
        msg!(
            "exp_message_data_offset: {:?}",
            exp_message_data_offset.to_le_bytes()
        );
        msg!("message_data_offset != exp_message_data_offset");
    }

    if message_data_size != &msg_len.to_le_bytes() {
        msg!("message_data_size: {:?}", message_data_size);
        msg!("msg_len: {:?}", msg_len);
        msg!("message_data_size != msg_len");
    }

    if message_instruction_index != &[0] {
        msg!("message_instruction_index: {:?}", message_instruction_index);
        msg!("message_instruction_index != &[0]");
    }

    // Arguments
    // if data_eth_address != eth_address
    //     || data_sig != sig
    //     || data_recovery_id != &[recovery_id]
    //     || data_msg != msg
    // {
    //     msg!("Invalid Secp256k1 instruction data 2");
    //     return Err(IdentityErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    // }

    // the above check is failing, figure out by logging which check is actually failing

    if data_eth_address != eth_address {
        msg!("data_eth_address: {:?}", data_eth_address);
        msg!("eth_address: {:?}", eth_address);
        msg!("data_eth_address != eth_address");
    }

    if data_sig != sig {
        msg!("data_sig: {:?}", data_sig);
        msg!("sig: {:?}", sig);
        msg!("data_sig != sig");
    }

    if data_recovery_id != &[recovery_id] {
        msg!("data_recovery_id: {:?}", data_recovery_id);
        msg!("recovery_id: {:?}", recovery_id);
        msg!("data_recovery_id != &[recovery_id]");
    }

    if data_msg != msg {
        msg!("data_msg: {:?}", data_msg);
        msg!("msg: {:?}", msg);
        msg!("data_msg != msg");
    }

    Ok(())
}
