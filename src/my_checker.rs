use {
    crate::{
        board_checker::{BoardChecker, ScoreState},
        player::Player,
        board::Board,
        util::{Marker, Coord},
    }
};


pub struct MyChecker;

impl BoardChecker for MyChecker {
    fn check(&self, _board: &Board, player_x: &Player, player_o: &Player, last_coord: &Coord) -> ScoreState {
        self.score(_board, last_coord, player_x, player_o)
    }
}

impl MyChecker {
    fn score(&self, board: &Board, last_coord: &Coord, player_x: &Player, player_o: &Player) -> ScoreState {
        match(self.row(board, last_coord.0), self.column(board, last_coord.1), self.diagonal(board, last_coord)) {
            (Some(marker), _, _) => {
                return ScoreState::Won(self.winning_player(marker, player_x , player_o))
            },
            (_, Some(marker), _) => {
                return ScoreState::Won(self.winning_player(marker, player_x , player_o))
            },
            (_, _, Some(marker)) => {
                return ScoreState::Won(self.winning_player(marker, player_x , player_o))
            },
            _ => self.full_board(board)
        }
    }

    fn column(&self, board: &Board, last_y: usize) -> Option<Marker> {
        let cells = &board.cells;
        if cells[0][last_y] == cells[1][last_y] && cells[0][last_y] == cells[2][last_y]  {
                return Some(cells[0][last_y]);
            }
        None
    }

    fn row (&self, board: &Board, last_x: usize) -> Option<Marker> {
        let cells = &board.cells;
        if cells[last_x][0] == cells[last_x][1] &&  cells[last_x][0] == cells[last_x][2]  {
                return Some(cells[last_x][0]);
            }
        None
    }

    fn diagonal(&self, board: &Board, last_coord: &Coord) -> Option<Marker> {
        let last_x = last_coord.0;
        let last_y = last_coord.1;
        let cells = &board.cells;
        if last_x == last_y {
            if cells[0][0] == cells[1][1] && cells[2][2] == cells[1][1] {
                return Some(cells[1][1]);
            }
            return None;
        } else if last_x + last_y == 2 {
            if cells[2][0] == cells[1][1] && cells[0][2] == cells[1][1] {
                return Some(cells[1][1]);
            }
            return None;
        }
        None
    }

    fn full_board(&self, board: &Board) -> ScoreState {
        for i in 0..2 {
            if board.cells[i].contains(&Marker::Empty){
                return ScoreState::Continue;
            }
        }
        ScoreState::Tie
    }

    fn winning_player(&self, winning_marker: Marker, player_x: &Player, player_o: &Player) -> Player {
        if player_x.marker == winning_marker { player_x.clone() } else { player_o.clone()}
    }
}
