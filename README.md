# ChainSweep on-chain game

![Screenshot](/screenshot.png?raw=true)

*[Live demo on Arbitrum Stylus Testnet](https://chainsweep.vercel.app/)*

This game is an on-chain implementation of a game similar to the well-known "minesweeper" game.

In a field of blocks you have to validate blocks, avoiding the ones that contain bugs. When you click a buggy block, you lose. When you validated all blocks without bugs, you win.

The game is run fully on-chain (Arbitrum Stylus Testnet). Every move you make is a separate transaction.

The important piece of information in the game is where exactly the bugs are (the fields you shouldn't click). If the smart contract chooses these spots and stores them on chain, then it would be trivial to win the game by looking with a block explorer or other tools where the bugs are.
To avoid this, the contract only stores which fields have been revealed so far. When the player makes a new move, the smart contract re-generates a potential assignment of bugs on the field that's consistent with the moves made so far. This makes that the game remains unpredictable.

To make the calculation of possible assignments efficient, it uses Arbitrum Stylus so that Rust code can be run as WebAssembly and the simulation process doesn't take too much gas.

This is a submission for [Scaling Ethereum 2024](https://ethglobal.com/events/scaling2024).

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

Install solc (necessary to generate json ABI):

```bash
wget https://github.com/ethereum/solidity/releases/download/v0.8.25/solc-static-linux -O ~/.local/bin/solc
chmod a+x ~/.local/bin/solc
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

