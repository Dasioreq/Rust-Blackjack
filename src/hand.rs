use crate::card::{Card};
use crate::settings::*;

#[derive(Debug)]
pub enum HAND_STATE
{
    NONE,
    STAND,
    DOUBLE_DOWN(usize),
    BUST
}

#[derive(Debug)]
pub struct Hand
{
    pub cards: Vec<Card>,
    pub total: u8,
    pub state: HAND_STATE
}

impl Hand
{
    pub fn new() -> Self
    {
        Hand
        {
            cards: vec![],
            total: 0,
            state: HAND_STATE::NONE
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

    pub fn splittable(&self) -> bool
    {
        for (i, &card) in self.cards.iter().enumerate()
        {
            for other in self.cards[i + 1..self.cards.len()].iter()
            {
                if(card.face == other.face)
                {
                    return true
                }
            }
        }

        false
    }
}