use std::ops::Deref;

use rand::rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::card::*;

#[derive(Debug)]
pub struct Deck
{
    pub cards: Vec<Card>
}

impl Deck
{
    pub fn new(num_of_decks: u8) -> Self
    {
        let mut result: Deck = Deck{cards: vec![]};
        for _ in [0..=num_of_decks]
        {
            for suit in SUIT::iter()
            {
                for face in FACE::iter()
                {
                    result.cards.push(
                        Card::new
                        (
                            face,
                            suit
                        ));
                }
            }
        }
        result
    }

    pub fn shuffle(&mut self)
    {
        self.cards.shuffle(&mut rand::rng());
    }
}