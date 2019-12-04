#[macro_use]
extern crate lazy_static;

pub mod state;
pub mod player;
pub mod util;
pub mod board;

use {
    util::{
        Marker,
        Coord
    },
    player::Player,
    state::Game,
    std::io,
};

fn read_user_input(current_player: &Player) -> (Coord, Marker){
    println!("Please enter coordinates separated with a comma eg 0,1");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                match input.trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>().as_slice() {
                    [first, second] if first.parse::<usize>().is_ok() && second.parse::<usize>().is_ok() => {
                        let (first, second) = (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap());
                        return (Coord::new(first, second), current_player.marker)
                    },
                    x => {
                        println!("Unsupported input: {:?}", x)
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}




fn main() {
    let player = Player::new("Pierre", Marker::X);
    let mut game = Game::new(player);
    game.board = game.board.add_marker((2,2), Marker::X);
    println!("{}", game.board);
}
