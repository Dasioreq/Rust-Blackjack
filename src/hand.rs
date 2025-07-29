use crate::card::{Card, FACE, SUIT};
use crate::settings::*;

#[derive(Debug)]
pub struct Hand
{
    pub cards: Vec<Card>,
    pub total: u8
}

impl Hand
{
    pub fn new() -> Self
    {
        Hand
        {
            cards: vec![],
            total: 0
        }
    }

    pub fn draw(&self, settings: &Settings, x: usize, y: usize, hide_second: bool)
    {
        for (i, &card) in self.cards.iter().enumerate()
        {
            if(hide_second && i == 1)
            {
                card.draw(settings, x + settings.card_offset as usize * i, y, false);
            }
            else 
            {
                card.draw(settings, x + settings.card_offset as usize * i, y, true);
            }
        }
    }
}