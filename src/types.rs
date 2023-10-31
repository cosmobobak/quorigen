use std::str::FromStr;
use std::fmt::Display;

/// Represents the colour of a pawn.
pub enum Colour {
    White,
    Black,
}

/// Represents the orientation of a wall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// Returns the square at the given index, without checking the index.
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        Self(index)
    }

    /// Returns the square given by the file and rank.
    pub const fn from_file_rank(file: u8, rank: u8) -> Option<Self> {
        Self::from_index(file + rank * 9)
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

    /// Returns the file of the square.
    pub const fn file(self) -> u8 {
        self.0 % 9
    }

    /// Returns the rank of the square.
    pub const fn rank(self) -> u8 {
        self.0 / 9
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

/// Represents a move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Pawn {
        to_square: Square,
    },
    Wall {
        to_square: Square,
        orientation: WallOrientation,
    },
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FILE_NAMES: &[u8] = b"abcdefghi";
        const RANK_NAMES: &[u8] = b"123456789";
        match *self {
            Self::Pawn { to_square } => write!(f, "{to_square}"),
            Self::Wall {
                to_square,
                orientation,
            } => write!(
                f,
                "{file}{rank}{orientation}",
                file = FILE_NAMES[to_square.file() as usize] as char,
                rank = RANK_NAMES[to_square.rank() as usize] as char,
                orientation = match orientation {
                    WallOrientation::Horizontal => "h",
                    WallOrientation::Vertical => "v",
                }
            ),
        }
    }
}

mod tests {
    #[test]
    fn move_display() {
        use super::{Move, Square, WallOrientation};
        use std::str::FromStr;

        let e4 = Square::from_str("e4").unwrap();
        let mv_e45 = Move::Wall {
            to_square: e4,
            orientation: WallOrientation::Vertical,
        };
        let mv_4ef = Move::Wall {
            to_square: e4,
            orientation: WallOrientation::Horizontal,
        };

        assert_eq!(mv_e45.to_string(), "e4v");
        assert_eq!(mv_4ef.to_string(), "e4h");
    }

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

    #[test]
    fn rank_file_name_coherence() {
        #![allow(clippy::cast_possible_truncation)]
        use super::Square;

        let ranks = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let files = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        for (rank_index, rank) in ranks.iter().enumerate() {
            for (file_index, file) in files.iter().enumerate() {
                let square_name = format!("{file}{rank}");
                let square_from_name = square_name.parse::<Square>().unwrap();
                let square_from_file_rank =
                    Square::from_file_rank(file_index as u8, rank_index as u8).unwrap();
                assert_eq!(
                    square_from_file_rank, square_from_name,
                    "expected {square_from_name} got {square_from_file_rank}"
                );
            }
        }
    }
}
