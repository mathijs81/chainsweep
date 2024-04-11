# Scaling Ethereum 2024

This is my submission for [Scaling Ethereum 2024](https://ethglobal.com/events/scaling2024).

# What is it?

In progress -- current ly just trying things out.

## Setup

Install [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo

```bash
RUSTFLAGS="-C link-args=-rdynamic" cargo install --force cargo-stylus
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

### Testnet Information

All testnet information, including faucets and RPC endpoints can be found [here](https://docs.arbitrum.io/stylus/reference/testnet-information).

### Local development

#### 1. Setup

[Local dev node with stylus support setup](https://docs.arbitrum.io/stylus/how-tos/local-stylus-dev-node) (note, I had to replace `docker-compose` with `docker compose` in the init script for it to work)

Install [foundry](https://book.getfoundry.sh/getting-started/installation):

```
cargo install --git https://github.com/foundry-rs/foundry --profile local --locked forge cast chisel anvil
```

#### 2. Local dev

You start the local node with `test-node.bash --blockscout` in the directory where you installed the local stylus dev node. The local explorer is then accessible at <http://localhost:4000/>.

## License

This project is fully open source, under the Apache-2.0 license.

## Sources

The repository was bootstrapped from the [stylus-hello-world](https://github.com/OffchainLabs/stylus-hello-world/tree/45a9fbdca70924d9ae39e49ec2661dc6eb5ac610) project.
I looked at code from the [stylus-workshop-rust-solidity](https://github.com/OffchainLabs/stylus-workshop-rust-solidity) project as well.

