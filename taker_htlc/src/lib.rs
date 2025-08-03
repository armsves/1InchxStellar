// filepath: /htlc-project/taker_htlc/src/lib.rs
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub struct TakerHTLC {
    pub creator: Address,
    pub taker: Address,
    pub hashlock: [u8; 32],
    pub timelock: u64,
    pub amount: i128,
    pub is_claimed: bool,
}

#[contract]
pub struct TakerHTLCContract;

#[contractimpl]
impl TakerHTLCContract {
    pub fn create_htlc(
        env: Env,
        creator: Address,
        taker: Address,
        hashlock: [u8; 32],
        timelock: u64,
        amount: i128,
    ) {
        // Ensure the contract is initialized with the correct parameters
        creator.require_auth();

        let htlc = TakerHTLC {
            creator,
            taker,
            hashlock,
            timelock,
            amount,
            is_claimed: false,
        };

        env.storage().instance().set(&hashlock, &htlc);
    }

    pub fn claim(env: Env, hashlock: [u8; 32], secret: &[u8; 32]) {
        let htlc: TakerHTLC = env.storage().instance().get(&hashlock).unwrap();

        // Verify the secret matches the hashlock
        if !verify_secret(&htlc.hashlock, secret) {
            panic!("Invalid secret");
        }

        // Ensure the contract has not been claimed yet
        if htlc.is_claimed {
            panic!("Funds already claimed");
        }

        // Ensure the caller is the taker
        env.current_contract_address().require_auth();

        // Transfer the funds to the taker
        token::Client::new(&env, &htlc.amount).transfer(
            &env.current_contract_address(),
            &htlc.taker,
            &htlc.amount,
        );

        // Mark the contract as claimed
        let mut updated_htlc = htlc;
        updated_htlc.is_claimed = true;
        env.storage().instance().set(&hashlock, &updated_htlc);
    }

    pub fn refund(env: Env, hashlock: [u8; 32]) {
        let htlc: TakerHTLC = env.storage().instance().get(&hashlock).unwrap();

        // Ensure the caller is the creator
        htlc.creator.require_auth();

        // Check if the timelock has expired
        let current_time = env.ledger().timestamp();
        if current_time < htlc.timelock {
            panic!("Timelock has not expired yet");
        }

        // Ensure the contract has not been claimed yet
        if htlc.is_claimed {
            panic!("Funds already claimed");
        }

        // Transfer the funds back to the creator
        token::Client::new(&env, &htlc.amount).transfer(
            &env.current_contract_address(),
            &htlc.creator,
            &htlc.amount,
        );

        // Remove the HTLC from storage
        env.storage().instance().remove(&hashlock);
    }
}

fn verify_secret(hashlock: &[u8; 32], secret: &[u8; 32]) -> bool {
    // Implement the hash verification logic here
    // For example, using SHA256 or another hashing algorithm
    // This is a placeholder for the actual implementation
    hashlock == secret
}