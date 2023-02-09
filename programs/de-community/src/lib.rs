use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
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
        nft_collection: Option<Pubkey>,
    ) -> Result<()> {
        instructions::initialize_community(
            ctx,
            title,
            description,
            min_proposal_duration,
            nft_collection,
        )
    }

    pub fn initialize_member(ctx: Context<InitializeMember>) -> Result<()> {
        instructions::initialize_member(ctx)
    }

    pub fn initialize_proposer(ctx: Context<InitializeProposer>) -> Result<()> {
        instructions::initialize_proposer(ctx)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_type: ProposalType,
        end_date: i64,
    ) -> Result<()> {
        instructions::create_proposal(ctx, proposal_type, end_date)
    }
}
