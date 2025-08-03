#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, BytesN, String};

#[derive(Clone)]
#[contracttype]
pub struct CreatorHTLC {
    pub hash: u32,
    pub lock_time: u64,
    pub creator: Address,
    pub token: Address,
    pub amount: i128,
    pub evm_address: BytesN<20>,
    pub is_released: bool,
    pub dest_token: String,
    pub dest_amount: i128,
}

#[contract]
pub struct CreatorHTLCContract;

#[contractimpl]
impl CreatorHTLCContract {
    pub fn __constructor(
        env: Env,
        creator: Address,
        hash: u32,
        lock_time: u64,
        token: Address,
        amount: i128,
        evm_address: BytesN<20>,
        dest_token: String,
        dest_amount: i128,
    ) {
        creator.require_auth();

        let absolute_lock_time = env.ledger().timestamp() + lock_time;

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&creator, &env.current_contract_address(), &amount);

        let htlc = CreatorHTLC {
            hash,
            lock_time: absolute_lock_time,
            creator: creator.clone(),
            token: token.clone(),
            amount,
            evm_address: evm_address.clone(),
            is_released: false,
            dest_token: dest_token.clone(),
            dest_amount,
        };

        env.storage().instance().set(&hash, &htlc);
    }

    pub fn release_funds(env: Env, hash: u32, preimage: u32) {
        let mut htlc: CreatorHTLC = env.storage().instance().get(&hash).unwrap();

        if htlc.is_released {
            panic!("Funds already released");
        }

        let current_time = env.ledger().timestamp();
        if current_time >= htlc.lock_time {
            panic!("Lock time has already passed");
        }

        if Self::hash_preimage(preimage) != htlc.hash {
            panic!("Invalid preimage");
        }

        let token_client = token::Client::new(&env, &htlc.token);
        token_client.transfer(
            &env.current_contract_address(),
            &htlc.creator,
            &htlc.amount,
        );

        htlc.is_released = true;
        env.storage().instance().set(&hash, &htlc);
    }

    pub fn rescue_funds(env: Env, hash: u32, preimage: u32) {
        let mut htlc: CreatorHTLC = env.storage().instance().get(&hash).unwrap();

        if htlc.is_released {
            panic!("Funds already released");
        }

        if Self::hash_preimage(preimage) != htlc.hash {
            panic!("Invalid preimage");
        }

        let current_time = env.ledger().timestamp();
        if current_time < htlc.lock_time {
            panic!("Lock time has not yet passed");
        }

        let token_client = token::Client::new(&env, &htlc.token);
        token_client.transfer(
            &env.current_contract_address(),
            &htlc.creator,
            &htlc.amount,
        );

        htlc.is_released = true;
        env.storage().instance().set(&hash, &htlc);
    }

    pub fn hash_preimage(preimage: u32) -> u32 {
        preimage + 1 // Replace with a real hash in production
    }
}