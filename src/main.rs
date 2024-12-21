mod card;
mod game;

use game::Game;
use std::io;

fn main() {
    let mut game = Game::new();
    game.deal_initial_cards();
    println!("--------------------------------------------------------------------");
    println!("You are currently in crippling debt. However, you're feeling lucky today.");
    println!("You decide to play a game of blackjack with the local loan shark.");
    println!("You have 100 dollars. The dealer also has 100 dollars. One of you will leave this place with 200 bucks, the other with nothing.");
    println!("Good luck. If you lose, say goodbye to your kneecaps.");
    println!("--------------------------------------------------------------------");

    let mut running = true;
    while running {
        loop {
            println!("Player money: {}", game.player_money);
            println!("Dealer money: {}", game.dealer_money);

            if game.player_money == 0 || game.dealer_money == 0 {
                running = check_game_status(&game);
                if !running {
                    break;
                }
            }

            let mut bet: u32 = game.current_bet;
            if bet == 0 {
                println!("PLACE YOUR BET: ");
            }
            let mut input = String::new();
            while bet == 0 {
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter a number");
                bet = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
            }
            Game::bet(&mut game, bet);

            print_player_hand(&game);

            if Game::is_bust(game.player_value()) {
                println!("Player busts!");
                Game::reward_winner(&mut game, "dealer");
                running = check_game_status(&game);
                break;
            }

            println!("Hit or stand? (h/s)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "h" => {
                    println!("Player hits.");
                    game.player_hit();
                    print_player_hand(&game);
                    if Game::is_bust(game.player_value()) {
                        println!("Player busts!");
                        Game::reward_winner(&mut game, "dealer");
                        running = check_game_status(&game);
                        break;
                    }
                },
                "s" => break,
                _ => continue,
            }
        }

        if !running {
            break;
        }

        while game.dealer_value() < 17 {
            game.dealer_hit();
            println!("Dealer hits.");
            print_dealer_hand(&game);
        }

        print_dealer_hand(&game);

        if Game::is_bust(game.dealer_value()) {
            println!("Dealer busts! Player wins.");
            Game::reward_winner(&mut game, "player");
        } else if game.dealer_value() > game.player_value() {
            println!("Dealer wins.");
            Game::reward_winner(&mut game, "dealer");
        } else if game.dealer_value() < game.player_value() {
            println!("Player wins.");
            Game::reward_winner(&mut game, "player");
        } else {
            println!("It's a tie.");
            Game::reward_winner(&mut game, "tie");
        }

        running = check_game_status(&game);
        if !running {
            break;
        }

        Game::reset(&mut game);
    }
}

fn check_game_status(game: &Game) -> bool {
    if game.player_money == 0 {
        println!("You ran out of money. The loan shark takes your kneecaps.");
        return false
    } else if game.dealer_money == 0 {
        println!("The dealer is out of money. You are free!");
        return false
    }
    true
}

fn print_player_hand(game: &Game) {
    println!("---------------------------------");
    println!("Player hand: {:?}", game.get_formatted_hand("player"));
    println!("Player hand value: {}", game.player_value());
    println!("---------------------------------");
}

fn print_dealer_hand(game: &Game) {
    println!("---------------------------------");
    println!("Dealer hand: {:?}", game.get_formatted_hand("dealer"));
    println!("Dealer hand value: {}", game.dealer_value());
    println!("---------------------------------");
}