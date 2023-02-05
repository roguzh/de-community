use anchor_lang::prelude::*;

#[account]
pub struct Community {
    pub title: String,
    pub description: String,
    pub initializer: Pubkey,
    pub member_count: u32,
    pub proposer_count: u32,
    pub collection_nft: Option<Pubkey>
}

#[account]
pub struct Proposer {
    pub is_voted: bool,
    pub can_propose: bool,
    pub is_initializer: bool
}

#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub proposal_type: ProposalType,
    pub description: String,
    pub status: ProposalStatus,
    pub vote_option_yes: ProposalOption,
    pub vote_option_no: ProposalOption,
    pub init_date: i64,
    pub end_date: i64
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Initialized,
    Voting,
    Voted,
    Executed
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalType {
    Custom,
    AddMember,
    AddProposer,
    RemoveMember,
    RemoveProposer,
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProposalOption {
    pub option: String,
    pub vote_count: u64
}

// impl ProposalOption {
//     fn size(&self) -> u32{
//         return 8 + u32::try_from( self.option.len() ).unwrap();
//     }
// }

#[account]
pub struct Vote {
    pub is_voted: bool
}

#[account]
pub struct Member {
    pub can_vote: bool,
    pub reputation: u32
}