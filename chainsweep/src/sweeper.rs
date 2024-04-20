use alloy_primitives::{Uint, U256};
use alloy_sol_types::sol;
use stylus_sdk::{
    console, evm, msg,
    prelude::*,
    storage::{StorageU256, StorageU8},
};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::field::{is_open, GameData, BUG, UNOPENED, UNOPENED_BUGFREE};

// Field size is fixed
// This will fit in one u256 (4 bits * 8 * 8 = 256 bits)
// const WIDTH: u8 = 8;
// const HEIGHT: u8 = 8;

// The current setup with a metamask confirmation for every field
// is not very user friendly, so we'll keep the field size small
const WIDTH: u8 = 5;
const HEIGHT: u8 = 5;

// Chance of every field being a bug, as a percentage
const BUG_CHANCE_100: u8 = 20;

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
    board_encoded: StorageU256,
    state: StorageU8,
}
/*
TODO:
  ✅ winning condition
  ✅ auto-open empty fields
  ✅ randomize possible field on every guess
  ✅ make sure first guess is not a bug

  Nice to have:
  Allow multi-open
  store sequence of moves
  award NFT on game win
*/
impl Game {
    fn set_field(&mut self, x: u8, y: u8, value: u8) {
        let field = (x + y * WIDTH) as usize;
        let field_bit_offset = field * 4;

        let mut current256: [u8; 32] = self.board_encoded.get().to_le_bytes();
        let byte_index = field_bit_offset / 8;
        let bit_shift = field_bit_offset % 8;
        let mask = 0xFu8 << bit_shift;
        current256[byte_index] = (current256[byte_index] & !mask) | (value << bit_shift);
        self.board_encoded.set(U256::from_le_bytes(current256));
    }

    fn get_field(&self) -> GameData {
        let current256: [u8; 32] = self.board_encoded.get().to_le_bytes();
        let mut fields = Vec::new();
        for index in 0..WIDTH * HEIGHT {
            let mut field_byte = current256[(index / 2) as usize];
            if index % 2 == 1 {
                field_byte >>= 4;
            } else {
                field_byte &= 0xF;
            }
            fields.push(field_byte);
        }
        GameData::new(WIDTH, HEIGHT, fields)
    }

    pub fn init(&mut self) {
        self.state.set(Uint::from(STATE_PLAYING));
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.set_field(i, j, UNOPENED);
            }
        }
    }

    fn print_field(field_data: GameData, state: u8) -> String {
        let mut res = String::new();
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let fieldval = field_data.get(i, j).data;
                if fieldval == BUG {
                    res.push_str("X");
                } else if fieldval == UNOPENED || fieldval == UNOPENED_BUGFREE{
                    res.push_str(" ");
                } else {
                    res.push_str(&fieldval.to_string());
                }
            }
            res.push_str("\n");
        }
        match state {
            STATE_PLAYING => res.push_str("Playing\n"),
            STATE_LOST => res.push_str("Lost\n"),
            STATE_WON => res.push_str("Won\n"),
            _ => res.push_str("Unknown state"),
        }
        res
    }
    
    pub fn print(&self) -> String {
        if self.state.get().byte(0) == STATE_NOT_STARTED {
            return "Game not started".to_string();
        }
        let field_data = self.get_field();
        Self::print_field(field_data, self.state.get().byte(0))
    }

    pub fn print_filled_in(&self, rand_seed: u64) -> String {
        let filled_in =  self.get_field().fill_in(rand_seed, BUG_CHANCE_100);
        Self::print_field(filled_in, self.state.get().byte(0))
    }

    pub fn make_guess(&mut self, x: u8, y: u8, rand_seed: u64) -> Result<u8, GameError> {
        if self.state.get().byte(0) != STATE_PLAYING {
            return Err(GameError::GameAlreadyOver(GameAlreadyOver {}));
        }

        let mut field_data = self.get_field();
        let field = field_data.get(x, y).data;
        if is_open(field) {
            return Err(GameError::FieldAlreadyOpened(FieldAlreadyOpened {}));
        }
        // If this is the very first guess, make sure it's not a bug
        if field_data.num_open == 0 {
            field_data.set_data(x, y, UNOPENED_BUGFREE);
        }
        // fill in the field with a possible solution
        let mut filled_in = field_data.fill_in(rand_seed, BUG_CHANCE_100);
        console!("filled in: \n{}", filled_in.to_string());

        if filled_in.get(x, y).data == BUG {
            evm::log(FieldOpened {
                player: msg::sender(),
                x,
                y,
                value: BUG,
            });
            self.set_field(x, y, BUG);
            evm::log(GameOver {
                player: msg::sender(),
                won: false,
            });
            self.state.set(Uint::from(STATE_LOST));
            return Ok(BUG);
        }

        let count = self.do_open(x, y, &mut filled_in);
        // Check if won
        let mut won = true;
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let field = filled_in.get(i, j).data;
                if field == UNOPENED_BUGFREE {
                    won = false;
                    break;
                }
            }
        }
        if won {
            evm::log(GameOver {
                player: msg::sender(),
                won: true,
            });
            self.state.set(Uint::from(STATE_WON));
        }

        console!("current field: {}", self.print());
        Ok(count)
    }

    fn do_open(&mut self, x: u8, y: u8, field_data: &mut GameData) -> u8 {
        let count = field_data.get(x, y).adjacent_bugs;
        evm::log(FieldOpened {
            player: msg::sender(),
            x,
            y,
            value: count,
        });
        self.set_field(x, y, count);
        field_data.set_data(x, y, count);

        if count == 0 {
            self.open_adjacent(x, y, field_data);
        }
        count
    }

    fn open_adjacent(&mut self, x: u8, y: u8, field_data: &mut GameData) {
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = x as i8 + i;
                let y = y as i8 + j;
                if x < 0 || x as u8 >= WIDTH || y < 0 || y as u8 >= HEIGHT {
                    continue;
                }
                if !is_open(field_data.get(x as u8, y as u8).data) {
                    self.do_open(x as u8, y as u8, field_data);
                }
            }
        }
    }

    pub fn is_started(&self) -> bool {
        self.state.get().byte(0) != STATE_NOT_STARTED
    }

    pub fn is_ended(&self) -> bool {
        self.state.get().byte(0) == STATE_LOST || self.state.get().byte(0) == STATE_WON
    }
}