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

    #[test]
    fn perft_nowall() {
        use crate::board::Board;

        let mut board = Board::default();
        board.set_walls_in_pocket(0, 0);

        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });

        assert_eq!(count, 3, "perft(1) = {count}");
    }

    #[test]
    fn perft_midboard_nowall() {
        use crate::board::Board;
        use crate::types::Square;
        use crate::types::Move;

        let mut board = Board::default();
        board.set_walls_in_pocket(0, 0);
        board.make_move(Move::Pawn {
            to_square: Square::from_file_rank(4, 1).unwrap(),
        });
        board.make_move(Move::Pawn {
            to_square: Square::from_file_rank(4, 8).unwrap(),
        });

        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });

        assert_eq!(count, 4, "perft(1) = {count}");
    }
}
