#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub index: u16,
}

impl Coordinate {
    #[allow(dead_code)]
    pub fn init(x: u16, y: u16) -> Self {
        assert!(x < 19 && y < 19, "X or Y was above 19, which is impossible");
        Self {
            index: (x * 19u16) + y,
        }
    }

    pub fn from_index(index: u16) -> Self {
        debug_assert!(index < (19 * 19));
        Self { index }
    }

    pub fn to_board(self) -> (usize, usize) {
        debug_assert!(self.index < (19 * 19));
        ((self.index / 64) as usize, (self.index % 64) as usize)
    }
}
