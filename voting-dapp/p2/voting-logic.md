# Voting logic overview

This part of the tutorial assumes you have a working OpenBrush project with the Staking contract we built in the first part.

The purpose of the Voting contract is simple: any user should be able to *propose* something for other users to vote, while only users with voting power should be able to *vote* on something proposed. 

<img src="../assets/user-diagram-1.png"  width="300">  

For her to propose something to vote on, she just needs to pay a fee. 

<img src="../assets/user-diagram-3.png"  width="300">  

Remember Jeremy, who we used as an example for a user that managed to acquire Voting Power by staking some tokens? 

Let’s imagine a second user, Felicia who wants to propose on something to vote on and also wants to vote on some existing proposal. 

And if she wants to vote, she’ll need some Voting Power. 

<img src="../assets/user-diagram-4.png"  width="300">  

The rules we’ll enforce in our Voting contract:

- Any user can propose something to vote on
- Users must have at least 1_000_000 voting power units in order to vote on something