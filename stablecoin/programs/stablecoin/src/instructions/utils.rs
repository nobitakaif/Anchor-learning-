use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};


use crate::constants::{FEED_ID, MAIXIMUM_AGE};
use crate::state::{Collateral, Config};

pub fn calculate_health_factor(
    collateral: &Account<Collateral>,
    config : &Account<Config>,
    price_feed  : &Account<PriceUpdateV2>,
) -> Result<u64>{
    let collateral_vallue_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;

}

pub fn get_usd_value(& amount_in_lamport : &u64, price_feed  : &Account<PriceUpdateV2>)-> Result<u64>{
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAIXIMUM_AGE, &feed_id)?;
    
}