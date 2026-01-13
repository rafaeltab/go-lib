use std::collections::VecDeque;

use crate::go::flexible_board::{coordinate::FlexibleCoordinate, coordinate_set::CoordinateSet};

pub trait FlexibleBitMask: Sized {
    /// Get the size of the board in the form of a 1-based x, y tuple.
    fn get_size(&self) -> (u16, u16);

    /// Get the bit-value in a coordinate.
    fn get_bit_at(&self, coord: &FlexibleCoordinate) -> bool;

    /// Set the bit-value in a coordinate.
    fn set_bit_at(&mut self, coord: &FlexibleCoordinate, val: bool);

    fn flood_fill(&self, coord: FlexibleCoordinate) -> CoordinateSet {
        let mut queue = VecDeque::new();
        queue.push_back(coord);
        let mut res = CoordinateSet::new(vec![coord]);

        while let Some(next) = queue.pop_front() {
            let top = next.up();

            if top.is_in_mask(self) && self.get_bit_at(&top) && !res.contains(&top) {
                queue.push_back(top);
                res.insert(top);
            }

            let maybe_bottom = next.down();

            if let Some(bottom) = maybe_bottom
                && bottom.is_in_mask(self)
                && self.get_bit_at(&bottom)
                && !res.contains(&bottom)
            {
                queue.push_back(bottom);
                res.insert(bottom);
            }

            let right = next.right();

            if right.is_in_mask(self) && self.get_bit_at(&right) && !res.contains(&right) {
                queue.push_back(right);
                res.insert(right);
            }

            let maybe_left = next.left();

            if let Some(left) = maybe_left
                && left.is_in_mask(self)
                && self.get_bit_at(&left)
                && !res.contains(&left)
            {
                queue.push_back(left);
                res.insert(left);
            }
        }

        res
    }
}

pub struct TestMask {
    width: u16,
    height: u16,
    vals: Vec<Vec<bool>>,
}

impl TestMask {
    pub fn empty(size: (u16, u16)) -> Self {
        TestMask {
            width: size.0,
            height: size.1,
            vals: vec![vec![false; size.0 as usize]; size.1 as usize],
        }
    }

    pub fn new(vals: Vec<Vec<bool>>) -> Self {
        TestMask {
            width: vals[0].len() as u16,
            height: vals.len() as u16,
            vals,
        }
    }
}

impl FlexibleBitMask for TestMask {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_bit_at(&self, coord: &FlexibleCoordinate) -> bool {
        self.vals[coord.y as usize][coord.x as usize]
    }

    fn set_bit_at(&mut self, coord: &FlexibleCoordinate, val: bool) {
        self.vals[coord.y as usize][coord.x as usize] = val;
    }
}

#[test]
fn given_group_when_flood_fill_is_called_then_it_should_select_whole_group() {
    // Given
    let mask = TestMask::new(vec![
        vec![false, false, false, false, false],
        vec![false, true, true, true, false],
        vec![false, false, true, false, false],
        vec![false, true, true, false, false],
        vec![false, false, false, false, false],
    ]);

    // When
    let res = mask.flood_fill(FlexibleCoordinate { x: 1, y: 1 });

    // Then
    let expected = CoordinateSet::set(&[(1, 1), (1, 3), (2, 1), (2, 2), (2, 3), (3, 1)]);

    println!("{:?}", res);
    println!("{:?}", expected);

    assert!(res.equals(&expected));
}
