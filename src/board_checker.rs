
use {
    crate::board::Board,
    crate::player::Player,
    crate::util::{Marker, BOARD_SIZE, Coord}
};

pub enum ScoreState {
    Won(Player),
    Tie,
    Continue
}

pub trait BoardChecker {
    fn check(&self, board: &Board, player_x: &Player, player_o: &Player, last_coord: &Coord) -> ScoreState;
}

pub struct DefaultChecker;


impl BoardChecker for DefaultChecker {
    fn check(&self, board: &Board, player_x: &Player, player_o: &Player, last_coord: &Coord) -> ScoreState {
        match (self.crossed(board), self.column(board), self.row(board), self.full_board(board)) {
            (Some(marker), _, _, _) | (_, Some(marker), _, _) | (_, _, Some(marker), _) => ScoreState::Won(if player_x.marker == marker { player_x.clone() } else { player_o.clone() }),
            (_, _, _, true) => ScoreState::Tie,
            _ => ScoreState::Continue
        }
    }
}

impl DefaultChecker {
    fn crossed(&self, board: &Board) -> Option<Marker> {
        let cells = &board.cells;

        let left_cross =
            cells[0][0] == cells[1][1] &&
            cells[1][1] == cells[2][2] &&
            !(cells[0][0] == Marker::Empty);

        let right_cross =
            cells[2][2] == cells[1][1] &&
            cells[1][1] == cells[0][0] &&
            !(cells[2][2] == Marker::Empty);

        if left_cross || right_cross {
            Some(cells[1][1])
        } else {
            None
        }
    }

    fn column(&self, board: &Board) -> Option<Marker> {
        let cells = &board.cells;
        for i in 0..BOARD_SIZE {
            if cells[0][i] == cells[1][i] &&
                cells[1][i] == cells[2][i] &&
                !(cells[0][i] == Marker::Empty) {
                    return Some(cells[0][i])
                }
        }
        None
    }

    fn row(&self, board: &Board) -> Option<Marker> {
        let cells = &board.cells;
        for i in 0..BOARD_SIZE {
            if cells[i][0] == cells[i][1] &&
                cells[i][1] == cells[i][2] &&
                !(cells[i][0] == Marker::Empty) {
                    return Some(cells[i][0])
                }
        }
        None
    }

    fn full_board(&self, board: &Board) -> bool {
        for row in board.cells.iter() {
            if row.contains(&Marker::Empty) {
                return false
            }
        }
        true
    }
}
