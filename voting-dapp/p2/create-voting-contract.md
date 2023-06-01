## Create the Voting contract

Just like when we created the Staking contract earlier, we’ll use `cargo contract` to create the boilerplate for our Voting contract and update it so that it uses OpenBrush.

1. Create the new contract in the `contracts` directory.
    
    ```bash
    cargo contract new voting
    ```

1. Replace the contents of `contracts/voting/lib.rs` with the boilerplate code in the panel on the right. Note that that we're yet to create a lot of what is required to make this compilable.
    
    
1. Replace the contents of the `contracts/voting/Cargo.toml` file with:
    
    ```toml
    [package]
    name = "voting_contract"
    version = "1.0.0"
    edition = "2021"
    authors = ["The best developer ever"]

    [dependencies]
    ink = { version = "4.1.0", default-features = false }

    scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
    scale-info = { version = "2.3", default-features = false, features = ["derive"], optional = true }

    openbrush = { tag = "3.1.0", git = "https://github.com/Brushfam/openbrush-contracts", default-features = false, features = [] }
    dapp = { path = "../../src", default-features = false, features = ["voting"] }

    [lib]
    name = "voting_contract"
    path = "lib.rs"
    crate-type = [
        # Used for normal contract Wasm blobs.
        "cdylib",
    ]

    [features]
    default = ["std"]
    std = [
        "ink/std",
        "scale/std",
        "scale-info/std",
        "openbrush/std",
        "dapp/std"
    ]
    ink-as-dependency = []
    ```
    
3. In `src/Cargo.toml` add the `voting` features accordingly:
    
    ```toml
    // -- snip --
    # adds voting features to dapp
    voting = [] # <-- add this line
    ```

<!-- slide:break -->
<!-- tabs:start -->

#### **💡 Code explanation**

In `src/contracts/voting/lib.rs`:

- We’ve added a storage field for `VotingData`, which will hold the storage we'll create when we write the trait implementation for our voting logic
- We’ve implemented a Voting trait for the Voting contract’s storage, which we'll write in the next section
- We’ve created a constructor which will instantiate the contract using the address of our Staking contract, called `my_psp22_account`

In the contract's `Cargo.toml` file we've added `openbrush` and `dapp` (the top level project crate) as a dependency and specified `voting` as a feature this contract will use.

#### **👷 Boilerplate code**
   
```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod voting {
    // Imports from openbrush
    use dapp::impls::voting::{
        Data as VotingData,
        *,
    };
    use openbrush::traits::{Storage};

    #[ink(storage)]
    #[derive(Storage)]
    pub struct VotingContract {
        #[storage_field]
        voting: VotingData,
    }

    impl Voting for VotingContract {}

    impl VotingContract {
        #[ink(constructor)]
        pub fn new(my_psp22_account: AccountId) -> Self {
            Self {
                voting: VotingData::new(my_psp22_account),
            }
        }
    }
}
```

<!-- tabs:end -->

Once you complete this section, you'll notice that your code won't compile as is. If you have Rust analyzer turned on, it'll give you the following error:

```sh
missing ink! message
```

### Try it yourself!

Now that you're more familiar with extending your ink! contracts with OpenBrush from creating the Staking contract, you should be able to write the `voting` trait and implementation without much help! Feel free to try it out and use the next sections to check your work. Here's a high level overview of what you need to do:
- Add the a new trait and function signatures for the Voting contract, using `#[openbrush::trait_definition]`
- Add its respective trait implementation in the `impls` folder
- Make sure the modules in your project are linked correctly by specifying `voting` as a project feature
