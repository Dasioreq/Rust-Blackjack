#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crate::game::Game;
use crate::settings::Settings;

mod hand;
mod deck;
mod card;
mod settings;
mod game;

fn main()
{
    let settings = Settings::load_from_files("./settings/settings.json");
    
    let mut game: Game = Game::new(settings);
    let mut quit: bool = false;
    while(!quit)
    {
        game.draw();
        quit = game.get_player_input();
    }
}