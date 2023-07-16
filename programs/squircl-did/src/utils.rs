// Big kudos to https://github.com/GuidoDipietro/solana-ed25519-secp256k1-sig-verification/blob/master/programs/solana-ed25519-sig-verification/src/lib.rs

use anchor_lang::{
    prelude::*,
    solana_program::{
        ed25519_program::ID as ED25519_ID, instruction::Instruction, keccak,
        secp256k1_program::ID as SECP256K1_ID,
    },
};

use crate::errors::SquirclErrorCode;

pub fn get_ethereum_message_hash(message: String) -> Vec<u8> {
    let msg_data = [
        "\x19Ethereum Signed Message:\n".as_bytes(),
        message.len().to_string().as_bytes(),
        message.as_ref(),
    ]
    .concat();

    let hash = keccak::hash(&msg_data);

    [
        "\x19Ethereum Signed Message:\n32".as_bytes(),
        &hash.to_bytes(),
    ]
    .concat()
}

pub fn get_default_create_message(address: String, nonce: i64) -> String {
    format!(
        "I am creating a new Squircl DID with the address {}. Nonce: {}",
        address, nonce
    )
}

pub fn get_default_add_message_as_controller(
    controller: String,
    new_address: String,
    nonce: i64,
) -> String {
    format!(
        "I am adding {} to the Squircl DID with the address {}. Nonce: {}",
        new_address, controller, nonce
    )
}

pub fn get_default_add_message_as_new_address(new_address: String, nonce: i64) -> String {
    format!(
        "I am adding myself to the Squircl DID with the address {}. Nonce: {}",
        new_address, nonce
    )
}

pub fn get_default_remove_message_as_address(address: String, nonce: i64) -> String {
    format!(
        "I am removing myself from the Squircl DID with the address {}. Nonce: {}",
        address, nonce
    )
}

pub fn get_default_remove_message_as_controller(
    controller: String,
    address: String,
    nonce: i64,
) -> String {
    format!(
        "I am removing {} from the Squircl DID with the address {}. Nonce: {}",
        address, controller, nonce
    )
}

pub fn get_default_issue_credential_message(
    credential_id: &String,
    issuer_did: &String,
    subject_did: &String,
    uri: &String,
    hash: &String,
    nonce: i64,
) -> String {
    format!("I am issuing a {} credential to {} for the Squircl DID with the DID {}. Uri: {}. Hash: {}. Nonce: {}", credential_id, subject_did, issuer_did, uri, hash, nonce)
}

pub fn get_default_update_credential_message(
    credential_id: &String,
    issuer_did: &String,
    subject_did: &String,
    uri: &String,
    hash: &String,
    nonce: i64,
) -> String {
    format!(
        "I am updating the {} credential issued to {} by {}. New uri: {}. New hash: {}. Nonce: {}",
        credential_id, subject_did, issuer_did, uri, hash, nonce
    )
}

pub fn get_default_revoke_credential_message(
    credential_id: &String,
    issuer_did: &String,
    subject_did: &String,
    nonce: i64,
) -> String {
    format!(
        "I am revoking the {} credential issued to {} by {}. Nonce: {}",
        credential_id, subject_did, issuer_did, nonce
    )
}

