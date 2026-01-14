use std::fmt::{Debug, Display};

use crate::go::{
    bitmask::FlexibleBitMask,
    board::{BoardClearError, BoardPlacementError, FlexibleBoard},
    coordinate::FlexibleCoordinate,
    group::Group,
    player::Player,
    playermove::PlaceStoneMove,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BitMaskBoard<TBitMask: FlexibleBitMask + Eq + PartialEq + Debug + Clone> {
    width: u16,
    height: u16,
    black_mask: TBitMask,
    white_mask: TBitMask,
}

impl<TBitMask: FlexibleBitMask + Eq + PartialEq + Debug + Clone> BitMaskBoard<TBitMask> {
    pub fn new<TMaskFactory: Fn() -> TBitMask>(mask_factory: TMaskFactory) -> Self {
        let white_mask = mask_factory();
        let black_mask = mask_factory();
        let size = white_mask.get_size();

        assert_eq!(size, black_mask.get_size());
        Self {
            width: size.0,
            height: size.1,
            white_mask,
            black_mask,
        }
    }

    pub fn from_position<TMaskFactory: Fn() -> TBitMask>(
        mask_factory: TMaskFactory,
        position: Vec<Vec<Option<Player>>>,
    ) -> Self {
        let mut board = Self::new(mask_factory);

        assert_eq!(board.width, position.len() as u16);
        assert_eq!(board.height, position[0].len() as u16);

        for y in 0..board.width {
            let vals = &position[y as usize];
            for x in 0..board.height {
                let val = vals[x as usize];
                if let Some(player) = val {
                    board
                        .set_player_at(&FlexibleCoordinate { x, y }, &player)
                        .expect("Incorrect implementation of from_position method");
                }
            }
        }

        board
    }
}

impl<TBitMask: FlexibleBitMask + PartialEq + Eq + Debug + Clone> FlexibleBoard
    for BitMaskBoard<TBitMask>
{
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_player_at(&self, coord: &FlexibleCoordinate) -> Option<Player> {
        if self.black_mask.get_bit_at(coord) {
            return Some(Player::Black);
        } else if self.white_mask.get_bit_at(coord) {
            return Some(Player::White);
        }
        None
    }

    fn set_player_at(
        &mut self,
        coord: &FlexibleCoordinate,
        player: &Player,
    ) -> Result<(), BoardPlacementError> {
        let occupied_by = self.get_player_at(coord);
        if let Some(occupying_player) = occupied_by {
            return Err(BoardPlacementError::CoordinateOccupied {
                occupied_by: occupying_player,
            });
        }

        match player {
            Player::Black => self.black_mask.set_bit_at(coord, true),
            Player::White => self.white_mask.set_bit_at(coord, true),
        }

        Ok(())
    }

    fn clear_at(
        &mut self,
        coord: &FlexibleCoordinate,
    ) -> Result<(), super::board::BoardClearError> {
        let occupied_by = self.get_player_at(coord);
        if let Some(occupying_player) = occupied_by {
            match occupying_player {
                Player::Black => self.black_mask.set_bit_at(coord, false),
                Player::White => self.white_mask.set_bit_at(coord, false),
            };
            return Ok(());
        }

        Err(BoardClearError::CoordinateEmpty)
    }

    fn find_group(&self, coord: &FlexibleCoordinate) -> Option<Group> {
        let player = self.get_player_at(coord)?;
        let m = PlaceStoneMove {
            player,
            coord: *coord,
        };
        Some(self.predict_group(&m))
    }

    fn predict_group(&self, m: &PlaceStoneMove) -> Group {
        let coordinates = match m.player {
            Player::Black => &self.black_mask,
            Player::White => &self.white_mask,
        }
        .flood_fill(m.coord);

        Group {
            player: m.player,
            coordinates,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::{
        bitmask19::BitMask19,
        board::FlexibleBoard,
        coordinate::FlexibleCoordinate,
        player::{B, W},
    };

    #[test]
    fn given_empty_board_when_get_player_at_is_called_then_returns_none() {
        // Given
        let board = BitMaskBoard::new(BitMask19::init);

        // When
        let res = board.get_player_at(&FlexibleCoordinate { x: 0, y: 0 });

        // Then
        assert_eq!(None, res);
    }

    #[test]
    fn given_position_set_to_black_when_get_player_at_is_called_then_returns_black() {
        // Given
        let e = None;
        let position = vec![
            vec![B, W, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
        ];

        let board = BitMaskBoard::from_position(BitMask19::init, position);

        // When
        let res = board.get_player_at(&FlexibleCoordinate { x: 0, y: 0 });

        // Then
        assert_eq!(B, res);
    }

    #[test]
    fn given_position_set_to_white_when_get_player_at_is_called_then_returns_white() {
        // Given
        let e = None;
        let position = vec![
            vec![B, W, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
            vec![e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e, e],
        ];

        let board = BitMaskBoard::from_position(BitMask19::init, position);

        // When
        let res = board.get_player_at(&FlexibleCoordinate { x: 1, y: 0 });

        // Then
        assert_eq!(W, res);
    }
}
