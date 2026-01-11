#![allow(dead_code)] // library code does not need to be explicitly used to be useful
mod go;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        go::board::Board::init();
    }
}
