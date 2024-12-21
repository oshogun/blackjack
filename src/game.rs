use crate::card::{Card, Deck};

pub struct Game {
    deck: Deck,
    player_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new();
        deck.shuffle();
        Game {
            deck,
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
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
