use crate::go::{board::Board, coordinate::Coordinate, player::Player};
use std::fmt::Debug;

#[derive(Clone)]
pub struct GameState {
    board: Board,
    captured_black: u32, // captured BY black
    captured_white: u32,
    current_player: Player,
}

impl GameState {
    pub fn init() -> Self {
        GameState {
            board: Board::init(),
            current_player: Player::Black,
            captured_black: 0,
            captured_white: 0,
        }
    }

    pub fn make_move(&self, place: Coordinate) -> Option<Self> {
        if !self.is_legal_move(place) {
            return None;
        }

        let mut new_board = self.board.with_move(place, self.current_player);

        let (c_b, c_w) = new_board.clean_taken_pieces(self.current_player);

        Some(Self {
            board: new_board,
            captured_black: self.captured_black + c_b,
            captured_white: self.captured_white + c_w,
            current_player: !self.current_player,
        })
    }

    pub fn get_legal_moves(&self) -> Vec<Coordinate> {
        // use is_legal_move
        let mut out: Vec<Coordinate> = vec![];
        for i in 0..(19 * 19) {
            let coord = Coordinate::from_index(i);
            if self.is_legal_move(coord) {
                out.push(coord);
            }
        }
        out
    }

    fn is_legal_move(&self, place: Coordinate) -> bool {
        if !self.board.is_free(place) {
            return false;
        }
        self.board.can_place_at(place, self.current_player)
    }
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_string();
        for i in 0..19 {
            for j in 0..19 {
                let index = Coordinate::init(i, j);
                match self.board.get_piece_at(index) {
                    None => out += "_ ",
                    Some(Player::Black) => out += "X ",
                    Some(Player::White) => out += "O ",
                }
            }
            out += "\n";
        }
        f.write_str(out.as_str())
    }
}

#[test]
fn test_player_moves() {
    // tests whether the current player is swapped
    let mut state = GameState::init();
    assert!(state.current_player == Player::Black);
    state = state.make_move(Coordinate::from_index(0)).unwrap();
    assert!(state.current_player == Player::White);
    state = state.make_move(Coordinate::from_index(1)).unwrap();
    assert!(state.current_player == Player::Black);
}

#[test]
fn test_try_move() {
    let mut state = GameState::init();
    state = state.make_move(Coordinate::from_index(0)).unwrap();
    assert!(!state.is_legal_move(Coordinate::from_index(0)));
    assert!(state.is_legal_move(Coordinate::from_index(1)));
}

#[test]
fn test_get_moves() {
    // tests that an empty board only has valid moves
    let mut state = GameState::init();
    assert!(state.get_legal_moves().len() == (19 * 19));

    // test that filling one spot results in 1 less valid move
    state = state.make_move(Coordinate::from_index(0)).unwrap();
    let moves = state.get_legal_moves();
    assert!(moves.len() == (19 * 19) - 1);
    assert!(moves.iter().all(|mov| mov.index != 0));
}

// TODO: test capturing, scoring

// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o B B B o o o o o
// o o o o B B B o o o o B x B o o o o o
// o o o o B B B o o o o B B B o o o o o
// o o o o B B B o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
// o o o o o o o o o o o o o o o o o o o
