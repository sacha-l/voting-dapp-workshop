## Create the Voting contract

Just like when we created the Staking contract earlier, we’ll use `cargo contract` to create the boilerplate for our Voting contract and update it so that it uses OpenBrush.

1. Create the new contract in the `contracts` directory.
    
    ```bash
    cargo contract new voting
    ```

4. Replace the contents of `contracts/voting/lib.rs` with the following boilerplate:
    
    ```rust
    #![cfg_attr(not(feature = "std"), no_std)]
    #![feature(min_specialization)]
            
    #[openbrush::contract]
    pub mod voting {
        
      // Imports from openbrush
    	use openbrush::traits::Storage;
    	use dapp::impls::voting::{
    	        Data as VotingData,
    	        *,
    	    };
    
        #[ink(storage)]
        #[derive(Default, Storage)]
        pub struct VotingContract {
            #[storage_field]
            voting: VotingData,
        }
        
        impl Voting for VotingContract {}
    
        impl VotingContract {
            #[ink(constructor)]
            pub fn new(my_psp22_account: AccountId) -> Self {
                let mut instance = Self::default();
                instance.voting.my_psp22 = my_psp22_account;
                instance
            }
        }
    }
    ```
    
    Notice what this code is adding (and that we haven’t created a lot of what is required to make this compilable just yet!):
    
    - We’ve added a storage field for `VotingData`, which will hold this contract’s storage used when we create a custom trait for this contract
    - We’ve implemented a Voting trait for the Voting contract’s storage (which is yet to be written)
    - We’ve created a constructor which looks quite different from the constructor we made for the Staking contract. This is because when our Voting contract is instantiated, we want it to take in the address of our Staking contract (called `my_psp22`).
    
2. Replace the contents of the `contracts/voting/Cargo.toml` file with:
    
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
    
    # Include brush as a dependency and enable default implementation for PSP22 via brush feature
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

You'll notice that this won't compile as is. If you have Rust analyzer in the background, it'll give you the following error:

```sh
missing ink! message
```

But now that you're more familiar with extending your ink! contracts with OpenBrush from creating the Staking contract, you should be able to write the `voting` trait and implementation using OpenBrush without much help! Feel free to try it out and use the next sections to check your work. Here's a high level overview of what you need to do:
- Add the a new trait for the Voting contract, using `#[openbrush::trait_definition]`
- Add its respective trait implementation in the `impls` folder
- Make sure everything is linked correctly


