use crate::deck::*;
// use crate::card::*;
use crate::settings::*;
use crate::hand::*;

enum GAME_STATE
{
    BETTING,
    PLAYER,
    DEALER,
    PAYOUT
}

pub struct Game
{
    deck: Deck,
    player_hands: Vec<Hand>,
    dealer_hand: Hand,
    game_state: GAME_STATE,
    player_chips: usize,
    selected_hand_index: usize,
    settings: Settings
}

impl Game
{
    pub fn new(settings: Settings) -> Self
    {
        Game
        {
            deck: Deck::new(settings.decks),
            player_hands: vec![Hand::new(0)],
            dealer_hand: Hand::new(0),
            game_state: GAME_STATE::BETTING,
            player_chips: settings.starting_chips,
            selected_hand_index: 0,
            settings
        }
    }

    pub fn begin(&mut self, bet: usize)
    {
        let mut deck = Deck::new(self.settings.decks);
        deck.shuffle();

        self.deck = deck;

        self.player_hands = vec![Hand::new(bet)];
        self.dealer_hand = Hand::new(0);

        self.deck.deal_dealer(&mut self.dealer_hand, 2);

        for hand in self.player_hands.iter_mut()
        {
            self.deck.deal(hand, 2);
        }

        if let HAND_STATE::BLACKJACK = self.dealer_hand.state
        {
            self.game_state = GAME_STATE::PAYOUT;
            return;
        }

        for hand in self.player_hands.iter()
        {
            if let HAND_STATE::BLACKJACK = hand.state
            {
                self.game_state = GAME_STATE::PAYOUT;
                return;
            }
        }

        self.game_state = GAME_STATE::PLAYER;
    }

