use std::u8;

use crate::card::{Card, FACE};
use crate::settings::*;

#[derive(Debug, Clone, Copy)]
pub enum HAND_STATE
{
    NONE,
    STAND,
    DOUBLE_DOWN,
    BUST,
    DOUBLE_DOWN_BUST,
    BLACKJACK,
    DOUBLE_DOWN_BJ
}

#[derive(Debug)]
pub struct Hand
{
    pub cards: Vec<Card>,
    pub total: u8,
    pub state: HAND_STATE,
    pub bet: usize
}

impl Hand
{
    pub fn new(bet: usize) -> Self
    {
        Hand
        {
            cards: vec![],
            total: 0,
            state: HAND_STATE::NONE,
            bet
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

    /// @brief Method that determines the index of the first splittable card in a hand
    /// @brief Returns 0 if the hand isn't splittable
    pub fn splittable(&self) -> usize
    {
        for (i, &card) in self.cards.iter().enumerate()
        {
            for (j, other) in self.cards[i + 1..self.cards.len()].iter().enumerate()
            {
                if(card.face.index() == other.face.index())
                {
                    return j + i + 1;
                }
            }
        }

        0
    }

    pub fn split(&mut self, index: usize, bet: usize) -> Result<(Hand, Hand), &'static str>
    {
        let mut hands_data: ((u8, HAND_STATE), (u8, HAND_STATE)) = ((0, HAND_STATE::NONE), (0, HAND_STATE::NONE));
        
        let first_hand_cards = &self.cards[0..index];
        for card in first_hand_cards
        {
            hands_data.0.0 += card.value();
            if(hands_data.0.0 > 21)
            {
                if let FACE::ACE = card.face
                {
                    hands_data.0.0 -= 10;
                }
                else 
                {
                    hands_data.0.1 = HAND_STATE::BUST;
                }
            }
        }

        let second_hand_cards = &self.cards[index..self.cards.len()];
        for card in second_hand_cards
        {
            hands_data.1.0 += card.value();
            if(hands_data.1.0 > 21)
            {
                if let FACE::ACE = card.face
                {
                    hands_data.1.0 -= 10;
                }
                else 
                {
                    hands_data.1.1 = HAND_STATE::BUST;
                }
            }
        }

        Ok((
            Hand
            {
                cards: first_hand_cards.to_vec(),
                total: hands_data.0.0,
                state: hands_data.0.1,
                bet: self.bet
            },
            Hand
            {
                cards: second_hand_cards.to_vec(),
                total: hands_data.1.0,
                state: hands_data.1.1,
                bet
            }
        ))
    }
}