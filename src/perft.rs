use std::{collections::HashMap, hash::BuildHasher};

use crate::board::Board;

#[must_use]
pub fn perft(board: Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    if depth == 1 {
        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });
        return count;
    }

    let mut count = 0;
    board.generate_moves(|mv| {
        let mut board = board;
        board.make_move(mv);
        count += perft(board, depth - 1);
        false
    });

    count
}

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn perft_cached<S: BuildHasher>(
    board: Board,
    depth: u8,
    cache: &mut HashMap<(Board, u8), u64, S>,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    if depth == 1 {
        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });
        return count;
    }

    if let Some(&count) = cache.get(&(board, depth)) {
        return count;
    }

    let mut count = 0;
    board.generate_moves(|mv| {
        let mut board = board;
        board.make_move(mv);
        count += perft_cached(board, depth - 1, cache);
        false
    });

    cache.insert((board, depth), count);

    count
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr};

    #[test]
    fn perft_depth_one() {
        use crate::board::Board;
        const PAWN_MOVES: u64 = 3;
        const HORIZONTAL_WALL_MOVES: u64 = 8 * 8;
        const VERTICAL_WALL_MOVES: u64 = 8 * 8;
        const MOVES: u64 = PAWN_MOVES + HORIZONTAL_WALL_MOVES + VERTICAL_WALL_MOVES;

        let board = Board::default();

        let count = super::perft(board, 1);

        assert_eq!(count, MOVES, "perft(1) = {count}");
    }

    #[test]
    fn perft_depth_two() {
        use crate::board::Board;
        const PAWN_MOVES: u64 = 3;
        const HORIZONTAL_WALL_MOVES: u64 = 8 * 8;
        const VERTICAL_WALL_MOVES: u64 = 8 * 8;
        const MOVES: u64 = PAWN_MOVES + HORIZONTAL_WALL_MOVES + VERTICAL_WALL_MOVES;
        // there are 32 edge locations to place walls, those only block 3 moves each
        const THREE_BLOCKED_WALL_MOVES: u64 = 8 + 8 + 8 + 8;
        // there are 96 interior locations to place walls, those block 4 moves each
        const FOUR_BLOCKED_WALL_MOVES: u64 = 128 - THREE_BLOCKED_WALL_MOVES;

        const D2_MOVES_NO_PAWN_BLOCK: u64 = PAWN_MOVES * MOVES
            + THREE_BLOCKED_WALL_MOVES * (MOVES - 3)
            + FOUR_BLOCKED_WALL_MOVES * (MOVES - 4);
        // there are four wall moves that block pawn moves
        const D2_MOVES: u64 = D2_MOVES_NO_PAWN_BLOCK - 4;

        let board = Board::default();

        let count = super::perft(board, 2);

        assert_eq!(count, D2_MOVES, "perft(2) = {count}");
    }

    #[test]
    fn perft_depth_one_move_names() {
        use crate::board::Board;

        let board = Board::default();

        let mut moves = HashSet::new();
        board.generate_moves(|mv| {
            moves.insert(mv.to_string());
            false
        });

        let mut reference_movelist = HashSet::new();
        for pawn_move in ["e2", "d1", "f1"] {
            reference_movelist.insert(pawn_move.to_string());
        }
        for file in b'a'..=b'h' {
            for rank in b'1'..=b'8' {
                reference_movelist.insert(format!("{}{}h", file as char, rank as char));
                reference_movelist.insert(format!("{}{}v", file as char, rank as char));
            }
        }

        assert_eq!(
            moves, reference_movelist,
            "moves generated must match reference"
        );
    }

    #[test]
    fn pov_symmetric() {
        use crate::board::Board;

        let mut board = Board::default();
        let moves = super::perft(board, 1);
        board.pass_turn();
        let moves2 = super::perft(board, 1);

        assert_eq!(moves, moves2, "perft(1) = {moves} != {moves2}");
    }

    #[test]
    fn pawn_move_symmetric() {
        use crate::board::Board;

        let mut board = Board::default();
        let moves = super::perft(board, 1);
        board.make_move("e2".parse().unwrap());
        let moves2 = super::perft(board, 1);

        assert_eq!(
            moves, moves2,
            "just moving the pawn forward shouldn't mess anything up."
        );
    }

    #[test]
    fn doing_perft_2_to_death() {
        use crate::board::Board;

        const MOVES: u64 = 131;

        let pawn_moves = [
            "e2".parse().unwrap(),
            "d1".parse().unwrap(),
            "f1".parse().unwrap(),
        ];

        let horizontal_wall_moves = (b'a'..=b'h')
            .flat_map(|file| {
                (b'1'..=b'8').flat_map(move |rank| {
                    let mut moves = Vec::new();
                    moves.push(format!("{}{}h", file as char, rank as char));
                    moves
                })
            })
            .map(|mv| mv.parse().unwrap());

        let vertical_wall_moves = (b'a'..=b'h')
            .flat_map(|file| {
                (b'1'..=b'8').flat_map(move |rank| {
                    let mut moves = Vec::new();
                    moves.push(format!("{}{}v", file as char, rank as char));
                    moves
                })
            })
            .map(|mv| mv.parse().unwrap());

        let blocker_wall_moves = [
            "d8h".parse().unwrap(),
            "e8h".parse().unwrap(),
            "d8v".parse().unwrap(),
            "e8v".parse().unwrap(),
        ];

        let mut board = Board::default();

        for pawn_move in &pawn_moves {
            eprintln!("pawn move: {pawn_move}");
            board.make_move(*pawn_move);
            let legal_moves_from_here = super::perft(board, 1);
            assert_eq!(legal_moves_from_here, MOVES);
            board = Board::default();
        }

        for horizontal_wall_move in horizontal_wall_moves {
            eprintln!("horizontal wall move: {horizontal_wall_move}");
            board.make_move(horizontal_wall_move);
            let legal_moves_from_here = super::perft(board, 1);
            let sq = horizontal_wall_move.wall_to_square().unwrap();
            let offset = u64::from(blocker_wall_moves.contains(&horizontal_wall_move));
            assert_eq!(
                legal_moves_from_here,
                if sq.is_west_edge() || sq.is_east_edge() {
                    MOVES - 3
                } else {
                    MOVES - 4
                } - offset
            );
            board = Board::default();
        }

        for vertical_wall_move in vertical_wall_moves {
            eprintln!("vertical wall move: {vertical_wall_move}");
            board.make_move(vertical_wall_move);
            let legal_moves_from_here = super::perft(board, 1);
            let sq = vertical_wall_move.wall_to_square().unwrap();
            let offset = u64::from(blocker_wall_moves.contains(&vertical_wall_move));
            assert_eq!(
                legal_moves_from_here,
                if sq.is_north_edge() || sq.is_south_edge() {
                    MOVES - 3
                } else {
                    MOVES - 4
                } - offset
            );
            board = Board::default();
        }
    }

    #[test]
    fn wallplaced_move_names() {
        use crate::board::Board;

        let mut reference_movelist = HashSet::new();
        for pawn_move in ["e2", "d1", "f1"] {
            reference_movelist.insert(pawn_move.to_string());
        }
        for file in b'a'..=b'h' {
            for rank in b'1'..=b'8' {
                reference_movelist.insert(format!("{}{}h", file as char, rank as char));
                reference_movelist.insert(format!("{}{}v", file as char, rank as char));
            }
        }

        let mut board = Board::default();

        // we're going to play e4h, which will exclude
        // e4h, e4v, d4h, and f4h from being legal.
        board.make_move("e4h".parse().unwrap());
        board.pass_turn();

        let mut moves = HashSet::new();
        board.generate_moves(|mv| {
            moves.insert(mv.to_string());
            false
        });

        // remove the corresponding moves from the reference
        reference_movelist.remove("e4h");
        reference_movelist.remove("e4v");
        reference_movelist.remove("d4h");
        reference_movelist.remove("f4h");

        let generated_but_not_reference = moves
            .difference(&reference_movelist)
            .cloned()
            .collect::<Vec<_>>();
        let reference_but_not_generated = reference_movelist
            .difference(&moves)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            moves, reference_movelist,
            "moves generated must match reference, but generated {generated_but_not_reference:?} not in reference and reference {reference_but_not_generated:?} not generated"
        );
    }

    #[test]
    fn wallplaced2_move_names() {
        use crate::board::Board;

        let mut reference_movelist = HashSet::new();
        for pawn_move in ["e2", "d1", "f1"] {
            reference_movelist.insert(pawn_move.to_string());
        }
        for file in b'a'..=b'h' {
            for rank in b'1'..=b'8' {
                reference_movelist.insert(format!("{}{}h", file as char, rank as char));
                reference_movelist.insert(format!("{}{}v", file as char, rank as char));
            }
        }

        let mut board = Board::default();

        // we're going to play a2h, which will exclude
        // a2h, a2v, and b2h from being legal.
        board.make_move("a2h".parse().unwrap());
        board.pass_turn();

        let mut moves = HashSet::new();
        board.generate_moves(|mv| {
            moves.insert(mv.to_string());
            false
        });

        // remove the corresponding moves from the reference
        reference_movelist.remove("a2h");
        reference_movelist.remove("a2v");
        reference_movelist.remove("b2h");

        let generated_but_not_reference = moves
            .difference(&reference_movelist)
            .cloned()
            .collect::<Vec<_>>();
        let reference_but_not_generated = reference_movelist
            .difference(&moves)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            moves, reference_movelist,
            "moves generated must match reference, but generated {generated_but_not_reference:?} not in reference and reference {reference_but_not_generated:?} not generated"
        );
    }

    #[test]
    fn wallplaced3_move_names() {
        use crate::board::Board;

        let mut reference_movelist = HashSet::new();
        for pawn_move in ["e2", "d1", "f1"] {
            reference_movelist.insert(pawn_move.to_string());
        }
        for file in b'a'..=b'h' {
            for rank in b'1'..=b'8' {
                reference_movelist.insert(format!("{}{}h", file as char, rank as char));
                reference_movelist.insert(format!("{}{}v", file as char, rank as char));
            }
        }

        let mut board = Board::default();

        // we're going to play a2v, which will exclude
        // a2v, a2h, a1v, and a3v from being legal.
        board.make_move("a2v".parse().unwrap());
        board.pass_turn();

        let mut moves = HashSet::new();
        board.generate_moves(|mv| {
            moves.insert(mv.to_string());
            false
        });

        // remove the corresponding moves from the reference
        reference_movelist.remove("a2v");
        reference_movelist.remove("a2h");
        reference_movelist.remove("a1v");
        reference_movelist.remove("a3v");

        let generated_but_not_reference = moves
            .difference(&reference_movelist)
            .cloned()
            .collect::<Vec<_>>();
        let reference_but_not_generated = reference_movelist
            .difference(&moves)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            moves, reference_movelist,
            "moves generated must match reference, but generated {generated_but_not_reference:?} not in reference and reference {reference_but_not_generated:?} not generated"
        );
    }

    #[test]
    fn pawn_blocked_forward_move_names() {
        use crate::board::Board;

        let mut reference_movelist = HashSet::new();
        for pawn_move in ["e2", "d1", "f1"] {
            reference_movelist.insert(pawn_move.to_string());
        }
        for file in b'a'..=b'h' {
            for rank in b'1'..=b'8' {
                reference_movelist.insert(format!("{}{}h", file as char, rank as char));
                reference_movelist.insert(format!("{}{}v", file as char, rank as char));
            }
        }

        let mut board = Board::default();

        // we're going to play e1h, which will exclude
        // e1h, e1v, d1h, and f1h from being legal.
        // it will *also* prevent the pawn from moving
        // forward, making e2 illegal.
        board.make_move("e1h".parse().unwrap());
        board.pass_turn();

        let mut moves = HashSet::new();
        board.generate_moves(|mv| {
            moves.insert(mv.to_string());
            false
        });

        // remove the corresponding moves from the reference
        reference_movelist.remove("e1h");
        reference_movelist.remove("e1v");
        reference_movelist.remove("d1h");
        reference_movelist.remove("f1h");
        reference_movelist.remove("e2");

        let generated_but_not_reference = moves
            .difference(&reference_movelist)
            .cloned()
            .collect::<Vec<_>>();
        let reference_but_not_generated = reference_movelist
            .difference(&moves)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            moves, reference_movelist,
            "moves generated must match reference, but generated {generated_but_not_reference:?} not in reference and reference {reference_but_not_generated:?} not generated"
        );
    }

    #[test]
    fn pawn_blocked_left_move_names() {
        use crate::board::Board;

        let mut reference_movelist = HashSet::new();
        for pawn_move in ["e2", "d1", "f1"] {
            reference_movelist.insert(pawn_move.to_string());
        }
        for file in b'a'..=b'h' {
            for rank in b'1'..=b'8' {
                reference_movelist.insert(format!("{}{}h", file as char, rank as char));
                reference_movelist.insert(format!("{}{}v", file as char, rank as char));
            }
        }

        let mut board = Board::default();

        // we're going to play e1v, which will exclude
        // e1v, e2v, and e1h from being legal.
        // it will *also* prevent the pawn from moving
        // right, making f1 illegal.
        board.make_move("e1v".parse().unwrap());
        board.pass_turn();

        let mut moves = HashSet::new();
        board.generate_moves(|mv| {
            moves.insert(mv.to_string());
            false
        });

        // remove the corresponding moves from the reference
        reference_movelist.remove("e1h");
        reference_movelist.remove("e1v");
        reference_movelist.remove("e2v");
        reference_movelist.remove("f1");

        let generated_but_not_reference = moves
            .difference(&reference_movelist)
            .cloned()
            .collect::<Vec<_>>();
        let reference_but_not_generated = reference_movelist
            .difference(&moves)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            moves, reference_movelist,
            "moves generated must match reference, but generated {generated_but_not_reference:?} not in reference and reference {reference_but_not_generated:?} not generated"
        );
    }

    #[test]
    fn perft_nowall() {
        use crate::board::Board;

        let mut board = Board::default();
        board.set_walls_in_pocket(0, 0);

        let count = super::perft(board, 1);

        assert_eq!(count, 3, "perft(1) = {count}");
    }

    #[test]
    fn perft_midboard_nowall() {
        use crate::board::Board;
        use crate::types::Move;

        let mut board = Board::default();
        board.set_walls_in_pocket(0, 0);
        board.make_move(Move::from_str("e2").unwrap());
        board.make_move(Move::from_str("e8").unwrap());

        let count = super::perft(board, 1);

        assert_eq!(count, 4, "perft(1) = {count}");
    }
}
