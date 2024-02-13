use wheels::engine::game::{ Game, Winner };

fn main() {
    println!("Welcome to Wheels!");
    let winner = Game::get_choose_singleplayer().run_singleplayer();
    match winner {
        Winner::P1 => { println!("Player 1 wins!"); },
        Winner::P2 => { println!("Player 2 wins!"); },
        Winner::Draw => { println!("Draw!"); },
    }
}
