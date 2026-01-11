use crate::go::bitmask::{BitMask, FILLED};
use crate::go::{coordinate::Coordinate, player::Player};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    board: [BitMask; 2],
}

impl Board {
    pub fn init() -> Self {
        Self {
            board: [BitMask([0; 6]); 2],
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.board == [BitMask([0; 6]); 2]
    }

    pub fn is_free(&self, coord: Coordinate) -> bool {
        let (board, place) = coord.to_board();

        (self.board[0].0[board] | self.board[1].0[board]) & 1 << place == 0
    }

    pub fn with_move(&self, coord: Coordinate, player: Player) -> Self {
        let (board, place) = coord.to_board();
        let mut new_board = *self;
        new_board.board[player as usize].0[board] |= 1 << place;
        new_board
    }

    #[allow(dead_code)]
    pub fn is_valid_board(&self) -> bool {
        let has_overlapping_fields = !(self.board[0] & self.board[1]).is_empty();

        let has_bits_out_of_bounds = ((self.board[0] | self.board[1]) & FILLED).is_empty();

        !(has_overlapping_fields || has_bits_out_of_bounds)
    }

    pub fn can_place_at(&self, place: Coordinate, current_player: Player) -> bool {
        // Flood fill
        // 1. find the chain this would be part of
        // 2. check if that chain has air
        // 3. check if any neighboring chains now are airless
        let place_mask = BitMask::from_coord(place);
        let group = place_mask.flood_fill(!self.board[current_player as usize]);

        if group.flood_fill_step(self.board[!current_player as usize]) == group {
            // it borders air already
            return true;
        }
        let neighbor_enemy_chains = group.flood_fill(self.board[current_player as usize]);
        if neighbor_enemy_chains.flood_fill_step(self.board[0] | self.board[1])
            == neighbor_enemy_chains
        {
            // enemies will be capture
            return true;
        }

        false
    }

    pub fn clean_taken_pieces(&mut self, current_player: Player) -> (u32, u32) {
        let mut taken_current = 0u32;
        let mut taken_other = 0u32;

        let both = self.board[0] | self.board[1];

        let mut skip_mask = !both;

        for i in 0..(19 * 19) {
            let coord = Coordinate::from_index(i);
            if skip_mask.is_set(coord) {
                continue;
            }
            if self.board[!current_player as usize].is_set(coord) {
                let group =
                    BitMask::from_coord(coord).flood_fill(!self.board[!current_player as usize]);
                if group.flood_fill_step(self.board[current_player as usize]) == group {
                    self.board[current_player as usize] &= !group;
                    skip_mask |= group;
                    taken_current += group.count_ones();
                }
            }
        }

        for i in 0..(19 * 19) {
            let coord = Coordinate::from_index(i);
            if skip_mask.is_set(coord) {
                continue;
            }
            if self.board[current_player as usize].is_set(coord) {
                let group =
                    BitMask::from_coord(coord).flood_fill(!self.board[current_player as usize]);
                if group.flood_fill_step(self.board[!current_player as usize]) == group {
                    self.board[!current_player as usize] &= !group;
                    skip_mask |= group;
                    taken_other += group.count_ones();
                }
            }
        }

        match current_player {
            Player::Black => (taken_current, taken_other),
            Player::White => (taken_other, taken_current),
        }
    }

    pub fn get_piece_at(&self, coord: Coordinate) -> Option<Player> {
        if self.board[0].is_set(coord) {
            return Some(Player::Black);
        } else if self.board[1].is_set(coord) {
            return Some(Player::White);
        }
        None
    }
}
