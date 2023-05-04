# Setup your workspace

Before you start, please check that your Rust compiler is set to nightly (you should see a similar output to this):

```bash
$ rustc --version
rustc 1.68.0-nightly (d6f99e535 2023-01-02)
```

And finally, make sure your `cargo contract` version is updated to 2.0 or higher (you should see a similar output to this):

```bash
$ cargo contract --version
cargo-contract-contract 2.1.0-unknown-aarch64-apple-darwin
```

To make it easier for you to set up your workspace, you can use the `scaffolf-openbrush` Github repo which contains the file structure we’ll be using for building out our dApp. 

In the directory you've dedicated to completing this tutorial, clone this repository:

```bash
git clone https://github.com/sacha-l/scaffold-openbrush.git
```

Rename the newly created directory to `voting-dapp`:

```bash
mv scaffold-openbrush voting-dapp
```

Go to the contracts directory and open it in your preferred code editor, for example using this command from your terminal:

```bash
cd voting-dapp && code .
```

You should now have the scaffold file structure which you’ll be using to build out your dApp.

<!-- slide:break -->

If you've correctly completed this section, your file structure will look like this:

<img src="../assets/file-structure.png" width="200"> 


Here’s a quick overview of this file structure:

- `contracts`: will contain the contracts with their generic types.
- `src/impls`: will contain the implementations for each contract.
- `src/libs`: will contain common functionality shared by our contracts, such as for handling errors or doing safe math.
- `src/traits`: will contain the trait definitions for each contract.

