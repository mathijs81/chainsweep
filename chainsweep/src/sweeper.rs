use alloy_primitives::{Uint, U256};
use alloy_sol_types::sol;
use stylus_sdk::{evm, msg, prelude::*, storage::{StorageArray, StorageU256, StorageU8}};

// Field size is fixed
// This will fit in two u256
const WIDTH: u8 = 11;
const HEIGHT: u8 = 11;

const BUG_CHANCE_100: u8 = 35;

sol! {
    event GameStarted(address indexed player);
    event FieldOpened(address indexed player, uint8 x, uint8 y, uint8 value);
    event GameOver(address indexed player, bool won);

    error GameAlreadyOver();
    error GameAlreadyStarted();
    error FieldAlreadyOpened();
}

#[derive(SolidityError)]
pub enum GameError {
    GameAlreadyOver(GameAlreadyOver),
    GameAlreadyStarted(GameAlreadyStarted),
    FieldAlreadyOpened(FieldAlreadyOpened),
}

// enum not supported in stylus yet? https://github.com/OffchainLabs/stylus-sdk-rs/issues/54
// enum GameState {
//     Playing,
//     Lost,
//     Won
// }

// We use a u8 to represent the game state
type GameState = u8;
const STATE_NOT_STARTED: GameState = 0;
const STATE_PLAYING: GameState = 1;
const STATE_LOST: GameState = 2;
const STATE_WON: GameState = 3;


#[solidity_storage]
pub struct Game {
    board_encoded: StorageArray<StorageU256, 2>,
    state: StorageU8
}

// We encode each field in 4 bits.
// 0-8 = opened, number of bugs around
// 9 = bug
// 10 = unopened

const BUG: u8 = 9;
const UNOPENED: u8 = 10;

fn next_rand(seed: u64) -> u64 {
    let mut x = seed;
    x = ((x + 1337) * 16807) % 0x7FFFFFFF;
    x
}

impl Game {
    fn set_field(&mut self, x: u8, y: u8, value: u8) {
        let field = (x + y * WIDTH) as usize;
        let field_bit_offset = field * 4;
        // let field_uint_offset = field_bit_offset / 64;
        // let field_shift = field_bit_offset % 64;
        // let mask = 0xFu64 << field_shift;
        // let value = (value as u64) << field_shift;
        let mut current256: [u8; 32] = self.board_encoded.get(field_bit_offset / 256).unwrap().to_le_bytes();
        let byte_index = (field_bit_offset % 256) / 8;
        let byte_shift = (field_bit_offset % 256) % 8;
        let mask = 0xFu8 << byte_shift;
        current256[byte_index] = (current256[byte_index] & !mask) | (value << byte_shift);
        self.board_encoded.setter(field_bit_offset / 256).unwrap().set(U256::from_le_bytes(current256));
    }

    fn get_field(&self, x: u8, y: u8) -> u8 {
        let field = (x + y * WIDTH) as usize;
        let field_bit_offset = field * 4;
        let current256: [u8; 32] = self.board_encoded.get(field_bit_offset / 256).unwrap().to_le_bytes();
        let byte_index = (field_bit_offset % 256) / 8;
        let byte_shift = (field_bit_offset % 256) % 8;
        let mask = 0xFu8 << byte_shift;
        (current256[byte_index] & mask) >> byte_shift
    }

    pub fn init(&mut self, rand_seed: u64) {
        self.state.set(Uint::from(STATE_PLAYING));
        let mut r = rand_seed;
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.set_field(i, j, UNOPENED);
     
                let p = (r % 100) as u8;
                if p < BUG_CHANCE_100 {
                    self.set_field(i, j, BUG);
                }
                r = next_rand(r);
            }
        }
    }

    pub fn print(&self) -> String {
        if self.state.get().byte(0) == STATE_NOT_STARTED {
            return "Game not started".to_string();
        }
        let mut res = String::new();
        res.push_str("----\n");
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let fieldval = self.get_field(i, j);
                if fieldval == BUG {
                    res.push_str("X");
                } else if fieldval == UNOPENED {
                    res.push_str(" ");
                } else {
                    res.push_str(&fieldval.to_string());
                }
            }
            res.push_str("\n");
        }
        res
    }

    pub fn make_guess(&mut self, x: u8, y: u8) -> Result<u8, GameError> {
        if self.state.get().byte(0) != STATE_PLAYING {
            return Err(GameError::GameAlreadyOver(GameAlreadyOver {}));
        }

        let field = self.get_field(x, y);
        if field != BUG && field != UNOPENED {
            return Err(GameError::FieldAlreadyOpened(FieldAlreadyOpened {}));
        }
        evm::log(FieldOpened { player: msg::sender(), x, y, value: field });
        if field == BUG {
            evm::log(GameOver { player: msg::sender(), won: false });
            self.state.set(Uint::from(STATE_LOST));
            return Ok(BUG);
        }

            let mut count = 0;
            for i in -1..1 {
                for j in -1..1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let x = x as i8 + i;
                    let y = y as i8 + j;
                    if x < 0 || x as u8 >= WIDTH || y < 0 || y as u8 >= HEIGHT {
                        continue;
                    }
                    if self.get_field(x as u8, y as u8) == BUG {
                        count += 1;
                    }
                }
            }
            self.set_field(x, y, count);
            return Ok(count);
    }

    pub fn is_started(&self) -> bool {
        self.state.get().byte(0) != STATE_NOT_STARTED
    }

    pub fn is_ended(&self) -> bool {
        self.state.get().byte(0) == STATE_LOST || self.state.get().byte(0) == STATE_WON
    }
}