    pub fn draw(&self)
    {
        match self.game_state
        {
            GAME_STATE::BETTING =>
            {
                println!(
"\x1bc\x1b[1;1HCurrent bet: {}

Availible chips: {}
[1] +5    [2] +10    [3] -5    [4] -10    [5] \x1b[1;4mB\x1b[0met
", self.player_hands[0].bet, self.player_chips - self.player_hands[0].bet);

            }
            GAME_STATE::PLAYER =>
            {
                print!("\x1bc");
                self.dealer_hand.draw(&self.settings, 1, 1, true);
                print!("\nDealer total: {} + ?", self.dealer_hand.cards[0].value());

                for (i, hand) in self.player_hands.iter().enumerate()
                {
                    if(i > self.selected_hand_index)
                    {
                        hand.draw(&self.settings, 1, 2 + &self.settings.dealer_player_offset + &self.settings.split_hands_offset * i, false);
                    }
                    else 
                    {
                        hand.draw(&self.settings, 1, 1 + &self.settings.dealer_player_offset + &self.settings.split_hands_offset * i, false);
                    }

                    print!("\nHand {} total: {}", i + 1, hand.total);

                    match hand.state
                    {
                        HAND_STATE::BUST => println!("    \x1b[31mBust!\x1b[0m"),
                        HAND_STATE::STAND => println!("    Standing"),
                        HAND_STATE::DOUBLE_DOWN => println!("    \x1b[93mDoubled Down!\x1b[0m"),
                        HAND_STATE::BLACKJACK => println!("    \x1b[97mBlackjack!\x1b[0m"),
                        _ => {println!("")}
                    }

                    if(i == self.selected_hand_index)
                    {
                        print!("[1] \x1b[1;4mH\x1b[0mit    [2] \x1b[1;4mS\x1b[0mtand    [3] \x1b[1;4mD\x1b[0mouble Down    ");
                        if(hand.splittable() != 0)
                        {
                            println!("[4] S\x1b[1;4mp\x1b[0mlit");
                        }
                        else 
                        {
                            println!("");
                        }
                    }
                }
            },
            GAME_STATE::DEALER =>
            {
                print!("\x1bc");
                self.dealer_hand.draw(&self.settings, 1, 1, false);
                print!("\nDealer total: {}", self.dealer_hand.total);

                match self.dealer_hand.state
                {
                    HAND_STATE::STAND => println!("    Standing"),
                    HAND_STATE::BUST => println!("    \x1b[31mBust!\x1b[0m"),
                    HAND_STATE::BLACKJACK => println!("    \x1b[97mBlackjack!\x1b[0m"),
                    _ => {println!("")}
                }

                for (i, hand) in self.player_hands.iter().enumerate()
                {
                    hand.draw(&self.settings, 1, 1 + &self.settings.dealer_player_offset + &self.settings.split_hands_offset * i, false);

                    print!("\nHand {} total: {}", i + 1, hand.total);

                    match hand.state
                    {
                        HAND_STATE::BUST => println!("    \x1b[31mBust!\x1b[0m"),
                        HAND_STATE::STAND => println!("    Standing"),
                        HAND_STATE::DOUBLE_DOWN => println!("    \x1b[93mDoubled Down!\x1b[0m"),
                        HAND_STATE::DOUBLE_DOWN_BUST => println!("    \x1b[93mDoubled Down!\x1b[0m  \x1b[31mBust!\x1b[0m"),
                        HAND_STATE::BLACKJACK => println!("    \x1b[97mBlackjack!\x1b[0m"),
                        HAND_STATE::DOUBLE_DOWN_BJ => println!("    \x1b[93mDoubled Down!\x1b[0m  \x1b[97mBlackjack!\x1b[0m"),
                        _ => {println!("")}
                    }
                }
            }
            GAME_STATE::PAYOUT =>
            {
                print!("\x1bc");
                self.dealer_hand.draw(&self.settings, 1, 1, false);
                print!("\nDealer total: {}", self.dealer_hand.total);

                match self.dealer_hand.state
                {
                    HAND_STATE::STAND => println!("    Standing"),
                    HAND_STATE::BUST => println!("    \x1b[31mBust!\x1b[0m"),
                    HAND_STATE::BLACKJACK => println!("    \x1b[97mBlackjack!\x1b[0m"),
                    _ => {println!("")}
                }

                for (i, hand) in self.player_hands.iter().enumerate()
                {
                    hand.draw(&self.settings, 1, 1 + &self.settings.dealer_player_offset + &self.settings.split_hands_offset * i, false);

                    print!("\nHand {} total: {}", i + 1, hand.total);

                    match hand.state
                    {
                        HAND_STATE::BUST => println!("    \x1b[31mBust!\x1b[0m"),
                        HAND_STATE::STAND => println!("    Standing"),
                        HAND_STATE::DOUBLE_DOWN => println!("    \x1b[93mDoubled Down!\x1b[0m"),
                        HAND_STATE::DOUBLE_DOWN_BUST => println!("    \x1b[93mDoubled Down!\x1b[0m  \x1b[31mBust!\x1b[0m"),
                        HAND_STATE::BLACKJACK => println!("    \x1b[97mBlackjack!\x1b[0m"),
                        HAND_STATE::DOUBLE_DOWN_BJ => println!("    \x1b[93mDoubled Down!\x1b[0m  \x1b[97mBlackjack!\x1b[0m"),
                        _ => {println!("")}
                    }
                }

                let payout = self.payout();
                if(payout > self.player_bet())
                {
                    println!("You win {} chips!", payout - self.player_bet())
                }
                else if(payout < self.player_bet())
                {
                    println!("You lose {} chips!", self.player_bet())
                }
                else 
                {
                    println!("Push! No chips were won or lost");
                }

                if(self.player_chips > 0)
                {
                    print!("[1] \x1b[1;4mP\x1b[0mlay again ({})    ", self.player_chips + payout);
                }
                println!("[2] \x1b[1;4mQ\x1b[0muit")
            }
        }
    }

    pub fn get_player_input(&mut self) -> bool
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
                        '1' => self.player_hands[0].bet += std::cmp::min(5, self.player_chips - self.player_hands[0].bet),
                        '2' => self.player_hands[0].bet += std::cmp::min(10, self.player_chips - self.player_hands[0].bet),
                        '3' => self.player_hands[0].bet -= std::cmp::min(5, self.player_hands[0].bet),
                        '4' => self.player_hands[0].bet -= std::cmp::min(10, self.player_hands[0].bet),
                        '5' | 'b' => 
                        {
                            self.player_chips -= self.player_hands[0].bet;
                            self.begin(self.player_hands[0].bet);
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

                            if(!self.look_for_next_hand())
                            {
                                self.game_state = GAME_STATE::DEALER;
                            }
                        },
                        '2' | 's' =>
                        {
                            self.player_hands[self.selected_hand_index].state = HAND_STATE::STAND;

                            if(!self.look_for_next_hand())
                            {
                                self.game_state = GAME_STATE::DEALER;
                            }
                        },
                        '3' | 'd' =>
                        {
                            let bonus_bet = std::cmp::min(self.player_hands[self.selected_hand_index].bet, self.player_chips);
                            self.player_chips -= bonus_bet;
                            self.player_hands[self.selected_hand_index].state = HAND_STATE::DOUBLE_DOWN;
                            self.player_hands[self.selected_hand_index].bet += bonus_bet;

                            self.deck.deal(&mut self.player_hands[self.selected_hand_index], 1);

                            if(!self.look_for_next_hand())
                            {
                                self.game_state = GAME_STATE::DEALER;
                            }
                        },
                        '4' | 'p' => 
                        {
                            let splitting_index = self.player_hands[self.selected_hand_index].splittable();
                            if(splitting_index != 0)
                            {
                                let new_bet = std::cmp::min(self.player_chips, self.player_hands[self.selected_hand_index].bet);
                                self.player_chips -= new_bet;
                                let new_hands = self.player_hands[self.selected_hand_index].split(splitting_index, new_bet).unwrap();

                                self.player_hands[self.selected_hand_index] = new_hands.0;
                                self.player_hands.insert(self.selected_hand_index + 1, new_hands.1);

                                self.deck.deal(&mut self.player_hands[self.selected_hand_index], 1);
                                self.deck.deal(&mut self.player_hands[self.selected_hand_index + 1], 1);

                                if(!self.look_for_next_hand())
                                {
                                    self.game_state = GAME_STATE::DEALER;
                                }
                            }
                        },
                        _ => {}
                    }
                }
            },
            GAME_STATE::DEALER =>
            {
                println!("Press Enter to continue the dealer's turn");

                let mut input = String::new();

                std::io::stdin().read_line(&mut input).expect("Couldn't read user input!");

                if(self.dealer_hand.total >= 17)
                {
                    if let HAND_STATE::NONE = self.dealer_hand.state
                    {
                        self.dealer_hand.state = HAND_STATE::STAND;
                    }
                    self.game_state = GAME_STATE::PAYOUT;
                }
                else 
                {
                    self.deck.deal(&mut self.dealer_hand, 1);
                }

            },
            GAME_STATE::PAYOUT =>
            {
                let mut input = String::new();

                std::io::stdin().read_line(&mut input).expect("Couldn't read user input!");

                for char in input.chars()
                {
                    match char.to_ascii_lowercase()
                    {
                        '1' | 'p' => 
                        {
                            if(self.player_chips > 0)
                            {
                                self.player_chips += std::cmp::max(self.payout(), 0) as usize;
                                self.dealer_hand = Hand::new(0);
                                self.player_hands = vec![Hand::new(0)];

                                self.game_state = GAME_STATE::BETTING;
                            }
                        },
                        '2' | 'q' =>
                        {
                            return true;
                        },
                        _ => {}
                    }
                }
            }
        }

        false
    }

    pub fn look_for_next_hand(&mut self) -> bool
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
                        return true;
                    }
                }
            }

            false
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
                        return true;
                    }
                }
            }

            false
        }
    }

    pub fn player_bet(&self) -> usize
    {
        let mut sum: usize = 0;

        for hand in self.player_hands.iter()
        {
            sum += hand.bet;
        }

        sum
    }

    pub fn payout(&self) -> usize
    {
        let mut result: usize = 0;
        let d_hand = &self.dealer_hand;

        use HAND_STATE::*;

        for hand in self.player_hands.iter()
        {
            result += hand.bet;

            match hand.state
            {
                NONE | STAND | DOUBLE_DOWN => 
                {
                    match d_hand.state
                    {
                        NONE | STAND =>
                        {
                            if(hand.total > d_hand.total)
                            {
                                result += hand.bet;
                            }
                            else if(hand.total < d_hand.total)
                            {
                                result -= hand.bet;
                            }
                        },
                        BUST => result += hand.bet,
                        _ => {}
                    }
                }
                BUST | DOUBLE_DOWN_BUST =>
                {
                    match d_hand.state
                    {
                        BUST =>
                        {
                            if(hand.total < d_hand.total)
                            {
                                result += hand.bet;
                            }
                            else if(hand.total > d_hand.total)
                            {
                                result -= hand.bet;
                            }
                        },
                        _ => result -= hand.bet
                    }
                }
                BLACKJACK | DOUBLE_DOWN_BJ =>
                {
                    match d_hand.state
                    {
                        BLACKJACK => continue,
                        _ => {result += hand.bet * 3 / 2}
                    }
                }
            }
        }

        result
    }
}