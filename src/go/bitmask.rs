use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr},
};

use crate::utils::free_shift::{FreeLeftShift, FreeRightShift};

use crate::go::{
    bitmasks::{filled, lhs, rhs},
    coordinate::Coordinate,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct BitMask(pub [u64; 6]);

pub const LHS: BitMask = BitMask(lhs());
pub const RHS: BitMask = BitMask(rhs());
pub const FILLED: BitMask = BitMask(filled());

impl BitMask {
    pub fn init() -> Self {
        Self([0; 6])
    }
    pub fn from_coord(coord: Coordinate) -> Self {
        let mut mask = Self([0u64; 6]);
        let (board, index) = coord.to_board();
        mask.0[board] |= 1 << index;
        mask
    }

    pub fn flood_fill(&self, borders: Self) -> Self {
        let mut last_step = *self;
        let mut post_step = self.flood_fill_step(borders);

        while post_step != last_step {
            last_step = post_step;
            post_step = self.flood_fill_step(borders);
        }
        post_step
    }

    pub fn flood_fill_step(&self, borders: Self) -> Self {
        let mut out: BitMask = *self;

        out |= out << 19;
        out |= out >> 19;
        out |= out << 1 & !RHS;
        out |= out >> 1 & !LHS;

        out & !borders & FILLED
    }

    pub fn is_set(&self, coord: Coordinate) -> bool {
        let (board, index) = coord.to_board();
        (self.0[board] & 1 << index) != 0
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..6 {
            if self.0[i] != 0 {
                return false;
            }
        }
        true
    }

    pub fn count_ones(&self) -> u32 {
        let mut count = 0;
        for i in 0..6 {
            count += self.0[i].count_ones();
        }
        count
    }
}

// 00000000 00000000 00000000 00000000 00000000 00110000 << 6

// 00000000 00000000 00000000 00000000 00001100 00000000

// 00000000 00000000 00000000 00000000 00000000 00110000 << 14

// 0100
// 0000
// 0000
// 0000
//
// 0000
// 0000
// 0000
// 0000
//
// 0000 1000 << 3
// 0100 0000 >> 3
//

// 00000000 00000000 00000000 00000000 00000000 10000001
//        0        1        2        3        4        5
//        5        4        3        2        1        0

impl Shr<usize> for BitMask {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut out = self.0;
        if rhs == 0 {
            return self;
        }
        if rhs % 64 == 0 {
            let steps = rhs / 64;
            out[0..(6 - steps)].copy_from_slice(self.0[steps..6].as_ref());
        } else {
            let steps = rhs / 64;
            let range = 0..6;
            for index in range {
                if index + steps > 5 {
                    continue;
                }
                let i = index + steps;
                if i < 5 {
                    out[i] = self.0[i + 1].free_shl((64 - (rhs % 64)) as u32)
                        | self.0[i].free_shr((rhs % 64) as u32);
                } else if i == 5 {
                    out[i] = self.0[i].free_shr((rhs % 64) as u32);
                }
            }
        }
        Self(out)
    }
}

impl Shl<usize> for BitMask {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut out = [0; 6];
        if rhs == 0 {
            return Self(out);
        }

        if rhs % 64 == 0 {
            let steps = rhs / 64;
            out[steps..6].copy_from_slice(self.0[0..(6 - steps)].as_ref());
        } else {
            let steps = rhs / 64;
            let range = 0..6;
            for index in range {
                if steps > index {
                    continue;
                }
                let i = index - steps;
                if i > 0 {
                    out[i] = self.0[i].free_shl((rhs % 64) as u32)
                        | self.0[i - 1].free_shr((64 - (rhs % 64)) as u32);
                } else {
                    out[i] = self.0[i].free_shl((rhs % 64) as u32);
                }
            }
        }
        Self(out)
    }
}

impl BitOrAssign for BitMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = (*self | rhs).0
    }
}

impl BitOr for BitMask {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        for i in 0..6 {
            self.0[i] |= rhs.0[i];
        }
        self
    }
}

impl BitAndAssign for BitMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = (*self & rhs).0
    }
}

impl BitAnd for BitMask {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for i in 0..6 {
            self.0[i] &= rhs.0[i];
        }
        self
    }
}

impl Not for BitMask {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitMask(self.0.map(|f| !f))
    }
}

impl Debug for BitMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_string();
        for i in 0..19 {
            for j in 0..19 {
                let index = Coordinate::init(i, j);
                out += if self.is_set(index) { "x " } else { "_ " };
            }
            out += "\n";
        }
        f.write_str(out.as_str())
    }
}

#[test]
pub fn leftshift_test_1() {
    let board = BitMask([1, 0, 0, 0, 0, 0]);
    let bitshifted_board = board << 64;
    assert_eq!(board.0[0], bitshifted_board.0[1]);
}

#[test]
pub fn leftshift_test_2() {
    let board = BitMask([1, 0, 0, 0, 0, 0]);
    let bitshifted_board = board << (64 * 3);
    assert_eq!(board.0[0], bitshifted_board.0[3]);
}

#[test]
pub fn leftshift_test_3() {
    let board = BitMask([1, 0, 0, 0, 0, 0]);
    let bitshifted_board = board << 2;
    assert_eq!(board.0[0] << 2, bitshifted_board.0[0]);
}

#[test]
pub fn leftshift_test_4() {
    let board = BitMask([1 << 63, 0, 0, 0, 0, 0]);
    let bitshifted_board = board << 1;
    let expected_board = BitMask([0, 1, 0, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}

#[test]
pub fn leftshift_test_5() {
    let board = BitMask([1 << 60, 0, 0, 0, 0, 0]);
    let bitshifted_board = board << 4;
    let expected_board = BitMask([0, 1, 0, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}

#[test]
pub fn leftshift_test_6() {
    let board = BitMask([1 << 60, 0, 0, 0, 0, 0]);
    let bitshifted_board = board << (4 + 64);
    let expected_board = BitMask([0, 0, 1, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}

#[test]
pub fn rightshift_test_1() {
    let board = BitMask([0, 1, 0, 0, 0, 0]);
    let bitshifted_board = board >> 64;
    assert_eq!(board.0[1], bitshifted_board.0[0]);
}

#[test]
pub fn rightshift_test_2() {
    let board = BitMask([0, 0, 0, 1, 0, 0]);
    let bitshifted_board = board >> (64 * 3);
    assert_eq!(board.0[3], bitshifted_board.0[0]);
}

#[test]
pub fn rightshift_test_3() {
    let board = BitMask([0b100, 0, 0, 0, 0, 0]);
    let bitshifted_board = board >> 2;
    let expected_board = BitMask([1, 0, 0, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}

#[test]
pub fn rightshift_test_4() {
    let board = BitMask([0, 1, 0, 0, 0, 0]);
    let bitshifted_board = board >> 1;
    let expected_board = BitMask([1 << 63, 0, 0, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}

#[test]
pub fn rightshift_test_5() {
    let board = BitMask([0, 1 << 3, 0, 0, 0, 0]);
    let bitshifted_board = board >> 4;
    let expected_board = BitMask([1 << 63, 0, 0, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}

#[test]
pub fn rightshift_test_6() {
    let board = BitMask([0, 0, 1 << 3, 0, 0, 0]);
    let bitshifted_board = board >> (4 + 64);
    let expected_board = BitMask([1 << 63, 0, 0, 0, 0, 0]);
    assert_eq!(bitshifted_board, expected_board);
}
