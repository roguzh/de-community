use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("End date is too short")]
    EndDateTooShort,

    #[msg("Member is not authorized")]
    UnauthorizedMember,

    #[msg("Proposer is not authorized")]
    UnauthorizedProposer,

    #[msg("Voter can not vote for his manage proposal")]
    UnauthorizedVoter,

    #[msg("Member is not part of the community")]
    InvalidMember,
    
    #[msg("Invalid Proposal")]
    InvalidProposal,

    #[msg("Vote already has the same value")]
    InvalidVoteValue,

}