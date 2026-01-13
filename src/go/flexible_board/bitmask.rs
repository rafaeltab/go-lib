use crate::go::flexible_board::coordinate::FlexibleCoordinate;

pub trait FlexibleBitMask {
    /// Get the size of the board in the form of a 1-based x, y tuple.
    fn get_size(&self) -> (u16, u16);

    /// Get the bit-value in a coordinate.
    fn get_bit_at(&self, coord: &FlexibleCoordinate) -> bool;

    /// Set the bit-value in a coordinate.
    fn set_bit_at(&mut self, coord: &FlexibleCoordinate, val: bool);
}
