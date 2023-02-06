use anchor_lang::prelude::*;
use crate::{state::{Community, Proposer, Member, ManageActionType, ManageMemberProposal, ProposalStatus, BaseProposal, ProposalOption}, errors::CustomErrorCode};

pub const APPROVAL_TEXT: &str = "Approve";
pub const DENIAL_TEXT: &str = "Deny";

#[derive(Accounts)]
pub struct CreateManageMemberProposal<'info> {
    #[account(
        init,
        seeds = [ 
            community.key().as_ref(), 
            proposer.key().as_ref(),
            proposer.proposal_count.to_be_bytes().as_ref() 
        ], 
        bump,
        space = 8 
            // BaseProposal
            + 32 //Proposer 
            + (1+1) //ProposalStatus
            + (8 + (4 + APPROVAL_TEXT.len())) //ProposalOption
            + (8 + (4 + DENIAL_TEXT.len())) //ProposalOption
            + 8 //init_date
            + 8 //end_date
        + (1+1) //ManageActionType
        + 32 //voted_account
        + 32,//voted_account_owner
        payer = signer
    )]
    pub proposal: Account<'info, ManageMemberProposal>,
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account(
        mut,
        seeds = [ b"proposer", community.key().as_ref(), signer.key().as_ref() ],
        bump
    )]
    pub proposer: Account<'info, Proposer>,
    #[account(
        seeds = [ b"member", community.key().as_ref(), managed_member_owner.key().as_ref() ],
        bump,
    )]
    pub managed_member: Account<'info, Member>,
    ///CHECK
    #[account()]
    pub managed_member_owner: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}


pub fn create_manage_member_proposal(
    ctx: Context<CreateManageMemberProposal>,
    action_type: ManageActionType,
    end_date: i64
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let community = &mut ctx.accounts.community;
    let proposer = &mut ctx.accounts.proposer;
    let managed_member = &ctx.accounts.managed_member;
    let managed_member_owner = &ctx.accounts.managed_member_owner;

    let current_time = Clock::get().unwrap().unix_timestamp;
    require_gte!(
        end_date, (current_time + community.min_proposal_duration),
        CustomErrorCode::EndDateTooShort
    );

    proposal.proposal = BaseProposal {
        proposer: proposer.key(),
        status: ProposalStatus::Voting,
        approval_count: ProposalOption {
            option: APPROVAL_TEXT.to_owned(),
            vote_count: 0,
        },
        denial_count: ProposalOption {
            option: DENIAL_TEXT.to_owned(),
            vote_count: 0,
        },
        init_date: Clock::get().unwrap().unix_timestamp,
        end_date,
    };
    proposal.action = action_type;
    proposal.voted_account = managed_member.key();
    proposal.voted_account_owner = managed_member_owner.key();

    community.proposal_count = community.proposal_count.checked_add(1).unwrap();
    proposer.proposal_count = proposer.proposal_count.checked_add(1).unwrap();

    Ok(())
}