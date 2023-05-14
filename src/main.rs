// Import required dependencies
use ink_lang::contract;
use ink_prelude::string::String;
use ink_storage::{
    collections::{HashMap as StorageHashMap},
    traits::{PackedLayout, SpreadLayout},
};
use ink_env::Balance;

mod chama_dao {
    use ink_storage::{
        collections::{HashMap as StorageHashMap},
        traits::{PackedLayout, SpreadLayout},
        Lazy,
    };
    
    // Define the structure of a proposal
    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    pub struct Proposal {
        pub proposer: AccountId,
        pub description: String,
        pub votes: u32,
        pub approved: bool,
    }
    
    // Define the structure of a member
    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    pub struct Member {
        pub wallet_address: AccountId,
        pub voting_power: u32,
    }
    
    // Define the Chama DAO contract
    #[ink(storage)]
    pub struct ChamaDao {
        // Members of the Chama DAO
        members: StorageHashMap<AccountId, Lazy<Member>>,
        // Proposals submitted to the Chama DAO
        proposals: StorageHashMap<u32, Lazy<Proposal>>,
        // Total number of proposals submitted
        total_proposals: u32,
        // Chama DAO fund balance
        balance: Balance,
    }
    
    impl ChamaDao {
        // Constructor function for the Chama DAO
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                members: StorageHashMap::new(),
                proposals: StorageHashMap::new(),
                total_proposals: 0,
                balance: 0,
            }
        }
        
        // Add a member to the Chama DAO
        #[ink(message)]
        pub fn add_member(&mut self, wallet_address: AccountId, voting_power: u32) {
            let member = Member {
                wallet_address,
                voting_power,
            };
            self.members.insert(wallet_address, Lazy::new(member));
        }
        
        // Create a new proposal
        #[ink(message)]
        pub fn create_proposal(&mut self, description: String) -> u32 {
            let proposal = Proposal {
                proposer: Self::env().caller(),
                description,
                votes: 0,
                approved: false,
            };
            let proposal_id = self.total_proposals;
            self.proposals.insert(proposal_id, Lazy::new(proposal));
            self.total_proposals += 1;
            proposal_id
        }
        
        // Vote on a proposal
        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) {
            let proposal = self.proposals.get_mut(&proposal_id).unwrap();
            let voter = self.members.get(&Self::env().caller()).unwrap();
            proposal.votes += voter.voting_power;
        }
        
        // Disburse funds to an approved proposal
        #[ink(message)]
        pub fn disburse_funds(&mut self, proposal_id: u32, amount: Balance) {
            let proposal = self.proposals.get_mut(&proposal_id).unwrap();
            assert!(proposal.approved, "Proposal has not been approved");
            assert!(self.balance >= amount, "Insufficient funds");
            self.balance -= amount;
            Self::env().transfer(proposal.proposer, amount).unwrap();
        }
        
        // Get the balance of the Chama DAO
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.balance
        }
    
        // Get the number of members in the Chama DAO
        #[ink(message)]
        pub fn get_num_members(&self) -> u32 {
            self.members.len() as u32
        }
            // Get the number of proposals submitted to the Chama DAO
    #[ink(message)]
    pub fn get_num_proposals(&self) -> u32 {
        self.total_proposals
    }
    
    // Get the details of a proposal
    #[ink(message)]
    pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
        self.proposals.get(&proposal_id).map(|p| (*p).clone())
    }
    
    // Get the voting power of a member
    #[ink(message)]
    pub fn get_member_voting_power(&self, member_address: AccountId) -> Option<u32> {
        self.members.get(&member_address).map(|m| (*m).voting_power)
    }
}
}