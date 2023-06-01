## Add a trait definition for Staking

A common pattern in developing ink! contracts with OpenBrush is to first define your traits and then implement them in their own modules. This keeps your project organized, easy to review and extend. But because it uses Cargo features to link all trait and impl files back to the main contract, it's common to run into compilation errors if these features have not been updated in the project's Cargo.toml file.

Before writing the Staking trait, update the `openbrush` dependency in the `Cargo.toml` in the root of your project so it includes the PSP22 feature. Replace the line that has the `openbrush` dependency with:

```toml
openbrush = { tag = "3.1.0", git = "https://github.com/Brushfam/openbrush-contracts", default-features = false, features = ["psp22"] }
```

### Define the Staking trait

1. In the `src/traits` folder of our project, create a Rust module called `staking.rs`.
2. In the `src/traits/mod.rs` file, make this module accessible to the rest of your project by adding:
    
    ```rust
    pub mod staking;
    ```
    
3. In the `staking.rs` file, add the following code :
    
    ```rust
    pub use openbrush::{
        contracts::psp22::*,
        traits::{
            AccountId,
            Balance,
        },
    };
    
    #[openbrush::wrapper]
    pub type StakingRef = dyn Staking + PSP22;
    
    #[openbrush::trait_definition]
    pub trait Staking {
        #[ink(message)]
        fn stake(&mut self, amount: Balance) -> Result<(), PSP22Error>;
    
        #[ink(message)]
        fn unstake(&mut self, amount: Balance) -> Result<(), PSP22Error>;
    
        #[ink(message)]
        fn voting_power(&self, account: AccountId) -> u128;
    }
    ```

### Create the Staking contract error type

In the Staking trait definition you just wrote, we used the`PSP22Error` type [from the PSP22 standard](https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md#Errors). But you'll want your Staking contract to emit more descriptive errors. To do this, you will need to create a custom error type called `StakingErr` and update the trait definition to use it.

For brevity, we won’t explain too much here. Just know that `PSP22Error` conveniently has an item called `Custom` which takes a String parameter which we use to pass in items from our custom error enum.

1. Create a new file called `errors.rs` in `src/libs` and paste in the following code:
    
    ```rust
    use openbrush::{
        contracts::psp22::PSP22Error,
        traits::String,
    };
    use scale::{
        Decode,
        Encode,
    };
    
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum StakingErr {
        PSP22Error(PSP22Error),
    
        // Custom errors go here
    
       }
    
    impl From<StakingErr> for PSP22Error {
        fn from(err: StakingErr) -> Self {
            match err {
                StakingErr::PSP22Error(err) => err,
                _ => PSP22Error::Custom(String::from("Custom")),
            }
        }
    }
    
    impl From<PSP22Error> for StakingErr {
        fn from(err: PSP22Error) -> Self {
            StakingErr::PSP22Error(err)
        }
    }
    ```
    
2. Add in the following errors by replacing the line that says `//Custom errors go here` with:
    
    ```rust
    	LockingPeriodNotEnded,
        AmountMustBeAboveZero,
        NothingToWithdraw,
    ```
    
3. Update the `src/libs/mod.rs` file to add the error module to your project’s scope:
    
    ```rust
    pub mod errors;
    ```
    
4. Go back to `src/traits/staking.rs`  and import `StakingErr`:
    
    ```rust
    pub use crate::libs::errors::StakingErr;
    ```
    

5. Update the trait function signature return types with the new `StakingErr` type:

```rust
// -- snip --

#[openbrush::trait_definition]
pub trait Staking {
    #[ink(message)]
    fn stake(&mut self, amount: Balance) -> Result<(), StakingErr>;

    #[ink(message)]
    fn unstake(&mut self, amount: Balance) -> Result<(), StakingErr>;

// -- snip --
}
```

<!-- slide:break -->

<!-- tabs:start -->

#### **💡Code explanation**

We’ve added the Staking trait's function signatures and created a custom error type for the staking functions to return. Just like declaring any publicly callable function in an ink! contract, notice how each function signature is annotated with `#[ink(message)]`. This ensures that the methods in this trait are callable when we implement it for the staking contract. 

You’ll also notice that OpenBrush is doing some macro magic for us too:

- `#[openbrush::wrapper]` : this provides a wrapper around the storage declaration to pass it into our ink! contract.
- `#[openbrush::traits]`: this allows us to define a trait in a separate file that can be used by our generic Staking contract.

The `errors.rs` file and error code we created will allow us to return specific errors using descriptive enums relevant to our Staking trait implementation, overriding the default `PSP22Error` type.


#### **✅ Final code for `src/traits/staking.rs`**

```rust
pub use crate::libs::errors::StakingErr;
pub use openbrush::{
    contracts::psp22::*,
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type StakingRef = dyn Staking + PSP22;

#[openbrush::trait_definition]
pub trait Staking {
    #[ink(message)]
    fn stake(&mut self, amount: Balance) -> Result<(), StakingErr>;

    #[ink(message)]
    fn unstake(&mut self, amount: Balance) -> Result<(), StakingErr>;

    #[ink(message)]
    fn voting_power(&self, account: AccountId) -> u128;
}
```

#### **✅ Final code for `src/libs/errors.rs`**

```rust
use openbrush::{
    contracts::psp22::PSP22Error,
    traits::String,
};
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StakingErr {
    PSP22Error(PSP22Error),
    LockingPeriodNotEnded,
    AmountMustBeAboveZero,
    NothingToWithdraw,
}

impl From<StakingErr> for PSP22Error {
    fn from(err: StakingErr) -> Self {
        match err {
            StakingErr::PSP22Error(err) => err,
            _ => PSP22Error::Custom(String::from("Custom")),
        }
    }
}

impl From<PSP22Error> for StakingErr {
    fn from(err: PSP22Error) -> Self {
        StakingErr::PSP22Error(err)
    }
}
```

<!-- tabs:end -->

At this point, the code you just added isn't actually linked to the ink! contract so running `cargo contract check` won't do anything. You will do this in the [Link dependencies](./dependencies) section later on. If you are using Rust Analyzer, it should be happy at this point. 😊