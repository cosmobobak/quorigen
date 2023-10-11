use crate::types::{Move, Square, SquareSet, WallOrientation};

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
    /// Ply since the start of the game.
    ply: u16,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pawns: ["e1".parse().unwrap(), "e9".parse().unwrap()],
            horizontal_walls: SquareSet::default(),
            vertical_walls: SquareSet::default(),
            walls_in_pocket: [10, 10],
            ply: 0,
        }
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

    pub fn generate_moves(&self, mut callback: impl FnMut(Move) -> bool) {
        let turn_index = usize::from(self.ply % 2);
        let pawn = self.pawns[turn_index];
        let opponent_pawn = self.pawns[1 - turn_index];

        // generate pawn moves
        if let Some(mut to_square) = pawn.above() {
            if to_square == opponent_pawn {
                to_square = opponent_pawn.above().unwrap();
            }
            if !self.horizontal_walls.contains(to_square) && callback(Move::Pawn { to_square }) {
                return;
            }
        }
        if let Some(mut to_square) = pawn.below() {
            if to_square == opponent_pawn {
                to_square = opponent_pawn.below().unwrap();
            }
            if !self.horizontal_walls.contains(to_square) && callback(Move::Pawn { to_square }) {
                return;
            }
        }
        if let Some(mut to_square) = pawn.left() {
            if to_square == opponent_pawn {
                to_square = opponent_pawn.left().unwrap();
            }
            if !self.vertical_walls.contains(to_square) && callback(Move::Pawn { to_square }) {
                return;
            }
        }
        if let Some(mut to_square) = pawn.right() {
            if to_square == opponent_pawn {
                to_square = opponent_pawn.right().unwrap();
            }
            if !self.vertical_walls.contains(to_square) && callback(Move::Pawn { to_square }) {
                return;
            }
        }

        // generate wall moves
        let our_wall_count = self.walls_in_pocket[turn_index];
        if our_wall_count == 0 {
            return;
        }

        // generate horizontal wall moves
        // these moves are blocked by
        // 1. the left-hand sides of existing horizontal walls
        let mut blockers = self.horizontal_walls;
        // 2. the right-hand sides of existing horizontal walls
        blockers |= blockers.east_one();
        // 3. the middles of existing vertical walls
        blockers |= self.vertical_walls.east_one().south_one();

        let moves = blockers.complement();
        for to_square in moves {
            if callback(Move::Wall { to_square, orientation: WallOrientation::Horizontal }) {
                return;
            }
        }

        // generate vertical wall moves
        // these moves are blocked by
        // 1. the top sides of existing vertical walls
        let mut blockers = self.vertical_walls;
        // 2. the bottom sides of existing vertical walls
        blockers |= blockers.south_one();
        // 3. the middles of existing horizontal walls
        blockers |= self.horizontal_walls.east_one().south_one();

        let moves = blockers.complement();
        for to_square in moves {
            if callback(Move::Wall { to_square, orientation: WallOrientation::Vertical }) {
                return;
            }
        }
    }
}