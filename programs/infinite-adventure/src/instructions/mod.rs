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
        seeds = [GAME_MAP_ACCOUNT_SEED],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<GameMapAccount>() + (4 + 2048), // Adjust space for Vec
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

#[derive(Accounts)]
pub struct MoveWest<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveEast<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

pub fn print_player(player_position: u8) {
    match player_position {
        0 => msg!("o......."),
        1 => msg!("..o....."),
        2 => msg!("....o..."),
        3 => msg!("......o."),
        _ => msg!("Invalid position!"),
    }
}
