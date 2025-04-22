use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Exit {
    pub direction: String,
    pub target_index: Option<u32>, // Index of the linked location, None if unexplored
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Location {
    pub description: String,
    pub exits: Vec<Exit>,
    pub items: Vec<String>,
    pub visited: bool,
}
#[account]
pub struct GameMapAccount {
  pub locations: Vec<Location>,
}

#[account]
pub struct GameDataAccount {
  pub player_location_index: u32,
  pub player_inventory: Vec<String>,
}
