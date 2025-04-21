use crate::constants::constants::*;
use crate::state::Location;
use anchor_lang::prelude::*;
use instructions::*; // Import all instructions
use std::collections::HashMap;

pub mod constants;
pub mod error; // Declare the error module
pub mod instructions; // Declare the instructions module
pub mod state; // Declare the state module // Declare the constants module

declare_id!("33BuEyGHLbL7up1w6NK8NHRTxYiQmcRANCKp5tfKuv1m");

#[program]
pub mod infinite_adventure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let new_game_data_account = &mut ctx.accounts.new_game_data_account;
        new_game_data_account.player_location = START_LOCATION_ID.to_string();
        new_game_data_account.player_inventory = Vec::new();

        let new_game_map_account = &mut ctx.accounts.new_game_map_account;
        let exits = vec![
            ("north".to_string(), Some("forest".to_string())),
            ("south".to_string(), Some("cave".to_string())),
            ("east".to_string(), None),
            ("west".to_string(), None),
        ];
        let loc = Location {
            description: "You are in a dense forest.".to_string(),
            exits: vec![
                ("south".to_string(), Some(START_LOCATION_ID.to_string())),
                ("east".to_string(), None),
                ("west".to_string(), None),
            ],
            items: vec!["rare flower".to_string()],
            visited: false,
        };

        // new_game_map_account
        //     .locations
        //     .insert(START_LOCATION_ID.to_string(), loc);

        msg!("A Journey Begins!");
        Ok(())
    }

    // pub fn move_west(ctx: Context<MoveWest>) -> Result<()> {
    //     let game_data_account = &mut ctx.accounts.game_data_account;
    //     if game_data_account.player_position == 0 {
    //         msg!("You are back at the start.");
    //     } else {
    //         game_data_account.player_position -= 1;
    //         print_player(game_data_account.player_position);
    //     }
    //     Ok(())
    // }

    // pub fn move_east(ctx: Context<MoveEast>) -> Result<()> {
    //     let game_data_account = &mut ctx.accounts.game_data_account;
    //     if game_data_account.player_position == 3 {
    //         msg!("You have reached the end! Super!");
    //     } else {
    //         game_data_account.player_position = game_data_account.player_position + 1;
    //         print_player(game_data_account.player_position);
    //     }
    //     Ok(())
    // }
}
