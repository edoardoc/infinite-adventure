use anchor_lang::prelude::*;

#[constant]
pub mod constants {
  pub const GAME_LEVEL_SEED: &[u8] = b"level1";
  pub const GAME_MAP_ACCOUNT_SEED: &[u8] = b"game_map";
  pub const START_LOCATION_INDEX: u32 = 0; // Index of the starting location in the Vec
}
