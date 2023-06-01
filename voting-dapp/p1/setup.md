# Setup your workspace

Before you start, please check that your Rust compiler is set to Nightly.
You should see a similar output to this:

```bash
$ rustc --version
rustc 1.68.0-nightly (d6f99e535 2023-01-02)
```

> 💡 Curious to read more about why we're using the Rust Nightly compiler? Check out the tab on the right 😉. In general you can expect any extra information you may want to know about to be over there as you complete the tutorial steps.  Going forward, expect to find things like code snips and other explanations. 

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
<!-- tabs:start -->

#### **💡Why we're using the Rust Nightly compiler**

OpenBrush currently relies on the Rust nightly compiler because it uses a feature from the unstable Rust book called [**specialization**](https://doc.rust-lang.org/beta/unstable-book/language-features/min-specialization.html) (or more accurately, its currently released variant called `min_specialization`). 
This feature is an extension of Rust's trait system which allows developers to inherit and customize the default implementations of the OpenBrush contracts library.

Specifically, you'll be seeing this line of code which specifies the use of this feature in the contract code:

```rust
#![feature(min_specialization)]
```

<!-- tabs:end -->

If you've correctly completed this section, your file structure will look like this:

<img src="../assets/file-structure.png" width="200"> 

- `contracts`: will contain the contracts with their generic types.
- `src/impls`: will contain the implementations for each contract.
- `src/libs`: will contain common functionality shared by our contracts, such as for handling errors or doing safe math.
- `src/traits`: will contain the trait definitions for each contract.
