use strum_macros::EnumIter;
use crate::settings::Settings;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum FACE
{
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum SUIT
{
    CLUBS = 0,
    DIAMONDS = 1,
    HEARTS = 2,
    SPADES = 3
} 

#[derive(Debug)]
pub struct Card
{
    face: FACE,
    suit: SUIT
}

impl Card
{
    pub fn new(face: FACE, suit: SUIT) -> Self
    {
        Card
        {
            face,
            suit
        }
    }

    pub fn draw(&self, settings: &Settings, x: usize, y: usize)
    {
        let sprite = &settings.cards[self.suit as usize][self.face as usize - 2];

        for (i, line) in sprite.iter().enumerate()
        {
            match self.suit 
            {
                SUIT::CLUBS => 
                {
                    print!("\x1b[{};{}H{}", y + i, x, line);
                },
                SUIT::DIAMONDS => 
                {
                    print!("\x1b[31m");
                    print!("\x1b[{};{}H{}", y + i, x, line);
                    print!("\x1b[0m");
                },
                SUIT::HEARTS => 
                {
                    print!("\x1b[31m");
                    print!("\x1b[{};{}H{}", y + i, x, line);
                    print!("\x1b[0m");
                },
                SUIT::SPADES => 
                {
                    print!("\x1b[{};{}H{}", y + i, x, line);
                }
            }
        }
    }
}