use std::str::FromStr;

use crate::constants::get_default_create_message;
use anchor_lang::solana_program::{
    keccak,
    pubkey::Pubkey,
    secp256k1_recover::{
        secp256k1_recover, Secp256k1Pubkey, Secp256k1RecoverError, SECP256K1_SIGNATURE_LENGTH,
    },
};
use ed25519_dalek::{PublicKey, Signature};

pub fn verify_eth_signature(
    address: String,
    signature: String,
    nonce: i64,
) -> Result<bool, Secp256k1RecoverError> {
    let message = get_default_create_message(address.clone(), nonce);

    let message_hash = keccak::hash(message.as_bytes());

    // get signature and recovery id from the signature string
    let signature_bytes = hex::decode(signature).unwrap();

    let signature = &signature_bytes[..SECP256K1_SIGNATURE_LENGTH];
    let recovery_id = signature_bytes[SECP256K1_SIGNATURE_LENGTH];

    // recover public key from signature
    let recovered_pubkey = secp256k1_recover(message_hash.as_ref(), recovery_id, &signature)?;

    // convert the address stirng to Secp256k1Pubkey
    let address_bytes = hex::decode(address).unwrap();
    let address_pubkey = Secp256k1Pubkey::new(&address_bytes);

    // compare the recovered public key with the address
    Ok(recovered_pubkey.to_bytes() == address_pubkey.to_bytes())
}

pub fn verify_sol_signature(address: String, signature: String, nonce: i64) -> bool {
    let message = get_default_create_message(address.clone(), nonce);
    let message_bytes = message.as_bytes();

    let pubkey = Pubkey::from_str(address.as_str()).unwrap();

    let public_key = PublicKey::from_bytes(&pubkey.to_bytes()).unwrap();

    // decode base58 signature
    let signature_bytes = bs58::decode(signature).into_vec().unwrap();

    let sig = Signature::from_bytes(&signature_bytes).unwrap();

    public_key.verify_strict(message_bytes, &sig).is_ok()
}
