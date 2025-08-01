    #![allow(unused_parens)]

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
    
    let mut game: Game = Game::new(&settings);

    while(true)
    {
        game.draw(&settings);
        game.get_player_input();
    }
}