use anchor_lang::{prelude::*};
use crate::{state::{Community, Proposer, Proposal, ProposalType, ProposalStatus, ProposalOption, Member }, errors::CustomErrorCode};

pub const APPROVAL_TEXT: &str = "Approve";
pub const DENIAL_TEXT: &str = "Deny";

#[derive(Accounts)]
#[instruction(proposal_type: ProposalType)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [
            proposer.key().as_ref(),
            proposer.proposal_count.to_be_bytes().as_ref() 
        ], 
        bump,
        space = 8 
            + 32 //Proposer 
            + (1+1) //ProposalStatus
            + (8 + (4 + APPROVAL_TEXT.len())) //ProposalOption
            + (8 + (4 + DENIAL_TEXT.len())) //ProposalOption
            + 8 //init_date
            + 8 //end_date
            + ProposalType::get_proposal_type_size(proposal_type),
        payer = signer
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account(
        mut,
        seeds = [ b"proposer", community.key().as_ref(), signer.key().as_ref() ],
        bump
    )]
    pub proposer: Account<'info, Proposer>,
    #[account[
        seeds = [ b"member", community.key().as_ref(), voted_member_owner.as_ref().unwrap().key().as_ref() ],
        bump
    ]]
    pub voted_member: Option<Account<'info, Member>>,
    ///CHECK
    pub voted_member_owner: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}


pub fn create_proposal(
    ctx: Context<CreateProposal>,
    proposal_type: ProposalType,
    end_date: i64
) -> Result<()> {
    let community = &mut ctx.accounts.community;
    let proposal = &mut ctx.accounts.proposal;
    let proposer = &mut ctx.accounts.proposer;

    require!(
        proposer.can_propose,
        CustomErrorCode::UnauthorizedProposer
    );

    let current_time = Clock::get().unwrap().unix_timestamp;
    require_gte!(
        end_date, (current_time + community.min_proposal_duration),
        CustomErrorCode::EndDateTooShort
    );

    proposal.proposer = proposer.key();
    proposal.status = ProposalStatus::Voting;
    proposal.approval_count = ProposalOption {
        option: APPROVAL_TEXT.to_owned(),
        vote_count: 0,
    };
    proposal.denial_count = ProposalOption {
        option: DENIAL_TEXT.to_owned(),
        vote_count: 0,
    };
    proposal.init_date = Clock::get().unwrap().unix_timestamp;
    proposal.end_date = end_date;

    match proposal_type {
        ProposalType::Custom { title, description } => {
            proposal.proposal_type = ProposalType::Custom { 
                title: title, 
                description: description 
            }
        },
        ProposalType::ManageMember { action, voted_account: _, voted_account_owner: _ } => {
            proposal.proposal_type = ProposalType::ManageMember { 
                action, 
                voted_account: ctx.accounts.voted_member.as_ref().unwrap().key(), 
                voted_account_owner: ctx.accounts.voted_member_owner.as_ref().unwrap().key(), 
            }
        },
    }

    proposer.proposal_count = proposer.proposal_count.checked_add(1).unwrap();
    community.proposal_count = community.proposal_count.checked_add(1).unwrap();

    Ok(())
}