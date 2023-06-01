# Link dependencies

Just like linking the dependencies for the Staking contract to inherit your dApps trait module, you'll need to do the same for the Voting contract.

### Include the voting implementation as a project feature

1. Add the Staking feature to end of the `src/Cargo.toml` file:
    ```toml
    // -- snip --
    # adds staking features to dapp library
    voting = []
    ```

1. In `contracts/voting/Cargo.toml`, add `src` as dependency called `dapp` with the staking feature included:

    ```toml
    [dependencies]
    // -- snip --
    dapp = { path = "../../src", default-features = false, features = ["voting"] }
    // -- snip --
    ```

1. Include it in the `std` features too:
    
    ```toml
    // -- snip --
    std = [
    // -- snip --
    "dapp/std", # add this line
    ]
    ```
    
### Check your contract compiles 

In the `contracts/voting` directory, run `cargo check` to check your contract compiles:

```bash
cargo contract check
```

Congratulations! If you've completed the previous steps correctly, you should see this message in the terminal:

```sh
Your contract's code was built successfully.
```

> 💡 If you run into errors, make sure to go back and check that you contract's `Cargo.toml` files include the `voting` feature and that you've correctly updated the `src/impls/mod.rs` file.

You're now ready to deploy your dApp and interact with it. 🎉