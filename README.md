# HTLC Project

This project implements three Hash Time-Locked Contracts (HTLC) for facilitating secure swaps between two parties. The contracts include:

1. **Creator HTLC**: This contract allows the creator to lock funds with specific conditions (hash and time constraints) that must be met for the funds to be released.

2. **Taker HTLC**: This contract allows the taker to lock their funds under similar conditions, ensuring that both parties have their funds secured until the swap is completed.

3. **Resolver Contract**: This contract checks the status of both the creator and taker contracts. It resolves the swap by releasing the funds based on the conditions defined in the HTLC contracts.

## Project Structure

```
htlc-project
├── creator_htlc
│   ├── src
│   │   └── lib.rs
│   └── Cargo.toml
├── taker_htlc
│   ├── src
│   │   └── lib.rs
│   └── Cargo.toml
├── resolver
│   ├── src
│   │   └── lib.rs
│   └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- Access to a blockchain environment that supports smart contracts.

### Building the Contracts

To build the contracts, navigate to each contract directory and run:

```bash
cargo build
```

### Deploying the Contracts

1. Deploy the **Creator HTLC** contract first. Make sure to note the contract address.
2. Deploy the **Taker HTLC** contract next, also noting its address.
3. Finally, deploy the **Resolver Contract** which will manage the interactions between the creator and taker contracts.

### Usage

- The creator locks their funds in the Creator HTLC contract by specifying the conditions.
- The taker does the same in the Taker HTLC contract.
- The resolver contract can then be used to check the conditions and release the funds accordingly.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.