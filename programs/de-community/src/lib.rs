use anchor_lang::prelude::*;
use instructions::*;

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
        nft_collection: Option<Pubkey>
    ) -> Result<()> {
        instructions::initialize_community(ctx, title, description, nft_collection)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
