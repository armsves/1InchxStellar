// filepath: /htlc-project/htlc-project/resolver/src/lib.rs
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub struct HTLC {
    pub creator: Address,
    pub taker: Address,
    pub hash: [u8; 32],
    pub timeout: u64,
    pub amount: i128,
    pub is_resolved: bool,
}

#[contract]
pub struct ResolverContract;

#[contractimpl]
impl ResolverContract {
    pub fn resolve(
        env: Env,
        htlc_creator: Address,
        htlc_taker: Address,
        secret: [u8; 32],
    ) {
        let htlc: HTLC = env.storage().instance().get(&htlc_creator).unwrap();

        if htlc.is_resolved {
            panic!("HTLC has already been resolved");
        }

        if htlc.hash != hash(secret) {
            panic!("Invalid secret");
        }

        // Transfer funds to the taker
        token::Client::new(&env, &htlc.token).transfer(
            &env.current_contract_address(),
            &htlc.taker,
            &htlc.amount,
        );

        // Mark the HTLC as resolved
        env.storage().instance().set(&htlc_creator, &HTLC {
            is_resolved: true,
            ..htlc
        });
    }

    pub fn timeout(
        env: Env,
        htlc_creator: Address,
    ) {
        let htlc: HTLC = env.storage().instance().get(&htlc_creator).unwrap();

        if env.ledger().timestamp() < htlc.timeout {
            panic!("HTLC has not timed out yet");
        }

        if htlc.is_resolved {
            panic!("HTLC has already been resolved");
        }

        // Transfer funds back to the creator
        token::Client::new(&env, &htlc.token).transfer(
            &env.current_contract_address(),
            &htlc.creator,
            &htlc.amount,
        );

        // Mark the HTLC as resolved
        env.storage().instance().set(&htlc_creator, &HTLC {
            is_resolved: true,
            ..htlc
        });
    }

    fn hash(secret: [u8; 32]) -> [u8; 32] {
        // Implement a hashing function (e.g., SHA256) to hash the secret
        // This is a placeholder for the actual hashing logic
        secret
    }
}