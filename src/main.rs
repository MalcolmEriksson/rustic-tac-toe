#[macro_use]
extern crate lazy_static;

pub mod state;
pub mod player;
pub mod util;
pub mod board;
pub mod app;
pub mod board_checker;
pub mod my_checker;

use {
    util::{
        Marker,
        Coord,
        BOARD_SIZE
    },
    player::Player,
    state::Game,
    app::App,
    std::io,
};

fn main() {

    let players = get_new_players();
    App::new(players.0, players.1, Box::new(crate::my_checker::MyChecker))
        .run();
}

fn get_new_players() -> (Player, Player) {
    let mut ret = (None, None);
    (0..2).for_each(|i| {
        let mut input = String::new();
        println!("Player {}, enter your name!", i);
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match i {
                    0 => ret.0 = Some(Player::new(&input, Marker::X)),
                    _ => ret.1 = Some(Player::new(&input, Marker::O))
                }
            }
            Err(error) => println!("error: {}", error),
        }
    });
    (ret.0.unwrap(), ret.1.unwrap())
}



