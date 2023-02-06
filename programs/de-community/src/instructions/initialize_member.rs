use anchor_lang::prelude::*;

use crate::state::{Community, Member};

#[derive(Accounts)]
pub struct InitializeMember<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account[
        init,
        seeds = [ b"member", community.key().as_ref(), member_owner.key().as_ref() ],
        bump,
        payer = payer,
        space = 8 + 1 + 4
    ]]
    pub member: Account<'info, Member>,
    ///CHECK
    pub member_owner: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}


pub fn initialize_member(
    ctx: Context<InitializeMember>
) -> Result<()> {
    let member = &mut ctx.accounts.member;

    msg!("Member initialized");
    member.can_vote = false;
    member.reputation = 0;

    Ok(())
}