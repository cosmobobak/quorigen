use crate::types::Move;

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
    //
    // locating pawns is easy: we can use a u8 for each pawn, as an index
    // mapped to some ordering of squares.
    //
    // locating walls is harder: we should probably use an NxN matrix that stores
    // wall location and orientation.

    // an initial implementation might look like this:
    /// the pawns are stored as a pair of u8s, each of which is an index
    /// into a 9x9 matrix of squares.
    pawns: [u8; 2],
    /// the walls are stored as a 10x10 matrix of booleans, where each
    /// slot represents a "crossroads" in wall grooves of the board.
    /// this means that the corners of the matrix are unused, as a wall
    /// placed in those slots would be partially off the board.
    walls: [[u8; 10]; 10],
}

impl Default for Board {
    fn default() -> Self {
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