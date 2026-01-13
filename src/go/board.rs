use std::collections::HashSet;

use crate::go::{
    coordinate::FlexibleCoordinate, coordinate_set::CoordinateSet, group::Group, player::Player,
};
use thiserror::{self, Error};

pub trait FlexibleBoard {
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

    fn get_liberties(&self, group: Group) -> CoordinateSet {
        let grown = group.coordinates.grow(self.get_size());
        let possible_liberties = grown.subtract(&group.coordinates);
        let liberties: HashSet<FlexibleCoordinate> = possible_liberties
            .into_iter()
            .filter(|x| self.get_player_at(x).is_none())
            .collect();

        CoordinateSet::from_set(liberties)
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

        let res = board.get_liberties(white_group);

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
        let white_group = board
            .find_group(&FlexibleCoordinate { x: 2, y: 0 })
            .expect("Expected group to be found");

        println!("{:?}", white_group);

        let res = board.get_liberties(white_group);

        println!("{:?}", res);

        // Then
        let expected =
            CoordinateSet::set(&[(3, 0), (3, 1), (3, 2), (0, 3), (1, 3), (2, 3), (1, 1)]);
        assert!(res.equals(&expected));
    }
}
