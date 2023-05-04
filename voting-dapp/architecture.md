## Application design and overview

This tutorial can be completed in 3 parts, each section building on one another:

* In the first part, you will learn how to build a P2P22 Staking contract. Our Voting dapp’s logic is based on the fact that users need to acquire this special PSP22 token in order to vote or propose items to be voted on. For simplicity, we assume that the tokens will be distributed by the contract owner.

* In the second part, we'll create a separate Voting contract which can make call to the Staking contract. We’ll also write tests for both contracts. 

* In the third and final part, we’ll add a front-end that will allow users to vote on proposals.

<!-- slide:break -->

<!-- tabs:start -->

</br> 
</br> 

<img src="../assets/dapp-architecture.png" width="500"> 

*Overview of the architecture of the Voting dApp we're building.*

<!-- tabs:end -->