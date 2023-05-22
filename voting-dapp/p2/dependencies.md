# Link dependencies

Once you update your project to include the Voting impl as a feature, you'll be able to include the storage of the Voting trait implementation to the ink! contract.

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

In the `contracts/staking` directory, run `cargo check` to check your contract compiles:

```bash
cargo contract check
```
