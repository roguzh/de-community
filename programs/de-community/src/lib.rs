use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod instructions;
pub mod errors;
pub mod state;

declare_id!("AHJ8w9ePSzS7mjhXV4LAqatQtdEk9DpTgTLdzrpHU6P2");

#[program]
pub mod de_community {
    use super::*;

    pub fn initialize_community(
        ctx: Context<InitializeCommunity>,
        title: String,
        description: String,
        min_proposal_duration: i64,
        nft_collection: Option<Pubkey>
    ) -> Result<()> {
        instructions::initialize_community(ctx, title, description, min_proposal_duration, nft_collection)
    }

    pub fn initialize_member(
        ctx: Context<InitializeMember>
    ) -> Result<()> {
        instructions::initialize_member(ctx)
    }

    pub fn create_manage_member_proposal(
        ctx: Context<CreateManageMemberProposal>,
        action_type: ManageActionType,
        end_date: i64
    ) -> Result<()> {
        instructions::create_manage_member_proposal(ctx, action_type, end_date)
    }

}
