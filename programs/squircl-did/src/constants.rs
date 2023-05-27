pub const DISCRIMINATOR_LENGTH: usize = 8;
pub const U8_LENGTH: usize = 1;
pub const U64_LENGTH: usize = 8;
pub const I64_LENGTH: usize = 8;
pub const BOOL_LENGTH: usize = 1;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const STRING_LENGTH_PREFIX: usize = 4;
pub const STRING_CHAR_MULTIPLIER: usize = 4;

pub const DID_LENGTH: usize = STRING_LENGTH_PREFIX + (48 * STRING_CHAR_MULTIPLIER); // 48 chars
pub const ETH_ADDRESS_LENGTH: usize = STRING_LENGTH_PREFIX + (42 * STRING_CHAR_MULTIPLIER); // 42 chars
pub const SOL_ADDRESS_LENGTH: usize = STRING_LENGTH_PREFIX + (44 * STRING_CHAR_MULTIPLIER); // 44 chars

pub const ETH_SIGNATURE_LENGTH: usize = STRING_LENGTH_PREFIX + (132 * STRING_CHAR_MULTIPLIER); // 132 chars
pub const SOL_SIGNATURE_LENGTH: usize = STRING_LENGTH_PREFIX + (64 * STRING_CHAR_MULTIPLIER); // 64 chars

pub fn get_default_create_message(address: String, nonce: i64) -> String {
    format!(
        "I am creating a new Squircl DID with the address {} and nonce {}",
        address, nonce,
    )
}
