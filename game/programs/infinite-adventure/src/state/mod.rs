use anchor_lang::prelude::*;

#[account]
pub struct GameDataAccount {
    pub player_position: u8,
}
