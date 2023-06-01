# Voting logic overview

At this point in the tutorial, you should have completed the steps to create the Staking contract.
In this part, we'll create the Voting contract to give the ability for users to propose things to be voted on and vote on proposals.

The purpose of the Voting contract is simple: it gives the ability for any user to be able to *propose* something for other users to vote on, whereby only users with voting power acquired by staking our dApp's PSP22 token are able to *vote* on something proposed. 

<img src="../assets/user-diagram-1.png"  width="300">  <img src="../assets/user-diagram-3.png"  width="300">  

Remember Jeremy, who we used as an example for a user who's acquired voting power from staking his tokens? 

Let’s imagine a second user, Felicia who wants to propose on something to vote on and also wants to vote on some existing proposal. 
While Felicia doesn't need to stake her tokens to propose something, she will need to in order to vote on a proposal.

<img src="../assets/user-diagram-4.png"  width="300">  

The rules we’ll enforce in our Voting contract:

- Any user can propose something to vote on
- Users must have at least 1_000_000 voting power units in order to vote on something
- Proposals must have at least two options
- TODO

<!-- slide:break -->
<!-- tabs:start -->

Below is what the UI could look like, where each proposal has title and at least 2 options.

<img src="../assets/gitcoin-snapshot.png" width="600"> 

*This is a screenshot taken from [Snapshot](https://snapshot.org/#/), a popular web3 DAO governance tool.*

<!-- tabs:end -->