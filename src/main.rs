    #![allow(unused_parens)]

use crate::deck::*;
use crate::settings::Settings;

mod deck;
mod card;
mod settings;

fn main()
{
    let settings = Settings::load_from_files("./settings/settings.json");
    let mut deck = Deck::new(settings.decks);
    deck.shuffle();

    for (i, card) in deck.cards.iter().enumerate()
    {
        card.draw(&settings, (1 + i as isize * settings.card_offset) as usize, 1);
    }
}