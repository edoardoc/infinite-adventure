use anchor_lang::prelude::*;
use instructions::*; // Import all instructions

pub mod state;        // Declare the state module
pub mod instructions; // Declare the instructions module
pub mod error;        // Declare the error module
pub mod constants;    // Declare the constants module

declare_id!("33BuEyGHLbL7up1w6NK8NHRTxYiQmcRANCKp5tfKuv1m");

#[program]
pub mod infinite_adventure {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
      ctx.accounts.new_game_data_account.player_position = 0;
      msg!("A Journey Begins!");
      msg!("o.......");
      Ok(())
  }

  pub fn move_left(ctx: Context<MoveLeft>) -> Result<()> {
      let game_data_account = &mut ctx.accounts.game_data_account;
      if game_data_account.player_position == 0 {
          msg!("You are back at the start.");
      } else {
          game_data_account.player_position -= 1;
          print_player(game_data_account.player_position);
      }
      Ok(())
  }

  pub fn move_right(ctx: Context<MoveRight>) -> Result<()> {
      let game_data_account = &mut ctx.accounts.game_data_account;
      if game_data_account.player_position == 3 {
          msg!("You have reached the end! Super!");
      } else {
          game_data_account.player_position = game_data_account.player_position + 1;
          print_player(game_data_account.player_position);
      }
      Ok(())
  }

}
