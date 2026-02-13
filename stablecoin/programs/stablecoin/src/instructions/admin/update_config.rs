use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::tools::account;

use crate::constants::{SEED_CONFIG};
use crate::{Config};
#[derive(Accounts)]
pub struct UpdateConfig<'info>{
    #[account(
        mut, seeds = [SEED_CONFIG], bump = config_account.bump
    )]
    pub config_account : Account<'info, Config>,
}

pub fn process_update_config(ctx: Context<UpdateConfig>, min_health_factor : u64)  -> Result<()>{
    let config_account   = &mut ctx.accounts.config_account;
    config_account.min_health_factor = min_health_factor;
    Ok(())
}