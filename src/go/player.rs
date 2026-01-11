use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum Player {
    Black,
    White,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
