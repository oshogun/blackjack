use crate::card::{Card, Deck};

pub struct Game {
    deck: Deck,
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
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

    pub fn get_formatted_player_hand(&self) -> String {
        let mut formatted_hand = String::new();
        formatted_hand.push_str("Player hand: ");
        for card in &self.player_hand {
            formatted_hand.push_str(&format!("a {} of {}, ", card.rank, card.suit));
        }
        formatted_hand
    }

    pub fn get_formatted_dealer_hand(&self) -> String {
        let mut formatted_hand = String::new();
        formatted_hand.push_str("Dealer hand: ");
        for card in &self.dealer_hand {
            formatted_hand.push_str(&format!("{} of {}, ", card.rank, card.suit));
        }
        formatted_hand
    }
}