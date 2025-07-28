#![allow(unused_parens)]

use crate::deck::*;
use crate::card::*;
mod deck;
mod card;

fn main()
{
    let mut b: Deck = Deck::new(1);
    b.shuffle();

    println!("{:#?}", b.cards.len());
}