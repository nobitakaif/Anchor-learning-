use anchor_lang::prelude::*;


pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_MINT: &[u8] = b"mint";
pub const SEED_COLLATERAL_ACCOUNT : &[u8] = b"collateral";
pub const SEED_SOL_ACCOUNT : &[u8] = b"sol";
#[constant]
pub const FEED_ID : &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const MAIXIMUM_AGE : u64 = 100;
pub const MINT_DECIMAL: u8 = 9;
pub const LIQUIDATION_THRESHOLD : u64 = 50;
pub const LIQUIDATION_BONUS : u64 = 10;
pub const MIN_HEALTH_FACTOR : u64 = 1;