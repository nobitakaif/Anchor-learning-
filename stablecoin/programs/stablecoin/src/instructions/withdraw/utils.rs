use anchor_lang::prelude::*;

use crate::constants::SEED_SOL_ACCOUNT;

pub fn withdraw_sol<'info>(
    bump : u8,
    depositor_key:&Pubkey,
) -> Result<()>{
    let signer_seeds : &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.key().as_ref(), &[bump]]];
    Ok(())
}