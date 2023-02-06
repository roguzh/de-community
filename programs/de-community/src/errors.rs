use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("End date is too short")]
    EndDateTooShort,
}