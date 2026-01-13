use crate::go::flexible_board::{bitmask::FlexibleBitMask, board::FlexibleBoard};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct FlexibleCoordinate {
    /// 0-based x-axis position
    pub x: u16,
    /// 0-based y-axis position.
    pub y: u16,
}

impl FlexibleCoordinate {
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Self {
            x: self.x,
            y: self.y - 1,
        })
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self {
            x: self.x - 1,
            y: self.y,
        })
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn is_in_mask<TMask: FlexibleBitMask>(&self, mask: &TMask) -> bool {
        let size = mask.get_size();
        if self.x >= size.0 {
            return false;
        }
        if self.y >= size.1 {
            return false;
        }
        true
    }

    pub fn is_in_board<TBoard: FlexibleBoard>(&self, board: &TBoard) -> bool {
        let size = board.get_size();
        if self.x >= size.0 {
            return false;
        }
        if self.y >= size.1 {
            return false;
        }
        true
    }
}
