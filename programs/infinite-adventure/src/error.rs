use anchor_lang::prelude::*;

#[error_code]
pub enum AdventureError {
    #[msg("Invalid move in the specified direction.")]
    InvalidMove,
    #[msg("The requested item was not found at this location.")]
    ItemNotFound,
    #[msg("Invalid location ID.")]
    InvalidLocation,
}
