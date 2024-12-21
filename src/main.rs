mod card;
mod game;
mod gameloop;
use game::Game;
use gameloop::game_loop;

fn main() {
    let game: Game = Game::new();
    game_loop();
}
