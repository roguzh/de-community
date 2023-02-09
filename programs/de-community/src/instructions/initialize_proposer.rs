use anchor_lang::prelude::*;

use crate::{state::{Community, Member, Proposer}, errors::CustomErrorCode};

#[derive(Accounts)]
pub struct InitializeProposer<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account[
        seeds = [ b"member", community.key().as_ref(), owner.key().as_ref() ],
        bump
    ]]
    pub member: Account<'info, Member>,
    #[account(
        init,
        seeds = [ b"proposer", community.key().as_ref(), owner.key().as_ref() ],
        bump,
        payer = payer,
        space = 8 + 1 + 1 + 8 + 32
    )]
    pub proposer: Account<'info, Proposer>,
    ///CHECK
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}


pub fn initialize_proposer(
    ctx: Context<InitializeProposer>
) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let proposer = &mut ctx.accounts.proposer;

    require!(member.can_vote, CustomErrorCode::UnauthorizedMember);

    proposer.is_voted = false;
    proposer.can_propose = false;
    proposer.proposal_count = 0;
    proposer.owner = ctx.accounts.owner.key();
    
    msg!("Proposer initialized");

    Ok(())
}