/// Verify Secp256k1Program instruction fields
pub fn verify_secp256k1_ix(
    ix: &Instruction,
    eth_address: &[u8],
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> Result<()> {
    if ix.program_id       != SECP256K1_ID                 ||  // The program id we expect
    ix.accounts.len()   != 0                            ||  // With no context accounts
    ix.data.len()       != (12 + 20 + 64 + 1 + msg.len())
    // And data of this size
    {
        msg!("Invalid Secp256k1 instruction");
        return Err(SquirclErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    check_secp256k1_data(&ix.data, eth_address, msg, sig, recovery_id)?; // If that's not the case, check data

    Ok(())
}

pub fn verify_ed25519_ix(ix: &Instruction, pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    if ix.program_id       != ED25519_ID                   ||  // The program id we expect
        ix.accounts.len()   != 0                            ||  // With no context accounts
        ix.data.len()       != (16 + 64 + 32 + msg.len())
    // And data of this size
    {
        msg!("Invalid Ed25519 instruction");
        return Err(SquirclErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    check_ed25519_data(&ix.data, pubkey, msg, sig)?; // If that's not the case, check data

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
    if num_signatures != &exp_num_signatures.to_le_bytes()
        || signature_offset != &exp_signature_offset.to_le_bytes()
        || signature_instruction_index != &[0]
        || eth_address_offset != &exp_eth_address_offset.to_le_bytes()
        || eth_address_instruction_index != &[0]
        || message_data_offset != &exp_message_data_offset.to_le_bytes()
        || message_data_size != &msg_len.to_le_bytes()
        || message_instruction_index != &[0]
    {
        msg!("Invalid Secp256k1 instruction data (header)");
        return Err(SquirclErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    // the above check is failing, figure out by logging which check is actually failing

    // if num_signatures != &exp_num_signatures.to_le_bytes() {
    //     msg!("num_signatures: {:?}", num_signatures);
    //     msg!("exp_num_signatures: {:?}", exp_num_signatures.to_le_bytes());
    //     msg!("num_signatures != exp_num_signatures");
    // }

    // if signature_offset != &exp_signature_offset.to_le_bytes() {
    //     msg!("signature_offset: {:?}", signature_offset);
    //     msg!(
    //         "exp_signature_offset: {:?}",
    //         exp_signature_offset.to_le_bytes()
    //     );
    //     msg!("signature_offset != exp_signature_offset");
    // }

    // if signature_instruction_index != &[0] {
    //     msg!(
    //         "signature_instruction_index: {:?}",
    //         signature_instruction_index
    //     );
    //     msg!("signature_instruction_index != &[0]");
    // }

    // if eth_address_offset != &exp_eth_address_offset.to_le_bytes() {
    //     msg!("eth_address_offset: {:?}", eth_address_offset);
    //     msg!(
    //         "exp_eth_address_offset: {:?}",
    //         exp_eth_address_offset.to_le_bytes()
    //     );
    //     msg!("eth_address_offset != exp_eth_address_offset");
    // }

    // if eth_address_instruction_index != &[0] {
    //     msg!(
    //         "eth_address_instruction_index: {:?}",
    //         eth_address_instruction_index
    //     );
    //     msg!("eth_address_instruction_index != &[0]");
    // }

    // if message_data_offset != &exp_message_data_offset.to_le_bytes() {
    //     msg!("message_data_offset: {:?}", message_data_offset);
    //     msg!(
    //         "exp_message_data_offset: {:?}",
    //         exp_message_data_offset.to_le_bytes()
    //     );
    //     msg!("message_data_offset != exp_message_data_offset");
    // }

    // if message_data_size != &msg_len.to_le_bytes() {
    //     msg!("message_data_size: {:?}", message_data_size);
    //     msg!("msg_len: {:?}", msg_len);
    //     msg!("message_data_size != msg_len");
    // }

    // if message_instruction_index != &[0] {
    //     msg!("message_instruction_index: {:?}", message_instruction_index);
    //     msg!("message_instruction_index != &[0]");
    // }

    // the above check is failing, figure out by logging which check is actually failing

    // if data_eth_address != eth_address {
    //     msg!("data_eth_address: {:?}", data_eth_address);
    //     msg!("eth_address: {:?}", eth_address);
    //     msg!("data_eth_address != eth_address");
    // }

    // if data_sig != sig {
    //     msg!("data_sig: {:?}", data_sig);
    //     msg!("sig: {:?}", sig);
    //     msg!("data_sig != sig");
    // }

    // if data_recovery_id != &[recovery_id] {
    //     msg!("data_recovery_id: {:?}", data_recovery_id);
    //     msg!("recovery_id: {:?}", recovery_id);
    //     msg!("data_recovery_id != &[recovery_id]");
    // }

    // if data_msg != msg {
    //     msg!("data_msg: {:?}", data_msg);
    //     msg!("msg: {:?}", msg);
    //     msg!("data_msg != msg");
    // }

    // Arguments
    if data_eth_address != eth_address
        || data_sig != sig
        || data_recovery_id != &[recovery_id]
        || data_msg != msg
    {
        msg!("Invalid Secp256k1 instruction data (arguments)");
        return Err(SquirclErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    Ok(())
}

/// Verify serialized Ed25519Program instruction data
pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33

    // "Deserializing" byte slices

    let num_signatures = &[data[0]]; // Byte  0
    let padding = &[data[1]]; // Byte  1
    let signature_offset = &data[2..=3]; // Bytes 2,3
    let signature_instruction_index = &data[4..=5]; // Bytes 4,5
    let public_key_offset = &data[6..=7]; // Bytes 6,7
    let public_key_instruction_index = &data[8..=9]; // Bytes 8,9
    let message_data_offset = &data[10..=11]; // Bytes 10,11
    let message_data_size = &data[12..=13]; // Bytes 12,13
    let message_instruction_index = &data[14..=15]; // Bytes 14,15

    let data_pubkey = &data[16..16 + 32]; // Bytes 16..16+32
    let data_sig = &data[48..48 + 64]; // Bytes 48..48+64
    let data_msg = &data[112..]; // Bytes 112..end

    // Expected values

    let exp_public_key_offset: u16 = 16; // 2*u8 + 7*u16
    let exp_signature_offset: u16 = exp_public_key_offset + pubkey.len() as u16;
    let exp_message_data_offset: u16 = exp_signature_offset + sig.len() as u16;
    let exp_num_signatures: u8 = 1;
    let exp_message_data_size: u16 = msg.len().try_into().unwrap();

    // Header and Arg Checks

    // Header
    if num_signatures != &exp_num_signatures.to_le_bytes()
        || padding != &[0]
        || signature_offset != &exp_signature_offset.to_le_bytes()
        || signature_instruction_index != &u16::MAX.to_le_bytes()
        || public_key_offset != &exp_public_key_offset.to_le_bytes()
        || public_key_instruction_index != &u16::MAX.to_le_bytes()
        || message_data_offset != &exp_message_data_offset.to_le_bytes()
        || message_data_size != &exp_message_data_size.to_le_bytes()
        || message_instruction_index != &u16::MAX.to_le_bytes()
    {
        msg!("Invalid Ed25519 instruction data (header)");
        return Err(SquirclErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    // Arguments
    if data_pubkey != pubkey || data_msg != msg || data_sig != sig {
        msg!("Invalid Ed25519 instruction data (arguments)");
        return Err(SquirclErrorCode::InvalidSignature.into()); // Otherwise, we can already throw err
    }

    Ok(())
}
