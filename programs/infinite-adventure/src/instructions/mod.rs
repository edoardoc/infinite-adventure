use crate::constants::constants::*;
use crate::state::GameDataAccount;
use crate::state::GameMapAccount;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
         init,
         seeds = [GAME_LEVEL_SEED],
         bump,
         payer = signer,
         space = 8 + std::mem::size_of::<GameDataAccount>()
     )]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
         init,
         payer = signer,
         space = 8 + std::mem::size_of::<GameMapAccount>() + (4 + 2048), // Adjust space for Vec
         seeds = [GAME_MAP_ACCOUNT_SEED],
         bump,
     )]
    pub new_game_map_account: Account<'info, GameMapAccount>,
}

#[derive(Accounts)]
pub struct MovePlayer<'info> {
    #[account(mut, seeds = [GAME_LEVEL_SEED], bump)]
    pub game_data_account: Account<'info, GameDataAccount>,
    #[account(mut, seeds = [GAME_MAP_ACCOUNT_SEED], bump)]
    pub game_map_account: Account<'info, GameMapAccount>,
    pub authority: Signer<'info>, // Player's wallet
}

#[derive(Accounts)]
pub struct CollectItem<'info> {
    #[account(mut, seeds = [GAME_LEVEL_SEED], bump)]
    pub game_data_account: Account<'info, GameDataAccount>,
    #[account(mut, seeds = [GAME_MAP_ACCOUNT_SEED], bump)]
    pub game_map_account: Account<'info, GameMapAccount>,
    pub authority: Signer<'info>, // Player's wallet
}

#[derive(Accounts)]
pub struct ViewLocation<'info> {
    #[account(seeds = [GAME_LEVEL_SEED], bump)]
    pub game_data_account: Account<'info, GameDataAccount>,
    #[account(seeds = [GAME_MAP_ACCOUNT_SEED], bump)]
    pub game_map_account: Account<'info, GameMapAccount>,
    pub authority: Signer<'info>, // Player's wallet
}

pub fn print_player_status(game_data_account: &Account<GameDataAccount>, game_map_account: &Account<GameMapAccount>) -> Result<()> {
  let current_location_index = game_data_account.player_location_index;
  let player_inventory = &game_data_account.player_inventory;

  if let Some(location) = game_map_account.locations.get(current_location_index as usize) {
      msg!("------------------------------------");
      msg!("You are currently at:");
      msg!("{}", location.description);
      msg!("------------------------------------");

      if !player_inventory.is_empty() {
          msg!("Your inventory contains:");
          for item in player_inventory {
              msg!("- {}", item);
          }
          msg!("------------------------------------");
      } else {
          msg!("Your inventory is empty.");
          msg!("------------------------------------");
      }

      msg!("Available exits:");
      for exit in &location.exits {
          msg!("- {}", exit.direction);
      }
      msg!("------------------------------------");
  } else {
      msg!("Error: Player location index is invalid!");
  }

  Ok(())
}