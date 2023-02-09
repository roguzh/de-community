use anchor_lang::{prelude::*};
use crate::{state::{Member, Community, Proposer}};

#[derive(Accounts)]
pub struct VoteProposal<'info> {
    ///CHECK
    #[account(
        mut,
        seeds = [
            proposer.key().as_ref(),
            proposer.proposal_count.to_be_bytes().as_ref() 
        ], 
        bump
    )]
    pub proposal: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [
            b"proposer", 
            community.key().as_ref(), 
            proposer.owner.key().as_ref()
        ],
        bump
    )]
    pub proposer: Account<'info, Proposer>,
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account[
        mut,
        seeds = [ b"member", community.key().as_ref(), signer.key().as_ref() ],
        bump,
    ]]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn vote_proposal(
    _ctx: Context<VoteProposal>,
    _vote: bool
) -> Result<()> {
    Ok(())
}