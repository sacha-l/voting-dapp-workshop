## Deploy your contracts using the Contracts UI

Congratulations! If you've made it this far, you are ready to deploy your contract on-chain and interact with it.

You could use the Cargo Contract CLI as described in the official ink! tutorials but here, you'll learn how to deploy it using the Contracts UI. You have two options, either deploy it to a local node if you have one set-up or deploy it on Rococo.

1. Build your contract by typing the following command inside the `contracts/staking` directory:
    
    ```bash
    cargo +nightly-2023-02-07 contract build
    ```
1. Go to the Contracts UI and select the node you want to connect to:

    <img src="../assets/contracts-ui-1.png" width="500">

1. Once you're connected, select "Upload a new contract" and upload the `my_psp22.contract` file from the `/voting-dapp/target/ink/my_psp22` directory. Once you upload it, should see something like this:

    <img src="../assets/contracts-ui-2.png" width="500">

1. Scroll to the bottom of the page and click "Next". This will bring you to another page as shown below. Also click "Next".

    <img src="../assets/contracts-ui-3.png" width="500">

1. Interact with the contract using the UI and explore the different methods provided by the PSP22 standard as well as our new Staking and Voting functions.

Great — you have the first version of your Voting dapp deployed! Here are things you can continue working on to further your learning journey about building ink! dApps with OpenBrush.

* Add events and custom errors
* Write unit tests for each contract
* Write integration tests