use crate::go::{coordinate_set::CoordinateSet, player::Player};

#[derive(Debug)]
pub struct Group {
    pub player: Player,
    pub coordinates: CoordinateSet,
}
