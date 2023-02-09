use anchor_lang::prelude::*;

use crate::state::{Community, Proposer, Member};

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct InitializeCommunity<'info> {
    #[account(
        init,
        seeds = [ initializer.key().as_ref(), title.as_bytes() ], 
        bump,
        payer = initializer, 
        space = 8 + (4 + title.len()) + (4 + description.len()) + 32 + 4 + 4 + 8 + 8 + (1+ 32)
    )]
    pub community: Account<'info, Community>,
    #[account(
        init,
        seeds = [ b"proposer", community.key().as_ref(), initializer.key().as_ref() ],
        bump,
        payer = initializer,
        space = 8 + 1 + 1 + 8 + 32
    )]
    pub initializer_proposer: Account<'info, Proposer>,
    #[account[
        init,
        seeds = [ b"member", community.key().as_ref(), initializer.key().as_ref() ],
        bump,
        payer = initializer,
        space = 8 + 1 + 4
    ]]
    pub initializer_member: Account<'info, Member>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}


pub fn initialize_community(
    ctx: Context<InitializeCommunity>,
    title: String,
    description: String,
    min_proposal_duration: i64,
    nft_collection: Option<Pubkey>
) -> Result<()> {
    let community = &mut ctx.accounts.community;
    let proposer = &mut ctx.accounts.initializer_proposer;
    let member = &mut ctx.accounts.initializer_member;

    community.title = title;
    community.description = description;
    community.initializer = ctx.accounts.initializer.key();
    community.voter_member_count = 1; 
    community.proposer_count = 1;
    community.proposal_count = 0;
    community.min_proposal_duration = min_proposal_duration;
    community.collection_nft = nft_collection;
    msg!("Community is created!");

    proposer.is_voted = false;
    proposer.can_propose = true;
    proposer.proposal_count = 0;
    proposer.owner = ctx.accounts.initializer.key();
    msg!("Initializer is set as a proposer!");

    member.can_vote = true;
    member.reputation = 0;
    msg!("Initializer is set as a member with vote right!");

    Ok(())
}