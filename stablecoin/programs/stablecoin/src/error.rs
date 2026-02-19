use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("invalide rpice")]
    InvalidPrice,
    #[msg("Below health factor")]
    BelowMinHealthFactor,
}
