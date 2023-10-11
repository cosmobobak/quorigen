use std::str::FromStr;

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
    pub const fn index(self) -> u8 {
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
    pub const fn above(self) -> Option<Self> {
        if self.0 < 9 {
            None
        } else {
            Some(Self(self.0 - 9))
        }
    }

    /// Returns the square below the given square.
    pub const fn below(self) -> Option<Self> {
        if self.0 > 71 {
            None
        } else {
            Some(Self(self.0 + 9))
        }
    }

    /// Returns the square to the left of the given square.
    pub const fn left(self) -> Option<Self> {
        if self.0 % 9 == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    /// Returns the square to the right of the given square.
    pub const fn right(self) -> Option<Self> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareParseError {
    TooLong(usize),
    TooShort,
    Empty,
    NonAlphabeticFile(char),
    NonDigitRank(char),
    FileSubtractionUnderflow(char),
    RankSubtractionUnderflow(char),
    FileOutOfRange(char),
    RankOutOfRange(char),
}

impl FromStr for Square {
    type Err = SquareParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            0 => return Err(SquareParseError::Empty),
            1 => return Err(SquareParseError::TooShort),
            2 => {}
            len => return Err(SquareParseError::TooLong(len)),
        }
        let mut chars = s.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();
        if !file.is_ascii_alphabetic() {
            return Err(SquareParseError::NonAlphabeticFile(file));
        }
        if !rank.is_ascii_digit() {
            return Err(SquareParseError::NonDigitRank(rank));
        }
        let file_lower = file.to_ascii_lowercase();
        let file_index = (file_lower as u8)
            .checked_sub(b'a')
            .ok_or(SquareParseError::FileSubtractionUnderflow(file))?;
        let rank_index = (rank as u8)
            .checked_sub(b'1')
            .ok_or(SquareParseError::RankSubtractionUnderflow(rank))?;
        if !(0..9).contains(&file_index) {
            return Err(SquareParseError::FileOutOfRange(file));
        }
        if !(0..9).contains(&rank_index) {
            return Err(SquareParseError::RankOutOfRange(rank));
        }
        Ok(Self(file_index + rank_index * 9))
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
    Pawn {
        to_sq: Square,
    },
    Wall {
        to_sq: Square,
        orientation: WallOrientation,
    },
}

mod tests {
    #[test]
    fn square_parsing() {
        use super::{Square, SquareParseError};
        use std::str::FromStr;

        assert_eq!(
            Square::from_str("a1").unwrap(),
            Square::from_index(0).unwrap()
        );
        assert_eq!(
            Square::from_str("i9").unwrap(),
            Square::from_index(80).unwrap()
        );

        assert_eq!(
            "a0".parse::<Square>(),
            Err(SquareParseError::RankSubtractionUnderflow('0'))
        );
        assert_eq!("a10".parse::<Square>(), Err(SquareParseError::TooLong(3)));
        assert_eq!(
            "j1".parse::<Square>(),
            Err(SquareParseError::FileOutOfRange('j'))
        );
        assert_eq!("A1".parse::<Square>(), Ok(Square::from_index(0).unwrap()));
        assert_eq!("a".parse::<Square>(), Err(SquareParseError::TooShort));
        assert_eq!("".parse::<Square>(), Err(SquareParseError::Empty));
        assert_eq!(
            "!!".parse::<Square>(),
            Err(SquareParseError::NonAlphabeticFile('!'))
        );
    }

    #[test]
    fn square_round_trip() {
        use super::Square;
        use std::str::FromStr;

        for index in 0..81 {
            let square = Square::from_index(index).unwrap();
            let square_str = square.to_string();
            let square_parsed = Square::from_str(&square_str).unwrap();
            assert_eq!(square, square_parsed);
        }
    }
}
