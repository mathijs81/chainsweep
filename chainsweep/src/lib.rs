#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
#![cfg_attr(not(any(feature = "export-abi", test)), no_std)]

mod sweeper;
mod field;

extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;


use alloc::string::String;

use alloy_primitives::Address;
use stylus_sdk::{block, evm, msg, prelude::*};
use sweeper::{GameAlreadyStarted, GameError, GameStarted, Game};

sol_storage! {
    #[entrypoint]
    pub struct SweeperGame {
        mapping(address => Game) games;
    }
}

#[external]
impl SweeperGame {
    pub fn new_game(&mut self) -> Result<String, GameError> {
        let caller = msg::sender();
        let mut game = self.games.setter(caller);
        if game.is_started() && !game.is_ended() {
            return Err(GameError::GameAlreadyStarted(GameAlreadyStarted {}));
        }
        game.init();
        evm::log(GameStarted { player: caller });

        Ok(game.print())
    }

    pub fn view_for(&self, address: Address) -> Result<String, GameError> {
        Ok(self.games.get(address).print())
    }

    pub fn view_completed(&self, address: Address, seed: u64) -> Result<String, GameError> {
        let game = self.games.get(address);
        Ok(game.print_filled_in(seed))
    }

    pub fn make_guess(&mut self, x: u8, y: u8) -> Result<u8, GameError> {
        let caller = msg::sender();
        let mut game = self.games.setter(caller);
        game.make_guess(x, y, block::timestamp() ^ block::gas_limit())
    }
}

