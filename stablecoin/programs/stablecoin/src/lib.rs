
use anchor_lang::prelude::*;

use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;
mod error;

declare_id!("EpmQqtC5wRxYr5fnWy7LqnUquPUYJuQVKaSwQwYLZZw4");


#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx : Context<InitializeConfig>) -> Result<()>{
        process_initialize_config(ctx);
        Ok(())
    }
    pub fn update_config(ctx : Context<UpdateConfig>, min_health_factor : u64) -> Result<()>{
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_token(
        ctx : Context<DepositCollateralAndMintToken>,
        amount_collateral : u64,
        amount_to_mint: u64
    ) -> Result<()>{
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }
    
}
