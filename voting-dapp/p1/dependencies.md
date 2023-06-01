# Link dependencies

We use Rust features to link the Staking implementation to the ink! contract.
This will allow you to include the storage of the Staking trait implementation to the ink! contract, otherwise ink! has no idea about it.

### Include the staking implementation as a project feature

1. Replace the content of `src/impls/mod.rs` with the following code to include the Staking implementation as a feature for your dApp: 

    ```rust
    #[cfg(feature = "staking")]
    pub mod staking;

    #[cfg(feature = "staking")]
    pub use staking::*;
    ```

1. Add the Staking feature to end of the `src/Cargo.toml` file:
    ```toml
    // -- snip --
    # adds staking features to dApp library
    staking = []
    ```

1. In `contracts/staking/Cargo.toml`, add `src` as dependency called `dapp` with the staking feature included:

    ```toml
    [dependencies]
    // -- snip --
    dapp = { path = "../../src", default-features = false, features = ["staking"] }
    // -- snip --
    ```

1. Include it in the `std` features too:
    
    ```toml
    // -- snip --
    std = [
    // -- snip --
    "dapp/std", # add this line
    ]
    ```

### Add `StakingData` to your contract

1. In `contracts/staking/lib.rs` import the `Data` struct from our staking implementation and create an alias called `StakingData`:

    ```rust
    // Imports from the Staking impl
    use dapp::impls::staking::{
        Data as StakingData,
    };
    ```

1. Add it to the Staking storage struct:

    ```rust
        #[ink(storage)]
        #[derive(Default, Storage)]
        pub struct Staking {
            #[storage_field]
            psp22: psp22::Data,
            #[storage_field]      // <- add this line
            staking: StakingData, // <- add this line
        }
    ```

1. Finally, implement the Staking trait for the contract by adding:

    ```rust
    impl dapp::Staking for Staking {}
    ```

### Check your contract compiles 

In the `contracts/staking` directory, run `cargo check` to check your contract compiles:

```bash
cargo contract check
```

<!-- slide:break -->

<!-- tabs:start -->

#### **✅ Final code for `contracts/staking/lib.rs`**

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    // imports from openbrush
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };
    // imports from the Staking impl
    use dapp::impls::staking::Data as StakingData;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Staking {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field] // <- add this line
        staking: StakingData, // <- add this line
    }

    // contains default implementation without any modifications
    impl PSP22 for Staking {}

    // contains the extended logic to the PSP22 contract
    impl dapp::Staking for Staking {}

    impl Staking {
        /// Mint a fixed supply of 42_000_000 Staking tokens to be used for Voting.
        ///
        /// Tokens are issued to the account instantiating this contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
                ._mint_to(instance.env().caller(), 42_000_000 * 10u128.pow(18))
                .expect("Should mint");
            instance
        }
    }
}
```
<!-- tabs:end -->

Congratulations! If you've managed to check that your contract code will compile, you're ready to move onto the next part of this tutorial and create the voting contract. You should see this message in the terminal after running `cargo contract check`:

```sh
Your contract's code was built successfully.
```