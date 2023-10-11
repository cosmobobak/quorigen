
/// Represents the colour of a pawn.
pub enum Colour {
    White,
    Black,
}

/// Represents the orientation of a wall.
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square(u8);

impl Square {
    /// Returns the index of the square.
    const fn index(self) -> u8 {
        self.0
    }

    /// Returns the square at the given index.
    const fn from_index(index: u8) -> Option<Self> {
        if index < 81 {
            Some(Self(index))
        } else {
            None
        }
    }

    /// Returns the square above the given square.
    const fn above(self) -> Option<Self> {
        if self.0 < 9 {
            None
        } else {
            Some(Self(self.0 - 9))
        }
    }

    /// Returns the square below the given square.
    const fn below(self) -> Option<Self> {
        if self.0 > 71 {
            None
        } else {
            Some(Self(self.0 + 9))
        }
    }

    /// Returns the square to the left of the given square.
    const fn left(self) -> Option<Self> {
        if self.0 % 9 == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    /// Returns the square to the right of the given square.
    const fn right(self) -> Option<Self> {
        if self.0 % 9 == 8 {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.0 % 9;
        let rank = self.0 / 9;
        write!(f, "{}{}", (b'a' + file) as char, rank + 1)
    }
}

/// Represents an occupancy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SquareSet {
    /// The inner representation of the occupancy.
    /// Must be 128 bits, as there are 81 squares on the board.
    inner: u128,
}

impl SquareSet {
    /// Creates a new empty `SquareSet`.
    const fn new() -> Self {
        Self { inner: 0 }
    }

    /// Creates a new `SquareSet` with the given square set.
    const fn with(inner: u128) -> Self {
        Self { inner }
    }

    /// Returns whether the given square is occupied.
    const fn contains(self, square: Square) -> bool {
        self.inner & (1 << square.index()) != 0
    }

    /// Adds the given square to the set.
    fn add(&mut self, square: Square) {
        self.inner |= 1 << square.index();
    }

    /// Removes the given square from the set.
    fn remove(&mut self, square: Square) {
        self.inner &= !(1 << square.index());
    }
}

impl std::fmt::Display for SquareSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..9).rev() {
            for col in 0..9 {
                let square = Square::from_index(row * 9 + col).unwrap();
                if self.contains(square) {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Represents a move.
pub enum Move {
    Pawn { to_sq: Square },
    Wall { to_sq: Square, orientation: WallOrientation },
}