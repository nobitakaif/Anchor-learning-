use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, TokenAccount, Token2022};

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::instructions::check_health_factor;
use crate::{Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG};

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnToken<'info>{
    #[account(mut)]
    pub depositer : Signer<'info>,

    pub price_update : Account<'info, PriceUpdateV2>,
    #[account(
        seeds = [SEED_CONFIG],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account : Account<'info, Config>,
    
    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositer.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = token_account
    )]
    pub collateral_account : Account<'info, Collateral>,
    #[account(mut)]
    pub sol_account : SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program : Program<'info, TokenAccount>,
    pub token_program : Program<'info, Token2022>
}

pub fn process_redeem_collateral_and_burn_token(ctx: Context<RedeemCollateralAndBurnToken>, amount_collateral: u64, amount_to_burn: u64) -> Result<()>{
    let colletral_accounnt = &mut ctx.accounts.collateral_account;
    colletral_accounnt.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;
    colletral_accounnt.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    );
    Ok(())
}