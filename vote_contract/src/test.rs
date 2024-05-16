#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, Env, log, Symbol, symbol_short, storage::HashMap};

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub options: HashMap<Symbol, u64>,
    pub total_votes: u64,
    pub expiry_time: u64, // Timestamp when the poll expires
}

#[contracttype]
pub struct Registry {
    registered_voters: HashMap<Symbol, ()>, // Track registered voters
}

#[contracttype]
#[derive(Clone)]
pub struct Record {
    pub name: Symbol,
    pub selected: Symbol,
    pub votes: u64,
    pub time: u64,
}

#[contract]
pub struct VoteContract;

#[contractimpl]
impl VoteContract {

    // Record votes for a given user and option
    pub fn record_votes(env: Env, user: Symbol, votes: u64, selected: Symbol) {
        // Check if the user is registered
        if !Self::is_registered_voter(&env, &user) {
            panic!("User is not registered to vote");
        }

        // Check if the poll has expired
        let poll = Self::view_poll(env.clone());
        if env.ledger().timestamp() > poll.expiry_time {
            panic!("Poll has expired, voting is closed");
        }

        // Check if the user has already voted
        if Self::has_voted(&env, &user) {
            panic!("User has already voted");
        }

        // Check if the selected option exists in the poll
        if !poll.options.contains_key(&selected) {
            panic!("Selected option does not exist in the poll");
        }

        // Check if the number of votes is valid
        if votes == 0 || votes > 5 {
            panic!("Invalid number of votes");
        }

        // Record the vote
        let mut records = Self::view_voter(env.clone(), user.clone());
        records.name = user.clone();
        records.selected = selected;
        records.votes = votes;
        records.time = env.ledger().timestamp();

        // Update poll counts
        let mut poll = poll.clone();
        *poll.options.get_mut(&selected).unwrap() += votes;
        poll.total_votes += votes;

        // Save updated records and poll
        env.storage().persistent().set(&Registry::Record(user), &records);
        env.storage().persistent().set(&Registry::Poll, &poll);

        log!(&env, "Votes Registered!");
    }

    // Check if a user is a registered voter
    fn is_registered_voter(env: &Env, user: &Symbol) -> bool {
        let registry: Registry = env.storage().persistent().get(&Registry::registered_voters).unwrap_or_default();
        registry.registered_voters.contains_key(user)
    }

    // Check if a user has already voted
    fn has_voted(env: &Env, user: &Symbol) -> bool {
        let record: Record = env.storage().persistent().get(&Registry::Record(user)).unwrap_or_default();
        record.time != 0
    }

    // Get the current poll details
    pub fn view_poll(env: Env) -> Poll {
        env.storage().instance().get(&Registry::Poll).unwrap_or_default()
    }
    
    // Get the voting record for a user
    pub fn view_voter(env: Env, voter: Symbol) -> Record {
        env.storage().instance().get(&Registry::Record(voter.clone())).unwrap_or_default()
    }
}
