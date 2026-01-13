use std::collections::HashSet;

use crate::go::flexible_board::coordinate::FlexibleCoordinate;

#[derive(Debug)]
pub struct CoordinateSet(HashSet<FlexibleCoordinate>);

impl CoordinateSet {
    pub fn from_set(set: HashSet<FlexibleCoordinate>) -> Self {
        CoordinateSet(set)
    }

    pub fn new(coords: Vec<FlexibleCoordinate>) -> CoordinateSet {
        CoordinateSet(coords.into_iter().collect())
    }

    pub fn iter(&self) -> impl Iterator<Item = &FlexibleCoordinate> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = FlexibleCoordinate> {
        self.0.into_iter()
    }

    pub fn remove(&mut self, coord: &FlexibleCoordinate) -> bool {
        self.0.remove(coord)
    }

    pub fn set(coords: &[(u16, u16)]) -> CoordinateSet {
        CoordinateSet(
            coords
                .iter()
                .map(|&(x, y)| FlexibleCoordinate { x, y })
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn insert(&mut self, coord: FlexibleCoordinate) -> bool {
        self.0.insert(coord)
    }

    pub fn contains(&self, coord: &FlexibleCoordinate) -> bool {
        self.0.contains(coord)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.subtract(other).is_empty() && other.subtract(self).is_empty()
    }

    pub fn subtract(&self, other: &Self) -> Self {
        let mut new = self.0.clone();

        for val in &other.0 {
            new.remove(val);
        }

        Self(new)
    }

    pub fn grow(&self, board_size: (u16, u16)) -> Self {
        let (width, height) = board_size;

        let mut result: HashSet<FlexibleCoordinate> = HashSet::new();

        for coord in &self.0 {
            // Always keep the original coordinate
            result.insert(FlexibleCoordinate {
                x: coord.x,
                y: coord.y,
            });

            // Left
            if coord.x > 0 {
                result.insert(FlexibleCoordinate {
                    x: coord.x - 1,
                    y: coord.y,
                });
            }

            // Right
            if coord.x + 1 < width {
                result.insert(FlexibleCoordinate {
                    x: coord.x + 1,
                    y: coord.y,
                });
            }

            // Up
            if coord.y > 0 {
                result.insert(FlexibleCoordinate {
                    x: coord.x,
                    y: coord.y - 1,
                });
            }

            // Down
            if coord.y + 1 < height {
                result.insert(FlexibleCoordinate {
                    x: coord.x,
                    y: coord.y + 1,
                });
            }
        }

        Self(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grow_single_coordinate_center() {
        let input = CoordinateSet::set(&[(5, 5)]);
        let result = input.grow((9, 9));

        let expected = CoordinateSet::set(&[(5, 5), (4, 5), (6, 5), (5, 4), (5, 6)]);

        assert_eq!(result.0, expected.0);
    }

    #[test]
    fn grow_single_coordinate_corner() {
        let input = CoordinateSet::set(&[(0, 0)]);
        let result = input.grow((9, 9));

        let expected = CoordinateSet::set(&[(0, 0), (1, 0), (0, 1)]);

        assert_eq!(result.0, expected.0);
    }

    #[test]
    fn grow_single_coordinate_edge() {
        let input = CoordinateSet::set(&[(0, 5)]);
        let result = input.grow((9, 9));

        let expected = CoordinateSet::set(&[(0, 5), (1, 5), (0, 4), (0, 6)]);

        assert_eq!(result.0, expected.0);
    }

    #[test]
    fn grow_twice_produces_larger_set() {
        let input = CoordinateSet::set(&[(5, 5)]);
        let once = input.grow((9, 9));
        let twice = once.grow((9, 9));

        // Diamond of radius 2 has 13 cells
        assert_eq!(twice.0.len(), 13);
    }

    #[test]
    fn grow_respects_board_bounds() {
        let input = CoordinateSet::set(&[(0, 0)]);
        let result = input.grow((2, 2));

        let expected = CoordinateSet::set(&[(0, 0), (1, 0), (0, 1)]);

        assert_eq!(result.0, expected.0);
    }

    #[test]
    fn subtract_removes_values_in_other_from_self() {
        let this = CoordinateSet::set(&[(0, 0), (0, 1), (0, 2), (0, 3)]);
        let other = CoordinateSet::set(&[(0, 0), (0, 3)]);

        let result = this.subtract(&other);

        let expected = CoordinateSet::set(&[(0, 1), (0, 2)]);

        assert_eq!(result.0, expected.0);
    }
}
