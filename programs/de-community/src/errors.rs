use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("End date is too short")]
    EndDateTooShort,

    #[msg("Member is not authorized")]
    UnauthorizedMember,

    #[msg("Proposer is not authorized")]
    UnauthorizedProposer,

    #[msg("Invalid Proposal")]
    InvalidProposal,
}