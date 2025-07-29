use strum_macros::EnumIter;
use crate::settings::Settings;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum FACE
{
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum SUIT
{
    CLUBS = 0,
    DIAMONDS = 1,
    HEARTS = 2,
    SPADES = 3,
} 

#[derive(Debug, Clone, Copy)]
pub struct Card
{
    pub face: FACE,
    pub suit: SUIT,
}

impl FACE
{
    pub fn index(&self) -> usize
    {
        match self
        {
            FACE::TWO => 0,
            FACE::THREE => 1,
            FACE::FOUR => 2,
            FACE::FIVE => 3,
            FACE::SIX => 4,
            FACE::SEVEN => 5,
            FACE::EIGHT => 6,
            FACE::NINE => 7,
            FACE::TEN => 8,
            FACE::JACK => 9,
            FACE::QUEEN => 10,
            FACE::KING => 11,
            FACE::ACE => 12,
        }
    }
}

impl Card
{
    pub fn new(face: FACE, suit: SUIT) -> Self
    {
        Card
        {
            face,
            suit,
        }
    }

    pub fn draw(&self, settings: &Settings, x: usize, y: usize, shown: bool)
    {
        let sprite = match shown
        {
            true => &settings.cards[self.suit as usize][self.face.index()],
            false => &settings.reverse
        };

        for (i, line) in sprite.iter().enumerate()
        {
            if(shown)
            {
                match self.suit 
                {
                    SUIT::DIAMONDS | SUIT::HEARTS => 
                    {
                        print!("\x1b[31m");
                        print!("\x1b[{};{}H{}", y + i, x, line);
                        print!("\x1b[0m");
                    },
                    _ => 
                    {
                        print!("\x1b[{};{}H{}", y + i, x, line);
                    }
                }
            }
            else 
            {
                print!("\x1b[{};{}H{}", y + i, x, line);
            }
        }
    }

    pub fn value(&self) -> u8
    {
        match self.face
        {
            FACE::TWO => 2,
            FACE::THREE => 3,
            FACE::FOUR => 4,
            FACE::FIVE => 5,
            FACE::SIX => 6,
            FACE::SEVEN => 7,
            FACE::EIGHT => 8,
            FACE::NINE => 9,
            FACE::TEN => 10,
            FACE::JACK => 10,
            FACE::QUEEN => 10,
            FACE::KING => 10,
            FACE::ACE => 11,
        }
    }
}