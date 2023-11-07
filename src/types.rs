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
pub struct Square9x9(u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square8x8(u8);

impl Square9x9 {
    /// Returns the index of the square.
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Determines whether the square is on the edge of the board.
    pub const fn is_edge(self) -> bool {
        self.is_west_edge() || self.is_east_edge() || self.is_north_edge() || self.is_south_edge()
    }

    /// Determines whether the square is on the west edge of the board.
    pub const fn is_west_edge(self) -> bool {
        self.file() == 0
    }

    /// Determines whether the square is on the east edge of the board.
    pub const fn is_east_edge(self) -> bool {
        self.file() == 8
    }

    /// Determines whether the square is on the north edge of the board.
    pub const fn is_north_edge(self) -> bool {
        self.rank() == 8
    }

    /// Determines whether the square is on the south edge of the board.
    pub const fn is_south_edge(self) -> bool {
        self.rank() == 0
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

    /// Returns the square below the given square.
    pub const fn below(self) -> Option<Self> {
        if self.0 < 9 {
            None
        } else {
            Some(Self(self.0 - 9))
        }
    }

    /// Returns the square above the given square.
    pub const fn above(self) -> Option<Self> {
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

impl Square8x8 {
    /// Returns the index of the square.
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Determines whether the square is on the edge of the board.
    pub const fn is_edge(self) -> bool {
        self.is_west_edge() || self.is_east_edge() || self.is_north_edge() || self.is_south_edge()
    }

    /// Determines whether the square is on the west edge of the board.
    pub const fn is_west_edge(self) -> bool {
        self.file() == 0
    }

    /// Determines whether the square is on the east edge of the board.
    pub const fn is_east_edge(self) -> bool {
        self.file() == 7
    }

    /// Determines whether the square is on the north edge of the board.
    pub const fn is_north_edge(self) -> bool {
        self.rank() == 7
    }

    /// Determines whether the square is on the south edge of the board.
    pub const fn is_south_edge(self) -> bool {
        self.rank() == 0
    }

    /// Returns the square at the given index.
    const fn from_index(index: u8) -> Option<Self> {
        if index < 64 {
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
        Self::from_index(file + rank * 8)
    }

    /// Returns the square below the given square.
    pub const fn below(self) -> Option<Self> {
        if self.0 < 8 {
            None
        } else {
            Some(Self(self.0 - 8))
        }
    }

    /// Returns the square above the given square.
    pub const fn above(self) -> Option<Self> {
        if self.0 > 55 {
            None
        } else {
            Some(Self(self.0 + 8))
        }
    }

    /// Returns the square to the left of the given square.
    pub const fn left(self) -> Option<Self> {
        if self.0 % 8 == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    /// Returns the square to the right of the given square.
    pub const fn right(self) -> Option<Self> {
        if self.0 % 8 == 7 {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    /// Returns the file of the square.
    pub const fn file(self) -> u8 {
        self.0 % 8
    }

    /// Returns the rank of the square.
    pub const fn rank(self) -> u8 {
        self.0 / 8
    }
}

impl std::fmt::Display for Square9x9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.0 % 9;
        let rank = self.0 / 9;
        write!(f, "{}{}", (b'a' + file) as char, rank + 1)
    }
}

impl std::fmt::Display for Square8x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.0 % 8;
        let rank = self.0 / 8;
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

impl FromStr for Square9x9 {
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

impl FromStr for Square8x8 {
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
        if !(0..8).contains(&file_index) {
            return Err(SquareParseError::FileOutOfRange(file));
        }
        if !(0..8).contains(&rank_index) {
            return Err(SquareParseError::RankOutOfRange(rank));
        }
        Ok(Self(file_index + rank_index * 8))
    }
}

/// Represents a move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Pawn {
        to_square: Square9x9,
    },
    Wall {
        to_square: Square8x8,
        orientation: WallOrientation,
    },
}

impl Move {
    /// Gets the to-square of a pawn move.
    pub const fn pawn_to_square(self) -> Option<Square9x9> {
        match self {
            Self::Pawn { to_square } => Some(to_square),
            Self::Wall { .. } => None,
        }
    }

    /// Gets the to-square of a wall move.
    pub const fn wall_to_square(self) -> Option<Square8x8> {
        match self {
            Self::Pawn { .. } => None,
            Self::Wall { to_square, .. } => Some(to_square),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveParseError {
    Square9x9ParseError(SquareParseError),
    Square8x8ParseError(SquareParseError),
    AlignmentParseError(char),
    TooShort,
    TooLong(usize),
    Empty,
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            0 => Err(MoveParseError::Empty),
            1 => Err(MoveParseError::TooShort),
            2 => {
                let to_square = s.parse::<Square9x9>().map_err(MoveParseError::Square9x9ParseError)?;
                Ok(Self::Pawn { to_square })
            }
            3 => {
                let square = &s[..2];
                let square = square.parse::<Square8x8>().map_err(MoveParseError::Square8x8ParseError)?;
                let orientation = &s[2..];
                let orientation = match orientation {
                    "h" => WallOrientation::Horizontal,
                    "v" => WallOrientation::Vertical,
                    _ => return Err(MoveParseError::AlignmentParseError(orientation.chars().next().unwrap())),
                };
                Ok(Self::Wall { to_square: square, orientation })
            }
            len => Err(MoveParseError::TooLong(len)),
        }
    }
}

impl TryFrom<Square9x9> for Square8x8 {
    type Error = ();

    fn try_from(value: Square9x9) -> Result<Self, Self::Error> {
        let rank = value.rank();
        let file = value.file();
        Self::from_file_rank(file, rank).ok_or(())
    }
}

mod tests {
    #[test]
    fn move_display() {
        use super::{Move, Square9x9, Square8x8, WallOrientation};
        use std::str::FromStr;

        let e4: Square8x8 = Square9x9::from_str("e4").unwrap().try_into().unwrap();
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
        use super::{Square9x9, SquareParseError};
        use std::str::FromStr;

        assert_eq!(
            Square9x9::from_str("a1").unwrap(),
            Square9x9::from_index(0).unwrap()
        );
        assert_eq!(
            Square9x9::from_str("i9").unwrap(),
            Square9x9::from_index(80).unwrap()
        );

        assert_eq!(
            "a0".parse::<Square9x9>(),
            Err(SquareParseError::RankSubtractionUnderflow('0'))
        );
        assert_eq!("a10".parse::<Square9x9>(), Err(SquareParseError::TooLong(3)));
        assert_eq!(
            "j1".parse::<Square9x9>(),
            Err(SquareParseError::FileOutOfRange('j'))
        );
        assert_eq!("A1".parse::<Square9x9>(), Ok(Square9x9::from_index(0).unwrap()));
        assert_eq!("a".parse::<Square9x9>(), Err(SquareParseError::TooShort));
        assert_eq!("".parse::<Square9x9>(), Err(SquareParseError::Empty));
        assert_eq!(
            "!!".parse::<Square9x9>(),
            Err(SquareParseError::NonAlphabeticFile('!'))
        );
    }

    #[test]
    fn square_round_trip() {
        use super::Square9x9;
        use std::str::FromStr;

        for index in 0..81 {
            let square = Square9x9::from_index(index).unwrap();
            let square_str = square.to_string();
            let square_parsed = Square9x9::from_str(&square_str).unwrap();
            assert_eq!(square, square_parsed);
        }
    }

    #[test]
    fn rank_file_name_coherence() {
        #![allow(clippy::cast_possible_truncation)]
        use super::Square9x9;

        let ranks = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let files = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        for (rank_index, rank) in ranks.iter().enumerate() {
            for (file_index, file) in files.iter().enumerate() {
                let square_name = format!("{file}{rank}");
                let square_from_name = square_name.parse::<Square9x9>().unwrap();
                let square_from_file_rank =
                    Square9x9::from_file_rank(file_index as u8, rank_index as u8).unwrap();
                assert_eq!(
                    square_from_file_rank, square_from_name,
                    "expected {square_from_name} got {square_from_file_rank}"
                );
            }
        }
    }

    #[test]
    fn a2v_edge() {
        let a2v = "a2v".parse::<super::Move>().unwrap();
        let a2 = a2v.wall_to_square().unwrap();
        assert!(a2.is_edge());
        assert!(a2.is_west_edge());
        assert!(!a2.is_east_edge());
        assert!(!a2.is_north_edge());
        assert!(!a2.is_south_edge());
    }
}
