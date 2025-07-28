use strum_macros::EnumIter;

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
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES
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
}