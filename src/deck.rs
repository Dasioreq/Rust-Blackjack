use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::{card::*, hand::{Hand, HAND_STATE}};

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
        for _ in 0..num_of_decks
        {
            for suit in SUIT::iter()
            {
                for face in FACE::iter()
                {
                    result.cards.push(
                        Card::new
                        (
                            face,
                            suit,
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

    pub fn deal(&mut self, hand: &mut Hand, num: usize)
    {
        for _ in 0..num
        {
            let card = self.cards.remove(0);

            hand.total += card.value();
            if(hand.total > 21)
            {
                if let FACE::ACE = card.face
                {
                    hand.total -= 10;
                }
                else if let HAND_STATE::DOUBLE_DOWN = hand.state
                {
                    hand.state = HAND_STATE::DOUBLE_DOWN_BUST;
                }
                else 
                {
                    hand.state = HAND_STATE::BUST;
                }
            }
            if(hand.total == 21 && hand.cards.len() == 1)
            {
                if let HAND_STATE::DOUBLE_DOWN = hand.state
                {
                    hand.state = HAND_STATE::DOUBLE_DOWN_BJ;
                }
                else
                {
                    hand.state = HAND_STATE::BLACKJACK;
                }
            }

            hand.cards.push(card);
        }
    }

    pub fn deal_dealer(&mut self, hand: &mut Hand, num: usize)
    {
        for _ in 0..num
        {
            let card = self.cards.remove(0);

            hand.total += card.value();
            if(hand.total > 21)
            {
                if let FACE::ACE = card.face
                {
                    hand.total -= 10;
                }
                else if let HAND_STATE::DOUBLE_DOWN = hand.state
                {
                    hand.state = HAND_STATE::DOUBLE_DOWN_BUST;
                }
                else 
                {
                    hand.state = HAND_STATE::BUST;
                }
            }
            if(hand.total == 21 && hand.cards.len() == 1)
            {
                if let HAND_STATE::DOUBLE_DOWN = hand.state
                {
                    hand.state = HAND_STATE::DOUBLE_DOWN_BJ;
                }
                else
                {
                    hand.state = HAND_STATE::BLACKJACK;
                }
            }

            hand.cards.push(card);
        }
    }
}