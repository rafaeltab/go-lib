use crate::go::{
    board::FlexibleBoard,
    player::Player,
    playermove::{Move, PlaceStoneMove},
};

struct Game<TBoard: FlexibleBoard> {
    board: TBoard,
    captured_black: u16,
    captured_white: u16,
    current_player: Player,
}

impl<TBoard: FlexibleBoard> Game<TBoard> {
    pub fn new(board: TBoard) -> Self {
        Game {
            board,
            captured_black: 0,
            captured_white: 0,
            current_player: Player::Black,
        }
    }

    pub fn make_move(&mut self, m: Move) -> Result<(), MoveError> {
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

                for group in groups_to_capture {
                    self.board
                        .capture(&group.coordinates)
                        .expect("Expected capture to work");
                }

                self.board
                    .set_player_at(&coord, &player)
                    .expect("Already checked whether spot is occupied or not");

                Ok(())
            }
            Move::Skip { .. } => todo!(),
        }
    }

    pub fn get_board(&self) -> &TBoard {
        &self.board
    }
}

pub enum MoveError {
    CoordinateOccupied { occupied_by: Player },
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
        let res = game.make_move(Move::PlaceStone(PlaceStoneMove {
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
        let res = game.make_move(Move::PlaceStone(PlaceStoneMove {
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
    }
}
