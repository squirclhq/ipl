pub const DISCRIMINATOR_LENGTH: usize = 8;
pub const U8_LENGTH: usize = 1;
pub const U64_LENGTH: usize = 8;
pub const I64_LENGTH: usize = 8;
pub const BOOL_LENGTH: usize = 1;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const STRING_LENGTH_PREFIX: usize = 4;
pub const STRING_CHAR_MULTIPLIER: usize = 4;

pub const DID_LENGTH: usize = STRING_LENGTH_PREFIX + (48 * STRING_CHAR_MULTIPLIER); // 48 chars

pub const ETH_ADDRESS_CHARS: usize = 42;
pub const SOL_ADDRESS_CHARS: usize = 44;
pub const ETH_SIGNATURE_CHARS: usize = 132;
pub const SOL_SIGNATURE_CHARS: usize = 88;

pub const ETH_ADDRESS_LENGTH: usize =
    STRING_LENGTH_PREFIX + (ETH_ADDRESS_CHARS * STRING_CHAR_MULTIPLIER); // 42 chars
pub const SOL_ADDRESS_LENGTH: usize =
    STRING_LENGTH_PREFIX + (SOL_ADDRESS_CHARS * STRING_CHAR_MULTIPLIER); // 44 chars
pub const ETH_SIGNATURE_LENGTH: usize =
    STRING_LENGTH_PREFIX + (ETH_SIGNATURE_CHARS * STRING_CHAR_MULTIPLIER); // 132 chars
pub const SOL_SIGNATURE_LENGTH: usize =
    STRING_LENGTH_PREFIX + (SOL_SIGNATURE_CHARS * STRING_CHAR_MULTIPLIER); // 88 chars

pub const ETHEREUM_MSG_PREFIX: &str = "\x19Ethereum Signed Message:\n";
