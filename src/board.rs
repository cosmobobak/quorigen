use crate::types::{Move, Square, SquareSet};

pub struct Board {
    // we want a nice memory-efficient representation of the board
    // that also allows for fast move generation.
    //
    // typically, we'd want to use bitboards for this, but since
    // quoridor uses a 9x9 board, we can't fit an occupancy bitboard
    // into a single 64-bit integer. (9 * 9 = 81, which is more than 64.)
    //
    // there are two relvant considerations for storing board state:
    // 1. where are the pawns?
    // 2. where are the walls?
    
    /// The pawns on the board.
    pawns: [Square; 2],
    /// The horizontal walls on the board.
    horizontal_walls: SquareSet,
    /// The vertical walls on the board.
    vertical_walls: SquareSet,
    /// The number of walls each player has left.
    walls_in_pocket: [u8; 2],
}

impl Default for Board {
    fn default() -> Self {
        // Self {
        //     pawns: [Square::from_index(4).unwrap(), Square::from_index(76).unwrap()],
        //     horizontal_walls: SquareSet::empty(),
        //     vertical_walls: SquareSet::empty(),
        //     walls_in_pocket: [10, 10],
        // }
        todo!()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        todo!()
    }

    pub fn generate_moves(&self, callback: impl FnMut(Move) -> bool) {
        todo!()
    }
}