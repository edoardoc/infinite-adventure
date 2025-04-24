use crate::constants::constants::*;
use crate::error::AdventureError;
use crate::state::Exit;
use crate::state::GameMapAccount;
use crate::state::Location;
use anchor_lang::prelude::*;
use instructions::*;
use oorandom::Rand32;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("ALF2mCoNcyjspCXoc6cdjAV2JcM63kguTqEMPo4YULg1");

#[program]
pub mod infinite_adventure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let new_game_data_account = &mut ctx.accounts.new_game_data_account;
        new_game_data_account.player_location_index = START_LOCATION_INDEX;
        new_game_data_account.player_inventory = Vec::new();

        let new_game_map_account = &mut ctx.accounts.new_game_map_account;
        new_game_map_account.locations = Vec::new();
        new_game_map_account.locations.push(Location {
            description: "You find yourself in a peaceful meadow.".to_string(),
            exits: vec![
                Exit {
                    direction: "north".to_string(),
                    target_index: None,
                },
                Exit {
                    direction: "east".to_string(),
                    target_index: None,
                },
            ],
            items: vec!["common mushroom".to_string()],
            visited: true,
        });

        msg!("A Journey Begins!");
        print_player_status(&ctx.accounts.new_game_data_account, &ctx.accounts.new_game_map_account)?;
        Ok(())
    }

    pub fn move_player(ctx: Context<MovePlayer>, direction: String) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        let game_map_account = &mut ctx.accounts.game_map_account;
        let current_location_index = game_data_account.player_location_index;

        if let Some(current_location) = game_map_account.locations.get_mut(current_location_index as usize) {
            if let Some(exit) = current_location.exits.iter().find(|e| e.direction == direction) {
                if let Some(next_location_index) = exit.target_index {
                    game_data_account.player_location_index = next_location_index;
                } else {
                    let new_location_index = generate_new_location(current_location_index, &direction, game_map_account);
                    if let Some(current_loc_mut) = game_map_account.locations.get_mut(current_location_index as usize) {
                        if let Some(exit_mut) = current_loc_mut.exits.iter_mut().find(|e| e.direction == direction) {
                            exit_mut.target_index = Some(new_location_index as u32);
                            // Sets the target index of the exit
                        }
                    }
                    game_data_account.player_location_index = new_location_index as u32;
                }
            } else {
                return err!(AdventureError::InvalidMove);
            }
        } else {
            return err!(AdventureError::InvalidLocationIndex);
        }
        print_player_status(&ctx.accounts.game_data_account, &ctx.accounts.game_map_account)?;
        Ok(())
    }

    pub fn collect_item(ctx: Context<CollectItem>, item_name: String) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        let game_map_account = &mut ctx.accounts.game_map_account;
        let current_location_index = game_data_account.player_location_index;

        if let Some(location) = game_map_account.locations.get_mut(current_location_index as usize) {
            if let Some(index) = location.items.iter().position(|item| *item == item_name) {
                let collected_item = location.items.remove(index);
                game_data_account.player_inventory.push(collected_item);
                msg!("Collected {}", item_name);
            } else {
                return err!(AdventureError::ItemNotFound);
            }
        } else {
            return err!(AdventureError::InvalidLocationIndex);
        }
        print_player_status(&ctx.accounts.game_data_account, &ctx.accounts.game_map_account)?;
        Ok(())
    }

    pub fn view_location(ctx: Context<ViewLocation>) -> Result<()> {
        let game_data_account = &ctx.accounts.game_data_account;
        let game_map_account = &ctx.accounts.game_map_account;
        let current_location_index = game_data_account.player_location_index;

        if let Some(location) = game_map_account.locations.get(current_location_index as usize) {
            msg!("Description: {}", location.description);
            msg!("Items: {}", location.items.join(", "));
            msg!(
                "Exits: {}",
                location
                    .exits
                    .iter()
                    .map(|e| format!("{}: {:?}", e.direction, e.target_index))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        } else {
            return err!(AdventureError::InvalidLocationIndex);
        }
        print_player_status(&ctx.accounts.game_data_account, &ctx.accounts.game_map_account)?;
        Ok(())
    }
}

fn generate_new_location(
    previous_location_index: u32,
    direction_from_previous: &str,
    game_map_account: &mut Account<GameMapAccount>,
) -> u32 {
    let new_location_index = game_map_account.locations.len() as u32;
    let opposite_direction = match direction_from_previous {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => "",
    };

    let mut exits = Vec::new();
    exits.push(Exit {
        direction: opposite_direction.to_string(),
        target_index: Some(previous_location_index),
    });

    let possible_new_directions = ["north", "south", "east", "west"]
        .iter()
        .filter(|&d| *d != direction_from_previous && *d != opposite_direction)
        .map(|d| d.to_string())
        .collect::<Vec<String>>();

    let mut rng = Rand32::new(Clock::get().unwrap().unix_timestamp as u64); // Seed with current timestamp
    let num_new_exits = (rng.rand_u32() % (std::cmp::min(3, possible_new_directions.len() as u32) + 1)) as usize;
    let shuffled_directions = {
        let mut temp = possible_new_directions;
        // Shuffle up the directions (Fisher-Yates)
        for i in (1..temp.len()).rev() {
            let j = (rng.rand_u32() % (i + 1) as u32) as usize;
            temp.swap(i, j);
        }
        temp
    };

    let mut exits = Vec::new();
    for i in 0..num_new_exits {
        exits.push(Exit {
            direction: shuffled_directions[i].to_string(),
            target_index: None,
        });
    }

    let descriptions = [
        "A dense patch of ferns.",
        "A rocky outcrop with a view.",
        "A quiet clearing.",
        "A whispering forest path.",
    ];
    let description = descriptions[(rng.rand_u32() % descriptions.len() as u32) as usize].to_string();
    let num_items = (rng.rand_u32() % 2) as usize;
    let mut items = Vec::new();
    for _ in 0..num_items {
        let mushroom_type = ["common mushroom", "rare mushroom"][(rng.rand_u32() % 2) as usize].to_string();
        items.push(mushroom_type);
    }

    game_map_account.locations.push(Location {
        description,
        exits,
        items,
        visited: true,
    });

    new_location_index
}
