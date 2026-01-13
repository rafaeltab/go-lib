use crate::go::{bitmask::FlexibleBitMask, coordinate::FlexibleCoordinate};

pub struct BitMask19(pub [u64; 6]);

impl FlexibleBitMask for BitMask19 {
    fn get_size(&self) -> (u16, u16) {
        (19, 19)
    }

    fn get_bit_at(&self, coord: &super::coordinate::FlexibleCoordinate) -> bool {
        let (arr_index, int_index) = BitMask19::get_bit_position(coord);
        (self.0[arr_index] & 1 << int_index) != 0
    }

    fn set_bit_at(&mut self, coord: &super::coordinate::FlexibleCoordinate, val: bool) {
        let (arr_index, int_index) = BitMask19::get_bit_position(coord);
        if val {
            self.0[arr_index] |= 1 << int_index;
        } else {
            self.0[arr_index] ^= 1 << int_index;
        }
    }
}

impl BitMask19 {
    pub fn init() -> Self {
        Self([0; 6])
    }

    pub fn filled() -> Self {
        Self([u64::MAX; 6])
    }

    fn get_bit_position(coord: &FlexibleCoordinate) -> (usize, usize) {
        let index = (coord.x * 19u16) + coord.y;

        ((index / 64) as usize, (index % 64) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_bitmark_when_get_bit_at_is_called_then_returns_false() {
        // Given
        let board = BitMask19::init();

        // When
        let res = board.get_bit_at(&FlexibleCoordinate { x: 0, y: 0 });

        // Then
        assert!(!res);
    }

    #[test]
    fn given_filled_board_when_get_bit_at_is_called_then_returns_true() {
        // Given
        let board = BitMask19::filled();

        // When
        let res = board.get_bit_at(&FlexibleCoordinate { x: 0, y: 0 });

        // Then
        assert!(res);
    }

    #[test]
    fn given_empty_board_when_bit_is_set_then_retrieved_then_returns_true() {
        // Given
        let mut board = BitMask19::init();
        let coord = FlexibleCoordinate { x: 0, y: 0 };

        // When
        board.set_bit_at(&coord, true);
        let res = board.get_bit_at(&coord);

        // Then
        assert!(res);
    }

    #[test]
    fn given_filled_board_when_bit_is_unset_then_retrieved_then_returns_false() {
        // Given
        let mut board = BitMask19::filled();
        let coord = FlexibleCoordinate { x: 0, y: 0 };

        // When
        board.set_bit_at(&coord, false);
        let res = board.get_bit_at(&coord);

        // Then
        assert!(!res);
    }
}
