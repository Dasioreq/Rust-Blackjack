use crate::deck::*;
use crate::card::*;
use crate::settings::*;
use crate::hand::*;

enum GAME_STATE
{
    BETTING,
    PLAYER,
    DEALER
}

pub struct Game
{
    deck: Deck,
    player_hands: Vec<Hand>,
    dealer_hand: Hand,
    game_state: GAME_STATE,
    player_chips: usize,
    current_bet: usize
}

impl Game
{
    pub fn new(settings: &Settings) -> Self
    {
        let mut deck = Deck::new(settings.decks);
        deck.shuffle();
        Game
        {
            deck,
            player_hands: vec![Hand::new()],
            dealer_hand: Hand::new(),
            game_state: GAME_STATE::BETTING,
            player_chips: settings.starting_chips,
            current_bet: 0
        }
    }

    pub fn begin(&mut self, bet: usize)
    {
        self.deck.deal_dealer(&mut self.dealer_hand, 2);
        self.deck.deal(&mut self.player_hands, 2);

        self.game_state = GAME_STATE::PLAYER;
    }

    pub fn draw(&self, settings: &Settings)
    {
        match self.game_state
        {
            GAME_STATE::BETTING =>
            {
                println!(
"\x1bc\x1b[1;1HCurrent bet: {}

Availible chips: {}
[1] +5\t[2] +10\t[3] -5\t[4] -10\t[5] \x1b[1;4mB\x1b[0met
", self.current_bet, self.player_chips - self.current_bet);

            }
            GAME_STATE::PLAYER =>
            {
                print!("\x1bc");
                self.dealer_hand.draw(settings, 1, 1, true);
                print!("\nDealer total: {} + ?", self.dealer_hand.cards[0].value());

                for (i, hand) in self.player_hands.iter().enumerate()
                {
                    hand.draw(settings, 1, 1 + settings.dealer_player_offset + settings.split_hands_offset * i, false);
                    print!("\nHand {} total: {}", i + 1, hand.total)
                }
            }
            _ => {}
        }
    }
}