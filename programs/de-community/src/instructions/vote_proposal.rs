use anchor_lang::{prelude::*};
use crate::{state::{Member, Community, Vote, Proposal, ProposalStatus, ProposalType}, errors::CustomErrorCode};

#[derive(Accounts)]
pub struct VoteProposal<'info> {
    #[account(
        mut,
        constraint = ( 
            proposal.status == ProposalStatus::Voting && 
            proposal.proposer.key() != voter.key() &&
            proposal.end_date >= Clock::get().unwrap().unix_timestamp
        )
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account(
        init_if_needed,
        seeds = [
            proposal.key().as_ref(),
            voter.key().as_ref()
        ],
        bump,
        space = 8 + Vote::get_size(),
        payer = signer
    )]
    pub vote: Account<'info, Vote>,
    #[account[
        mut,
        seeds = [ b"member", community.key().as_ref(), signer.key().as_ref() ],
        bump,
    ]]
    pub voter: Account<'info, Member>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn vote_proposal(
    ctx: Context<VoteProposal>,
    vote: bool
) -> Result<()> {
    let vote_account = &mut ctx.accounts.vote;
    let proposal = &mut ctx.accounts.proposal;
    let voter = &mut ctx.accounts.voter;

    match &proposal.proposal_type {
        ProposalType::ManageMember { action: _, voted_account, voted_account_owner: _ } => {
            require!(voted_account != &voter.key(), CustomErrorCode::UnauthorizedVoter)
        },
        _ => {},
    }

    if vote_account.is_initialized {
        require!(vote != vote_account.vote, CustomErrorCode::InvalidVoteValue);
        match vote {
            true => {
                proposal.denial_option.vote_count = proposal.denial_option.vote_count.checked_sub(1).unwrap();
                proposal.approval_option.vote_count = proposal.approval_option.vote_count.checked_add(1).unwrap();
            },
            false => {
                proposal.approval_option.vote_count = proposal.approval_option.vote_count.checked_sub(1).unwrap();
                proposal.denial_option.vote_count = proposal.denial_option.vote_count.checked_add(1).unwrap();
            },
        }
    } 
    else {
        vote_account.is_initialized = true;
        voter.reputation = voter.reputation.checked_add(1).unwrap();
        match vote {
            true => {
                proposal.approval_option.vote_count = proposal.approval_option.vote_count.checked_add(1).unwrap();
                vote_account.vote = true;
            },
            false => {
                proposal.denial_option.vote_count = proposal.denial_option.vote_count.checked_add(1).unwrap();
                vote_account.vote = false;
            },
        }
    }

    Ok(())
}