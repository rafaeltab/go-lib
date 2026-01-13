use crate::go::{flexible_board::coordinate::FlexibleCoordinate, player::Player};
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
