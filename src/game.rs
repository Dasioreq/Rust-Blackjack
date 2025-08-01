use crate::deck::*;
// use crate::card::*;
use crate::settings::*;
use crate::hand::*;

enum GAME_STATE
{
    BETTING,
    PLAYER,
    DEALER,
    INTERMISSION
}

pub struct Game
{
    deck: Deck,
    player_hands: Vec<Hand>,
    dealer_hand: Hand,
    game_state: GAME_STATE,
    player_chips: usize,
    current_bet: usize,
    selected_hand_index: usize
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
            player_hands: vec![Hand::new(), Hand::new(), Hand::new()],
            dealer_hand: Hand::new(),
            game_state: GAME_STATE::BETTING,
            player_chips: settings.starting_chips,
            current_bet: 0,
            selected_hand_index: 0
        }
    }

    pub fn begin(&mut self)
    {
        self.deck.deal_dealer(&mut self.dealer_hand, 2);

        for hand in self.player_hands.iter_mut()
        {
            self.deck.deal(hand, 2);
        }
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
[1] +5    [2] +10    [3] -5    [4] -10    [5] \x1b[1;4mB\x1b[0met
", self.current_bet, self.player_chips - self.current_bet);

            }
            GAME_STATE::PLAYER =>
            {
                print!("\x1bc");
                self.dealer_hand.draw(settings, 1, 1, true);
                print!("\nDealer total: {} + ?", self.dealer_hand.cards[0].value());

                for (i, hand) in self.player_hands.iter().enumerate()
                {
                    if(i > self.selected_hand_index)
                    {
                        hand.draw(settings, 1, 2 + settings.dealer_player_offset + settings.split_hands_offset * i, false);
                    }
                    else 
                    {
                        hand.draw(settings, 1, 1 + settings.dealer_player_offset + settings.split_hands_offset * i, false);
                    }

                    print!("\nHand {} total: {}", i + 1, hand.total);

                    match hand.state
                    {
                        HAND_STATE::BUST => println!("    \x1b[31mBust!\x1b[0m"),
                        HAND_STATE::STAND => println!("    Standing"),
                        HAND_STATE::DOUBLE_DOWN(x) => println!("    \x1b[93mDoubled Down ({x})!\x1b[0m"),
                        _ => {println!("")}
                    }

                    if(i == self.selected_hand_index)
                    {
                        println!("[1] \x1b[1;4mH\x1b[0mit    [2] \x1b[1;4mS\x1b[0mtand    [3] \x1b[1;4mD\x1b[0mouble Down    ");
                        if(hand.splittable())
                        {
                            print!("[4] S\x1b[1;4mp\x1b[0mlit");
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_player_input(&mut self)
    {
        match self.game_state
        {
            GAME_STATE::BETTING =>
            {
                let mut input = String::new();

                std::io::stdin().read_line(&mut input).expect("Couldn't read user input!");

                for char in input.chars()
                {
                    match char.to_ascii_lowercase()
                    {
                        '1' => self.current_bet += std::cmp::min(5, self.player_chips - self.current_bet),
                        '2' => self.current_bet += std::cmp::min(10, self.player_chips - self.current_bet),
                        '3' => self.current_bet -= std::cmp::min(5, self.current_bet),
                        '4' => self.current_bet -= std::cmp::min(10, self.current_bet),
                        '5' | 'b' => 
                        {
                            self.player_chips -= self.current_bet;
                            self.begin();
                        },
                        _ => {}
                    }
                }
            },
            GAME_STATE::PLAYER => 
            {
                let mut input = String::new();

                std::io::stdin().read_line(&mut input).expect("Couldn't read user input!");

                for char in input.chars()
                {
                    match char.to_ascii_lowercase()
                    {
                        '1' | 'h' => 
                        {
                            self.deck.deal(&mut self.player_hands[self.selected_hand_index], 1);
                            self.look_for_next_hand();
                        },
                        '2' | 's' =>
                        {
                            self.player_hands[self.selected_hand_index].state = HAND_STATE::STAND;
                            self.look_for_next_hand();
                        },
                        '3' | 'd' =>
                        {
                            self.deck.deal(&mut self.player_hands[self.selected_hand_index], 1);
                            self.player_hands[self.selected_hand_index].state = HAND_STATE::DOUBLE_DOWN(self.current_bet);
                            self.look_for_next_hand();
                        },
                        '4' => self.current_bet -= std::cmp::min(10, self.current_bet),
                        '5' | 'b' => 
                        {
                            self.player_chips -= self.current_bet;
                            self.begin();
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn look_for_next_hand(&mut self)
    {
        if(self.selected_hand_index == self.player_hands.len() - 1)
        {
            for i in 0..self.player_hands.len()
            {
                if let HAND_STATE::NONE = self.player_hands[i].state
                {
                    if(self.player_hands[i].total <= 21)
                    {
                        self.selected_hand_index = i;
                        break;
                    }
                }
            }
        }
        else 
        {
            for i in (self.selected_hand_index + 1)..self.player_hands.len()
            {
                if let HAND_STATE::NONE = self.player_hands[i].state
                {
                    if(self.player_hands[i].total <= 21)
                    {
                        self.selected_hand_index = i;
                        break;
                    }
                }
            }
        }
    }
}