# Scaling Ethereum 2024

This is my submission for [Scaling Ethereum 2024](https://ethglobal.com/events/scaling2024).

The repository was bootstrapped from the [stylus-hello-world](https://github.com/OffchainLabs/stylus-hello-world/tree/45a9fbdca70924d9ae39e49ec2661dc6eb5ac610) project.

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

[Local dev node with stylus support setup](https://docs.arbitrum.io/stylus/how-tos/local-stylus-dev-node) (note, I had to replace `docker-compose` with `docker compose` in the init script for it to work)



## License

This project is fully open source, under the Apache-2.0 license.
