use crate::go::{
    flexible_board::{board::FlexibleBoard, coordinate::FlexibleCoordinate, coordinate_set::CoordinateSet},
    player::Player,
};

struct Game<TBoard: FlexibleBoard> {
    board: TBoard,
    captured_black: u16,
    captured_white: u16,
    current_player: Player,
}

impl<TBoard: FlexibleBoard> Game<TBoard> {
    pub fn make_move(&mut self, coord: &FlexibleCoordinate) -> Result<(), MoveError> {
        let occupying_player = self.board.get_player_at(coord);
        if let Some(occupied_by) = occupying_player {
            return Err(MoveError::CoordinateOccupied { occupied_by });
        }

        Ok(())
    }

}

pub enum MoveError {
    CoordinateOccupied { occupied_by: Player },
    Suicide,
}

