# Customize your contract

The way we’re using the PSP22 standard is similar to the concept of inheritance in many programming languages. In the next few sections, we’ll use types and functions from the PSP22 standard to add custom functionality to our Staking contract. These include:

- `PSP22Data` : We already have the storage item for `PSP22Data` declared in the Staking storage field, which holds all data from the standard PSP22 implementation.
- `transfer_from` and `transfer`: We will need these when we extend our staking contract to implement `stake` and `unstake` respectively.

Before we jump into coding, let’s remind ourselves what our custom logic needs to look like:

- `fn stake`. The caller of this function will input some amount to *stake*: if the caller of this function already has some tokens staked, add the amount to the amount they already have staked. Otherwise just stake this amount.
- `fn unstake`. The caller of this function will input the amount they want to *unstake*: if this amount is greater than the amount they have staked, transfer all their stake back to the caller. Otherwise, remove the amount from their total amount staked and transfer just that amount back to their account.
- `fn voting_power`. This function is designed for the voting contract to call: it will return a `u128` calculated based on the amount staked and time left until the unlocking period is passed. If there’s no stake, then it returns `0`.

<!-- slide:break -->

<!-- tabs:start -->

<img src="../assets/contract-interface.png" width="400"> 

*The staking contract "inherits" the functions and traits from the standard PSP22 implementation*. 

<!-- tabs:end -->