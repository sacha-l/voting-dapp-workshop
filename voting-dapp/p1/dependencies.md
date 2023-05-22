# Link dependencies

You're almost ready to compile your contract and interact with it. But before jumping to that, you'll need to link the Staking implementation then ink! contract. In OpenBrush, this is done using Rust features.

Once you update your project to include the Staking impl as a feature, you'll be able to include the storage of the Staking trait implementation to the ink! contract.

### Include the staking implementation as a project feature

1. Replace the content of `src/impls/mod.rs` with the following code to include the Staking implementation as a feature for your dapp: 

    ```rust
    #[cfg(feature = "staking")]
    pub mod staking;

    #[cfg(feature = "staking")]
    pub use staking::*;
    ```

1. Add the Staking feature to end of the `src/Cargo.toml` file:
    ```toml
    // -- snip --
    # adds staking features to dapp library
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
