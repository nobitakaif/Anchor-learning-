use anchor_lang::prelude::*;

#[constant]
pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_MINT: &[u8] = b"mint";
pub const SEED_COLLATERAL_ACCOUNT : &[u8] = b"collateral";
pub const SEED_SOL_ACCOUNT : &[u8] = b"sol";

pub const MINT_DECIMAL: u8 = 9;

pub const LIQUIDATION_THRESHOLD : u64 = 50;
pub const LIQUIDATION_BONUS : u64 = 10;
pub const MIN_HEALTH_FACTOR : u64 = 1;