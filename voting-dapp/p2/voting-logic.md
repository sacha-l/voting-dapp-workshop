# Voting logic overview

This part of the tutorial assumes you have a working OpenBrush project with the Staking contract we built in the first part.

The purpose of the Voting contract is simple: any user should be able to *propose* something for other users to vote, while only users with voting power should be able to *vote* on something proposed. 

![Untitled_2023-04-18_12-16-50.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/3a2958b7-7fd6-4572-8ef9-b0dd584b4ed8/Untitled_2023-04-18_12-16-50.png)

For her to propose something to vote on, she just needs to pay a fee. 

![Untitled_2023-04-18_12-14-24.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/cfc9dc30-6160-4d7b-9e5b-74857e5a0301/Untitled_2023-04-18_12-14-24.png)

Remember Jeremy, who we used as an example for a user that managed to acquire Voting Power by staking some tokens? 

Let’s imagine a second user, Felicia who wants to propose on something to vote on and also wants to vote on some existing proposal. 

And if she wants to vote, she’ll need some Voting Power. 

![Untitled_2023-04-18_12-23-23.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/6ef5648e-9e37-4cbf-866b-33518b0fcc00/Untitled_2023-04-18_12-23-23.png)

The rules we’ll enforce in our Voting contract:

- Any user can propose something to vote on
- Users must have at least 1_000_000 voting power units in order to vote on something