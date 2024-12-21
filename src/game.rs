use crate::card::{Card, Deck};

pub struct Game {
    deck: Deck,
    player_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
    pub player_money: u32,
    pub dealer_money: u32,
    pub current_bet: u32,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new();
        deck.shuffle();
        Game {
            deck,
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            player_money: 100,
            dealer_money: 100,
            current_bet: 0,
        }
    }

    pub fn reset(&mut self) {
        self.deck = Deck::new();
        self.deck.shuffle();
        self.player_hand = Vec::new();
        self.dealer_hand = Vec::new();
        self.deal_initial_cards();
    }

    pub fn deal_initial_cards(&mut self) {
        for _ in 0..2 {
            self.player_hand.push(self.deck.draw().unwrap());
            self.dealer_hand.push(self.deck.draw().unwrap());
        }
    }

    pub fn player_hit(&mut self) {
        self.player_hand.push(self.deck.draw().unwrap());
    }

    pub fn dealer_hit(&mut self) {
        self.dealer_hand.push(self.deck.draw().unwrap());
    }

    pub fn calculate_hand_value(hand: &[Card]) -> u8 {
        hand.iter().map(|card| card.value()).sum()
    }

    pub fn player_value(&self) -> u8 {
        Self::calculate_hand_value(&self.player_hand)
    }

    pub fn dealer_value(&self) -> u8 {
        Self::calculate_hand_value(&self.dealer_hand)
    }

    pub fn bet(&mut self, amount: u32) {
        if amount > self.player_money || amount > self.dealer_money {
            // bet the full amount of whoever has the least money
            let min_money = std::cmp::min(self.player_money, self.dealer_money);
            self.current_bet = min_money;
        } else {
            self.current_bet = amount;
        }
        self.player_money -= self.current_bet;
        self.dealer_money -= self.current_bet;
    }

    pub fn reward_winner(&mut self, winner: &str) {
        match winner {
            "player" => self.player_money += self.current_bet * 2,
            "dealer" => self.dealer_money += self.current_bet * 2,
            "tie" => {
                self.player_money += self.current_bet;
                self.dealer_money += self.current_bet;
            },
            _ => (),
        }
        self.current_bet = 0;
    }



    pub fn is_bust(value: u8) -> bool {
        value > 21
    }

    pub fn get_formatted_hand(&self, player_or_dealer: &str) -> String {
        let mut formatted_hand = String::new();
        match player_or_dealer {
            "player" => {
                for card in &self.player_hand {
                    formatted_hand.push_str(&format!("[{}] ", Card::get_art(card)));
                }
            }
            "dealer" => {
                for card in &self.dealer_hand {
                    formatted_hand.push_str(&format!("[{}] ", Card::get_art(card)));
                }
            }
            _ => {}
        }
        formatted_hand
    }
    
}   