mod card;
mod game;

use game::Game;
use std::io;

fn main() {
    let mut game = Game::new();
    game.deal_initial_cards();

    loop {
        println!("Player hand: {:?}", game.get_formatted_player_hand());
        println!("Player hand value: {}", game.player_value());

        if Game::is_bust(game.player_value()) {
            println!("Player busts!");
            break;
        }

        println!("Hit or stand? (h/s)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "h" => game.player_hit(),
            "s" => break,
            _ => continue,
        }
    }

    while game.dealer_value() < 17 {
        game.dealer_hit();
    }

    println!("Dealer hand: {:?}", game.get_formatted_dealer_hand());
    println!("Dealer hand value: {}", game.dealer_value());

    if Game::is_bust(game.dealer_value()) {
        println!("Dealer busts! Player wins.");
    } else if game.dealer_value() > game.player_value() {
        println!("Dealer wins.");
    } else if game.dealer_value() < game.player_value() {
        println!("Player wins.");
    } else {
        println!("It's a tie.");
    }
}
