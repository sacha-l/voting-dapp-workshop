## Deploy your contract using the Contracts UI

Congratulations! If you've made it this far, you are ready to deploy your contract on-chain and interact with it.

You could use the Cargo Contract CLI as described in the official ink! tutorials but here, you'll learn how to deploy it using the Contracts UI. You have two options, either deploy it to a local node if you have one set-up or deploy it on Rococo.

1. Build your contract by typing the following command inside the `contracts/staking` directory:
    
    ```bash
    cargo contract build
    ```
1. Go to the Contracts UI and select the node you want to connect to:

    <img src="../assets/contracts-ui-1.png" width="400">

1. Once you're connected, select "Upload a new contract" and upload the `my_psp22.contract` file from the `/voting-dapp/target/ink/my_psp22` directory. Once you upload it, should see something like this:

    <img src="../assets/contracts-ui-2.png" width="400">

1. Scroll to the bottom of the page and click "Next". This will bring you to another page as shown below. Also click "Next".

    <img src="../assets/contracts-ui-3.png" width="400">

1. Interact with the contract using the UI and explore the different methods provided by the PSP22 standard as well as our new Staking functions.

Great — you have the first version of your Staking contract deployed! Continue to the next part of this tutorial to:
* Create a Voting contract to handle the voting logic
* Add events
* Write tests