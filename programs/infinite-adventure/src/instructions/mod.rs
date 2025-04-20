use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use crate::state::GameDataAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
         init,
         seeds = [b"level1"],
         bump,
         payer = signer,
         space = 8 + 1
     )]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveLeft<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveRight<'info> {
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