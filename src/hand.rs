use std::u8;

use crate::card::{Card};
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
    pub fn new(cards: Vec<Card>, total: u8, state: HAND_STATE, bet: usize) -> Self
    {
        Hand
        {
            cards,
            total,
            state,
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

    pub fn split(&mut self, index: usize, bet: usize) -> (Hand, Hand)
    {    
        let first_hand_cards = &self.cards[0..index];
        let mut first_hand_total: u8 = 0;
        for card in first_hand_cards
        {
            first_hand_total += card.value();
        }

        let mut first_hand: Hand = Hand::new(first_hand_cards.to_vec(), first_hand_total, self.state, self.bet);
        first_hand.calculate_state();

        let second_hand_cards = &self.cards[index..self.cards.len()];
        let mut second_hand_total: u8 = 0;
        for card in second_hand_cards
        {
            second_hand_total += card.value();
        }

        let mut second_hand: Hand = Hand::new(second_hand_cards.to_vec(), second_hand_total, HAND_STATE::NONE, bet);
        second_hand.calculate_state();

        (
            first_hand,
            second_hand
        )
    }

    pub fn calculate_state(&mut self)
    {
        if(self.total > 21)
        {
            if let HAND_STATE::DOUBLE_DOWN = self.state
            {
                self.state = HAND_STATE::DOUBLE_DOWN_BUST;
            }
            else 
            {
                self.state = HAND_STATE::BUST;
            }
        }
        else if(self.total == 21 && self.cards.len() == 1)
        {
            if let HAND_STATE::DOUBLE_DOWN = self.state
            {
                self.state = HAND_STATE::DOUBLE_DOWN_BJ;
            }
            else
            {
                self.state = HAND_STATE::BLACKJACK;
            }
        }
    }
}