use anchor_lang::prelude::*;

#[account]
pub struct Community {
    pub title: String,
    pub description: String,
    pub initializer: Pubkey,
    pub voter_member_count: u32,
    pub proposer_count: u32,
    pub proposal_count: u64,
    pub min_proposal_duration: i64,
    pub collection_nft: Option<Pubkey>
}

#[account]
pub struct Proposer {
    pub is_voted: bool,
    pub can_propose: bool,
    pub proposal_count: u64,
    pub is_initializer: bool
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BaseProposal {
    pub proposer: Pubkey,
    pub status: ProposalStatus,
    pub approval_count: ProposalOption,
    pub denial_count: ProposalOption,
    pub init_date: i64,
    pub end_date: i64
}

#[account]
pub struct CustomProposal {
    pub proposal: BaseProposal,
    pub title: String,
    pub description: Option<String>,
}

#[account]
pub struct ManageMemberProposal {
    pub proposal: BaseProposal,
    pub action: ManageActionType,
    pub voted_account: Pubkey,
    pub voted_account_owner: Pubkey
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Voting,
    Voted,
    Executed
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ManageActionType {
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

#[account]
pub struct Vote {
    pub is_voted: bool
}

#[account]
pub struct Member {
    pub can_vote: bool,
    pub reputation: u32
}