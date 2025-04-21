use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Location {
    pub description: String,
    pub exits: Vec<(String, Option<String>)>,
    pub items: Vec<String>,
    pub visited: bool,
}

#[account]
pub struct GameMapAccount {
  pub locations: Vec<(String, Location)>,
}

#[account]
pub struct GameDataAccount {
  pub player_location: String,
  pub player_inventory: Vec<String>,
}
