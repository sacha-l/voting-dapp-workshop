# Create a default PSP22 contract implementation 

Follow these steps on this page to create a default PSP22 implementation with a custom constructor:

1. Inside the `contracts` directory of your `voting-dapp` project, create a new contract using Cargo Contract: 

    ```bash
    cargo contract new staking
    ```
    
1. OpenBrush comes with a PSP22 Wizard to easily create default PSP implementations. Go to the OpenBrush Wizard by scrolling to the bottom of the [openbrush.io](https://parity.link/BgsMh) homepage.  

1. Use the “Name” field to rename it to `Staking`, then paste it to replace the contents of `contracts/staking/lib.rs`.

1. Update the contract’s `contracts/staking/Cargo.toml` file by copying the contents provided by the PSP22 Wizard’s Cargo.toml section.

    > **Note**: *the tutorial assumes you’re using ink! version >= 4.1.0 and OpenBrush version >= 3.1.0. You may need to double check the Cargo.toml files have these updated before moving on.*

    You’ll notice that default implementation has this constructor:
    
    ```rust
    impl Staking {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
			_instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
			_instance
        }
    }
    ```
    The constructor is what is called when the contract in instantiated. Right now, all it’s doing is to mint some arbitrary supply of tokens to the account instantiating the contract. Because we want a fixed supply of tokens in circulation, we're going to modify this.

1. Modify the constructor so that it mints a fixed amount of `42_000_000` tokens by updating the `impl Staking` block with the following code: 

    ```rust
    impl Staking {
            /// Mint a fixed supply of 42_000_000 Staking tokens to be used for Voting.
            /// 
            /// Tokens are issued to the account instantiating this contract.
            #[ink(constructor)]
            pub fn new() -> Self {
                let mut _instance = Self::default();
                            _instance
                                ._mint_to(_instance.env().caller(), 42_000_000 * 10u128.pow(18))
                                .expect("Should mint"); 
                            _instance
                }
        }
    ```

    *File: `contracts/staking/lib.rs`. Note that we account for the decimals for denominations using `10u128.pow(18)`.*

    > **Tip:** run `cargo fmt --all` after pasting in code throughout the tutorial to ensure your code stays formatted. 

1. 
Before moving to the next step, run `cargo contract check` inside the Staking contract directory to make sure it compiles. 

```bash
cd staking
cargo contract check
```

Doesn’t compile? Check your `Cargo.toml` file is using the right dependency versions. File an issue if you're still having compilation problems! 😀

<!-- slide:break -->

<!-- tabs:start -->
#### **💡 About the total supply**

If we want the total supply to be `42_000_000` units, we need to account for the decimal places for the denominations of a single unit, which for this token will be 18. Therefore, the total supply is actually `42_000_000 * 10u128.pow(18)`.

<!-- tabs:end -->

<img src="../assets/wizard-1.png"  width="400"> 

*The view of the OpenBrush Wizard for updating the `contracts/staking/lib.rs` file*.

<img src="../assets/wizard-2.png"  width="400"> 

*The view of the OpenBrush Wizard for updating the `contracts/staking/Cargo.toml` file*.
