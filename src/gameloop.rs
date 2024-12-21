use crate::Game;
use std::io::Write;
use std::{io, result};
pub struct GameState {
    playing: bool,
    player_money: u32,
    dealer_money: u32,
    bet: u32,
}

pub enum GameResult {
    PlayerWins,
    DealerWins,
    Tie,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            playing: false,
            player_money: 100,
            dealer_money: 100,
            bet: 0,
        }
    }
    pub fn setPlaying(&mut self, playing: bool) {
        self.playing = playing;
    }
}

pub fn game_loop() {
    loop {
        let logo = r#"
        ____  _            _    _            _    
        |  _ \| |          | |  (_)          | |   
        | |_) | | __ _  ___| | ___  __ _  ___| | __
        |  _ <| |/ _` |/ __| |/ / |/ _` |/ __| |/ /
        | |_) | | (_| | (__|   <| | (_| | (__|   < 
        |____/|_|\__,_|\___|_|\_\ |\__,_|\___|_|\_\
                               _/ |                
                              |__/                 
       "#;

        println!("{}", logo);
        println!("1. New Game");
        println!("2. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => start_game(),
            "2" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn start_game() {
    clear_console();
    println!("Starting game...");
    let mut state = GameState::new();
    state.setPlaying(true);
    let mut game = Game::new();
    game.deal_initial_cards();
    play(&mut game, &mut state);
}

fn play(game: &mut Game, state: &mut GameState) {
    clear_console();
    println!("--------------------------------------------------------------------");
    println!("You are currently in crippling debt. However, you're feeling lucky today.");
    println!("You decide to play a game of blackjack with the local loan shark.");
    println!("You have 100 dollars. The dealer also has 100 dollars. One of you will leave this place with 200 bucks, the other with nothing.");
    println!("Good luck. If you lose, say goodbye to your kneecaps.");
    println!("--------------------------------------------------------------------");

    while state.playing {
        if state.player_money == 0 || state.dealer_money == 0 {
            state.playing = false;
            break;
        }

        let mut bet_set: bool = false;

        while (!bet_set) {
            println!("Your money: {}", state.player_money);
            println!("Dealer money: {}", state.dealer_money);
            println!("Place your bet.");
            let mut bet = String::new();
            io::stdin()
                .read_line(&mut bet)
                .expect("Failed to read line");
            let bet: u32 = match bet.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid bet. Please enter a number.");
                    continue;
                }
            };
            if bet > state.player_money || bet > state.dealer_money {
                println!("Either you or the dealer don't have enough money to place that bet.");
                clear_console();
                continue;
            } else {
                state.bet = bet;
                bet_set = true;
            }
        }

        let result: GameResult = play_a_game(game);
        match result {
            GameResult::PlayerWins => {
                println!("Player wins!");
                state.player_money += state.bet;
                state.dealer_money -= state.bet;
                if state.dealer_money == 0 {
                    println!("The dealer is out of money. You win! Your debt is cleared.");
                    game_over();
                    state.playing = false;
                }
            }
            GameResult::DealerWins => {
                println!("Dealer wins!");
                state.player_money -= state.bet;
                state.dealer_money += state.bet;
                if state.player_money == 0 {
                    println!("You're out of money. The loan shark looks at your kneecaps with malicious intent.");
                    game_over();
                    state.playing = false;
                }
            }
            GameResult::Tie => {
                println!("It's a tie!");
            }
        }
        game.reset();
    }
}

fn play_a_game(game: &mut Game) -> GameResult {
    print_player_hand(&game);
    let mut result: GameResult = GameResult::Tie;
    loop {
        let mut action = String::new();
        println!("Hit or stand? (h/s)");
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "h" => {
                game.player_hit();
                print_player_hand(&game);
                if game.player_value() > 21 {
                    println!("Player busts!");
                    result = GameResult::DealerWins;
                    break;
                }
            }
            "s" => {
                while game.dealer_value() < 17 {
                    game.dealer_hit();
                }
                print_dealer_hand(&game);
                if game.dealer_value() > 21 {
                    println!("Dealer busts!");
                    result = GameResult::PlayerWins;
                } else if game.dealer_value() > game.player_value() {
                    result = GameResult::DealerWins;
                } else if game.dealer_value() < game.player_value() {
                    result = GameResult::PlayerWins;
                } else {
                    result = GameResult::Tie;
                }
                break;
            }
            _ => println!("Invalid action. Please enter 'h' or 's'."),
        }
    }
    result
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

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn game_over() {
    println!("Game over! Press any key to return to the main menu.");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
}
