use thiserror::Error;

use crate::go::{
    board::FlexibleBoard,
    player::Player,
    playermove::{Move, PlaceStoneMove},
};

pub struct Game<TBoard: FlexibleBoard> {
    board: TBoard,
    captured_by_black: u16,
    captured_by_white: u16,
    current_player: Player,
}

impl<TBoard: FlexibleBoard> Game<TBoard> {
    pub fn new(board: TBoard) -> Self {
        Game {
            board,
            captured_by_black: 0,
            captured_by_white: 0,
            current_player: Player::Black,
        }
    }

    pub fn make_move(&mut self, m: &Move) -> Result<(), MoveError> {
        match m {
            Move::PlaceStone(place_stone_move) => {
                let PlaceStoneMove { coord, player } = place_stone_move;
                let occupying_player = self.board.get_player_at(&coord);
                if let Some(occupied_by) = occupying_player {
                    return Err(MoveError::CoordinateOccupied { occupied_by });
                }

                let groups_to_capture = self
                    .board
                    .find_groups_to_capture_from_move(&place_stone_move);

                if groups_to_capture.is_empty() {
                    let is_suicide = self.board.is_potential_suicide(place_stone_move);

                    if is_suicide {
                        return Err(MoveError::Suicide);
                    }
                }

                let mut captured = 0;

                for group in groups_to_capture {
                    captured += self
                        .board
                        .capture(&group.coordinates)
                        .expect("Expected capture to work");
                }

                match player {
                    Player::Black => self.captured_by_black += captured,
                    Player::White => self.captured_by_white += captured,
                };

                self.board
                    .set_player_at(&coord, &player)
                    .expect("Already checked whether spot is occupied or not");

                Ok(())
            }
            Move::Skip { .. } => {
                self.current_player = !self.current_player;
                Ok(())
            },
        }
    }

    pub fn get_board(&self) -> &TBoard {
        &self.board
    }
}

#[derive(Debug, Error)]
pub enum MoveError {
    #[error("Can not place a stone where there already is a stone.")]
    CoordinateOccupied { occupied_by: Player },
    #[error("Killing yourself is not nice, we'd like for you to live thank you.")]
    Suicide,
}

#[cfg(test)]
mod test {
    use crate::go::{
        bitmask::TestMask,
        bitmask_board::BitMaskBoard,
        coordinate::FlexibleCoordinate,
        player::{B, W},
    };

    use super::*;

    #[test]
    fn given_empty_game_when_make_move_is_called_then_it_should_place_the_stone() {
        // Given
        let board = BitMaskBoard::new(|| TestMask::empty((9, 9)));
        let mut game = Game::new(board);

        // When
        let res = game.make_move(&Move::PlaceStone(PlaceStoneMove {
            player: Player::Black,
            coord: FlexibleCoordinate { x: 0, y: 0 },
        }));

        // Then
        assert!(res.is_ok());
        let stone_at_placed_coord = game
            .get_board()
            .get_player_at(&FlexibleCoordinate { x: 0, y: 0 });
        assert_eq!(stone_at_placed_coord, Some(Player::Black));
    }

    #[test]
    fn given_game_with_capture_position_when_make_move_is_called_then_it_should_capture_the_stones()
    {
        // Given
        let e = None;
        let position = vec![
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, B, e, e, e, e],
            vec![e, e, B, W, W, B, e, e, e],
            vec![e, e, e, B, W, B, e, e, e],
            vec![e, e, e, e, B, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let board = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), position);
        let mut game = Game::new(board);

        // When
        let res = game.make_move(&Move::PlaceStone(PlaceStoneMove {
            player: Player::Black,
            coord: FlexibleCoordinate { x: 3, y: 2 },
        }));

        // Then
        assert!(res.is_ok());

        let expected_position = vec![
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, B, B, e, e, e, e],
            vec![e, e, B, e, e, B, e, e, e],
            vec![e, e, e, B, e, B, e, e, e],
            vec![e, e, e, e, B, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let expected_board =
            BitMaskBoard::from_position(|| TestMask::empty((9, 9)), expected_position);

        assert_eq!(&expected_board, game.get_board());
        assert_eq!(3, game.captured_by_black);
    }

    #[test]
    fn given_an_empty_board_when_a_full_game_is_played_then_it_should_have_actual_results() {
        // Given
        let board = BitMaskBoard::new(|| TestMask::empty((9, 9)));
        let mut game = Game::new(board);
        let a = 0;
        let b = 1;
        let c = 2;
        let d = 3;
        let e = 4;
        let f = 5;
        let g = 6;
        let h = 7;
        let i = 8;
        let moves = vec![
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: g, y: c },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: e, y: e },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: e, y: d },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: g, y: e },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: d, y: e },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: d, y: f },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: f, y: e },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: e, y: f },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: f, y: d },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: f, y: f },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: g, y: f },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: g, y: g },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: c, y: f },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: c, y: g },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: c, y: e },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: h, y: f },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: b, y: g },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: b, y: h },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: b, y: f },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: d, y: h },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: h, y: d },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: h, y: e },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: g, y: d },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: i, y: d },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: h, y: g },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: h, y: c },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: i, y: c },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: i, y: b },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: h, y: b },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: i, y: c },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: g, y: b },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: i, y: e },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: h, y: a },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: a, y: g },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: i, y: a },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: a, y: f },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: i, y: g },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: h, y: h },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: a, y: e },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: a, y: h },
            },
            PlaceStoneMove {
                player: Player::Black,
                coord: FlexibleCoordinate { x: e, y: g },
            },
            PlaceStoneMove {
                player: Player::White,
                coord: FlexibleCoordinate { x: f, y: h },
            },
        ];

        // When
        for m in moves {
            game.make_move(&Move::PlaceStone(m))
                .expect("Expected move to be allowed");
        }

        // Then
        let e = None;
        let expected_position = vec![
            vec![e, e, e, e, e, e, e, B, B],
            vec![e, e, e, e, e, e, B, B, W],
            vec![e, e, e, e, e, e, B, W, W],
            vec![e, e, e, e, B, B, B, B, W],
            vec![B, e, B, B, W, B, W, W, W],
            vec![W, B, B, W, W, W, e, W, e],
            vec![W, B, W, e, B, e, W, B, B],
            vec![W, W, e, W, e, W, e, W, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let expected_board =
            BitMaskBoard::from_position(|| TestMask::empty((9, 9)), expected_position);

        assert_eq!(&expected_board, game.get_board());
        assert_eq!(0, game.captured_by_black);
        assert_eq!(2, game.captured_by_white);
    }
}
