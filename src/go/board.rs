use std::{collections::HashSet, fmt::Display};

use crate::go::{
    coordinate::FlexibleCoordinate, coordinate_set::CoordinateSet, group::Group, player::Player,
    playermove::PlaceStoneMove,
};
use thiserror::{self, Error};

pub trait FlexibleBoard: Sized {
    /// Get the size of the board in the form of a 1-based x, y tuple.
    fn get_size(&self) -> (u16, u16);

    /// Get the player placed in a specific position, or None if no player is set.
    fn get_player_at(&self, coord: &FlexibleCoordinate) -> Option<Player>;

    /// Set the player in a specific position.
    fn set_player_at(
        &mut self,
        coord: &FlexibleCoordinate,
        player: &Player,
    ) -> Result<(), BoardPlacementError>;

    fn clear_at(&mut self, coord: &FlexibleCoordinate) -> Result<(), BoardClearError>;

    fn find_group(&self, coord: &FlexibleCoordinate) -> Option<Group>;

    fn predict_group(&self, m: &PlaceStoneMove) -> Group;

    fn display(&self) -> DisplayFlexibleboard<'_, Self> {
        DisplayFlexibleboard(self)
    }

    fn get_liberties(&self, group: &Group) -> CoordinateSet {
        let grown = group.coordinates.grow(self.get_size());
        let possible_liberties = grown.subtract(&group.coordinates);
        let liberties: HashSet<FlexibleCoordinate> = possible_liberties
            .into_iter()
            .filter(|x| self.get_player_at(x).is_none())
            .collect();

        CoordinateSet::from_set(liberties)
    }

    fn capture(&mut self, coords: &CoordinateSet) -> Result<u16, BoardClearError> {
        for coord in coords.iter() {
            self.clear_at(coord)?;
        }
        Ok(coords.len())
    }

    fn is_potential_suicide(&self, m: &PlaceStoneMove) -> bool {
        let potential_group = self.predict_group(m);
        let liberties = self.get_liberties(&potential_group);
        liberties.is_empty()
    }

    fn find_groups_to_capture_from_move(&self, m: &PlaceStoneMove) -> Vec<Group> {
        let PlaceStoneMove { player, coord } = m;
        let mut res = vec![];
        let opponent = !*player;
        let mut neighbours = CoordinateSet::new(vec![*coord]).grow(self.get_size());
        neighbours.remove(coord);

        for neighbour in neighbours.into_iter() {
            if !neighbour.is_in_board(self) || self.get_player_at(&neighbour) != Some(opponent) {
                continue;
            }

            let group = self
                .find_group(&neighbour)
                .expect("Should find a group if there is at least one opponent stone there");
            let mut liberties = self.get_liberties(&group);
            liberties.remove(coord);
            if liberties.is_empty() {
                res.push(group);
            }
        }

        res
    }
}

pub struct DisplayFlexibleboard<'a, T: FlexibleBoard>(pub &'a T);

