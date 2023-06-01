# Deploy your contracts using the Contracts UI

Congratulations! If you've made it this far, you are ready to deploy your contract on-chain and interact with it.
You'll learn how to deploy your dApp using the Contracts UI.

> Make sure you have some ROC tokens to deploy your dApp. Use the [faucet here](https://parity.link/4e43J) to get some.

1. Build the Staking contract by typing the following command inside the `contracts/staking` directory:
    
    ```bash
    cargo +nightly-2023-02-07 contract build
    ```

    This will create a file called `my_psp.contract` in the `target` path of your project. You should see an output similar to:
    
    ```sh
    Your contract artifacts are ready. You can find them in:
    /Users/Name/voting-dapp-tutorial/solution/target/ink/my_psp22

    - my_psp22.contract (code + metadata)
    - my_psp22.wasm (the contract's code)
    - my_psp22.json (the contract's metadata)
  ```

  1. Build the Voting contract by typing the following command inside the `contracts/voting` directory:
    
    ```bash
    cargo +nightly-2023-02-07 contract build
    ```

    This will create a file called `voting_contract.contract` in the `target` path of your project. You should see an output similar to:
    
    ```sh
    Your contract artifacts are ready. You can find them in:
    /Users/Name/voting-dapp-tutorial/solution/target/ink/voting_contract

    - voting_contract.contract (code + metadata)
    - voting_contract.wasm (the contract's code)
    - voting_contract.json (the contract's metadata)
  ```

1. Go to the Contracts UI and select Contracts (Rococo) as the node you're connecting to.

    <img src="../assets/contracts-ui-1.png" width="500">

1. Once you're connected, select "Upload a new contract" and upload the `my_psp22.contract` file from the `/voting-dapp/target/ink/my_psp22` directory. Once you upload it you should see something like this:

    <img src="../assets/contracts-ui-2.png" width="500">

1. Scroll to the bottom of the page and click "Next". This will bring you to another page as shown below. Also click "Next".

    <img src="../assets/contracts-ui-3.png" width="500">

    Sign the transaction using your browser wallet extension.

1. Copy the contract code to your clipboard.

1. Deploy the `voting` contract using the Staking contract account ID in the constructor.

1. Interact with the contract using the UI and explore the different methods provided by the PSP22 standard as well as our new Staking and Voting functions.

<!-- slide:break -->

<!-- tabs:start -->

#### **💡 Where can I deploy my dApp?**

Read more on where you can deploy your dApp [here](https://use.ink/#where-can-i-deploy-ink-contracts). For this tutorial, you could use any available test network. However, in this section we demonstrate how to deploy it on Rococo, Polkadot's parachain community test network.

<!-- tabs:end -->

Fantastic — you've successfully completed the voting dapp tutorial! Here are things you can continue working on to further your learning journey about building ink! dApps with OpenBrush.

* Add events
* Write unit tests for each contract
* Write integration tests
* Write a front-end allowing end-users to interact with the dApp