#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
//! A library for the game "Quoridor".

pub mod board;
mod types;
mod squareset;

#[cfg(test)]
mod tests {
    #[test]
    fn perft_depth_one() {
        use crate::board::Board;
        const PAWN_MOVES: usize = 3;
        const HORIZONTAL_WALL_MOVES: usize = 8 * 8;
        const VERTICAL_WALL_MOVES: usize = 8 * 8;
        const MOVES: usize = PAWN_MOVES + HORIZONTAL_WALL_MOVES + VERTICAL_WALL_MOVES;

        let board = Board::default();
        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });

        assert_eq!(count, MOVES, "perft(1) = {count}");
    }
}
