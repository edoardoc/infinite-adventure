use anchor_lang::prelude::*;

#[constant]
pub mod constants {
    pub const GAME_LEVEL_SEED: &[u8] = b"level1";
    pub const GAME_MAP_SEED: &[u8] = b"game_map";
    pub const START_LOCATION_ID: &str = "start";
}
