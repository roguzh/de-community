use anchor_lang::prelude::*;

// use crate::instructions::create_manage_member_proposal::{APPROVAL_TEXT, DENIAL_TEXT};

#[account]
pub struct Community {
    pub title: String,
    pub description: String,
    pub initializer: Pubkey,
    pub voter_member_count: u32,
    pub proposer_count: u32,
    pub proposal_count: u64,
    pub min_proposal_duration: i64,
    pub collection_nft: Option<Pubkey>,
}

#[account]
pub struct Proposer {
    pub is_voted: bool,
    pub can_propose: bool,
    pub proposal_count: u64,
    pub owner: Pubkey,
}

#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub status: ProposalStatus,
    pub approval_count: ProposalOption,
    pub denial_count: ProposalOption,
    pub init_date: i64,
    pub end_date: i64,
    pub proposal_type: ProposalType,
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalType {
    Custom {
        title: String,
        description: Option<String>,
    },
    ManageMember {
        action: ManageActionType,
        voted_account: Pubkey,
        voted_account_owner: Pubkey,
    },
}

impl ProposalType {
    pub fn get_proposal_type_size(proposal_type: ProposalType) -> usize {
        let mut proposal_size = 1;

        match proposal_type {
            ProposalType::Custom {
                ref title,
                ref description,
            } => {
                proposal_size += 4 + title.len();
                match description {
                    Some(desc_str) => {
                        proposal_size += 1 + (4 + desc_str.len());
                    }
                    None => {
                        proposal_size += 1;
                    }
                }
            }
            ProposalType::ManageMember {
                action: _,
                voted_account: _,
                voted_account_owner: _,
            } => {
                proposal_size += 2 + 32 + 32;
            }
        }
        return proposal_size;
    }
}

#[derive(Debug, PartialEq, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Voting,
    Voted,
    Executed,
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
    pub vote_count: u64,
}

#[account]
pub struct Vote {
    pub is_voted: bool,
}

#[account]
pub struct Member {
    pub can_vote: bool,
    pub reputation: u32,
}
