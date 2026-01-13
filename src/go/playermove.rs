use crate::go::{coordinate::FlexibleCoordinate, player::Player};

pub enum Move {
    PlaceStone(PlaceStoneMove),
    Skip { player: Player },
}

pub struct PlaceStoneMove {
    pub player: Player,
    pub coord: FlexibleCoordinate,
}
