#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, BytesN};

#[derive(Clone)]
#[contracttype]
pub struct CreatorHTLC {
    pub hash: BytesN<32>,
    pub lock_time: u64,
    pub creator: Address,
    pub token: Address,         // Token contract address
    pub amount: i128,
    pub evm_address: BytesN<20>, // EVM address for destination chain
    pub is_released: bool,
}

#[contract]
pub struct CreatorHTLCContract;

#[contractimpl]
impl CreatorHTLCContract {
    /// The creator must approve the contract to spend `amount` of `token` before calling this.
    pub fn create_htlc(
        env: Env,
        hash: BytesN<32>,
        lock_time: u64,
        token: Address,
        amount: i128,
        evm_address: BytesN<20>,
    ) {
        let creator = env.current_contract_address(); // The user creating the HTLC

        // Transfer tokens from creator to contract (escrow)
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&creator, &env.current_contract_address(), &amount);

        let htlc = CreatorHTLC {
            hash: hash.clone(),
            lock_time,
            creator: creator.clone(),
            token: token.clone(),
            amount,
            evm_address: evm_address.clone(),
            is_released: false,
        };

        // Store the HTLC in the contract's storage
        env.storage().instance().set(&hash, &htlc);
    }

    pub fn release_funds(env: Env, hash: BytesN<32>, preimage: BytesN<32>) {
        let mut htlc: CreatorHTLC = env.storage().instance().get(&hash).unwrap();

        if htlc.is_released {
            panic!("Funds already released");
        }

        // Check if the preimage hashes to the expected hash
        if Self::hash_preimage(&env, preimage.clone()) != htlc.hash {
            panic!("Invalid preimage");
        }

        // Transfer funds to the creator (or recipient, depending on your logic)
        let token_client = token::Client::new(&env, &htlc.token);
        token_client.transfer(
            &env.current_contract_address(),
            &htlc.creator,
            &htlc.amount,
        );

        htlc.is_released = true;
        env.storage().instance().set(&hash, &htlc);
    }

    pub fn hash_preimage(env: &Env, preimage: BytesN<32>) -> BytesN<32> {
        env.crypto().sha256(&preimage.into()).into()
    }
}