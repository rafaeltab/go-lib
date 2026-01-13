use std::{fmt::Display, ops::Not};

pub const B: Option<Player> = Some(Player::Black);
pub const W: Option<Player> = Some(Player::White);

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum Player {
    Black,
    White,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Black => write!(f, "black"),
            Player::White => write!(f, "white"),
        }
    }
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