impl<T: FlexibleBoard> Display for DisplayFlexibleboard<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.get_size();
        for y in 0..size.1 {
            for x in 0..size.0 {
                match self.0.get_player_at(&FlexibleCoordinate { x, y }) {
                    None => write!(f, "  "),
                    Some(Player::Black) => write!(f, "⚫"),
                    Some(Player::White) => write!(f, "⚪"),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum BoardPlacementError {
    #[error("Position was already occupied on the board by {occupied_by}")]
    CoordinateOccupied { occupied_by: Player },
}

#[derive(Debug, Error)]
pub enum BoardClearError {
    #[error("Position has no players occupying it")]
    CoordinateEmpty,
}

#[cfg(test)]
mod tests {
    use crate::go::{
        bitmask::TestMask,
        bitmask_board::BitMaskBoard,
        player::{B, W},
    };

    use super::*;

    #[test]
    fn given_a_board_when_get_liberties_is_called_then_it_should_exclude_opponent_pieces() {
        // Given
        let e = None;
        let position = vec![
            vec![B, W, B, e, e, e, e, e, e],
            vec![B, e, B, e, e, e, e, e, e],
            vec![B, B, B, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let board = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), position);

        // When
        let white_group = board
            .find_group(&FlexibleCoordinate { x: 1, y: 0 })
            .expect("Expected group to be found");

        println!("{:?}", white_group);

        let res = board.get_liberties(&white_group);

        println!("{:?}", res);

        // Then
        let expected = CoordinateSet::set(&[(1, 1)]);
        assert!(res.equals(&expected));
    }

    #[test]
    fn given_a_board_when_get_liberties_is_called_then_it_should_exclude_opponent_pieces_two() {
        // Given
        let e = None;
        let position = vec![
            vec![B, W, B, e, e, e, e, e, e],
            vec![B, e, B, e, e, e, e, e, e],
            vec![B, B, B, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let board = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), position);

        // When
        let black_group = board
            .find_group(&FlexibleCoordinate { x: 2, y: 0 })
            .expect("Expected group to be found");

        println!("{:?}", black_group);

        let res = board.get_liberties(&black_group);

        println!("{:?}", res);

        // Then
        let expected =
            CoordinateSet::set(&[(3, 0), (3, 1), (3, 2), (0, 3), (1, 3), (2, 3), (1, 1)]);
        assert!(res.equals(&expected));
    }

    #[test]
    fn given_a_board_when_capture_is_called_then_it_should_remove_the_stones() {
        // Given
        let e = None;
        let position = vec![
            vec![B, W, B, e, e, e, e, e, e],
            vec![B, e, B, e, e, e, e, e, e],
            vec![B, B, B, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let mut board = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), position);

        // When
        let black_group = board
            .find_group(&FlexibleCoordinate { x: 2, y: 0 })
            .expect("Expected group to be found");

        println!("{:?}", black_group);

        board
            .capture(&black_group.coordinates)
            .expect("Should be able to capture stones");

        // Then
        let mut expected_position = vec![vec![e; 9]; 9];
        expected_position[0][1] = W;
        let expected = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), expected_position);
        assert_eq!(expected, board);
    }

    #[test]
    fn given_a_board_when_find_groups_to_capture_from_move_is_called_then_it_should_return_all_groups_that_will_have_no_liberties()
     {
        // Given
        let e = None;
        let position = vec![
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, W, e, e, e, e],
            vec![e, e, W, W, B, W, e, e, e],
            vec![e, W, B, B, e, B, W, e, e],
            vec![e, e, W, W, B, W, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let board = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), position);

        // When
        let capture_groups = board.find_groups_to_capture_from_move(&PlaceStoneMove {
            player: Player::White,
            coord: FlexibleCoordinate { x: 4, y: 4 },
        });

        println!("{:?}", capture_groups);

        // Then
        assert_eq!(3, capture_groups.len());

        let expected_groups = vec![
            CoordinateSet::set(&[(4, 3)]),
            CoordinateSet::set(&[(5, 4)]),
            CoordinateSet::set(&[(2, 4), (3, 4)]),
        ];

        for expected_group in expected_groups {
            let exists: Vec<&Group> = capture_groups
                .iter()
                .filter(|x| x.coordinates.equals(&expected_group))
                .collect();
            assert_eq!(1, exists.len());
        }
    }

    #[test]
    fn given_a_suicide_move_when_is_potential_suicide_is_called_then_it_should_return_true() {
        // Given
        let e = None;
        let position = vec![
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, B, e, e, e, e],
            vec![e, e, e, B, e, B, e, e, e],
            vec![e, e, e, e, B, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e],
        ];
        let board = BitMaskBoard::from_position(|| TestMask::empty((9, 9)), position);

        // When
        let res = board.is_potential_suicide(&PlaceStoneMove {
            player: Player::White,
            coord: FlexibleCoordinate { x: 4, y: 4 },
        });

        // Then
        assert!(res);
    }
}
