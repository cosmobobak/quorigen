use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub, SubAssign,
};
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

    /// Returns the square given by the file and rank.
    const fn from_file_rank(file: u8, rank: u8) -> Option<Self> {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SquareSet {
    /// The inner representation of the occupancy.
    /// Must be 128 bits, as there are 81 squares on the board.
    inner: u128,
}

impl SquareSet {
    const ALL_MASK: u128 = {
        let mut mask = 0;
        let mut index = 0;
        while index < 81 {
            mask |= 1 << index;
            index += 1;
        }
        mask
    };

    const A_FILE: Self = {
        let mut mask = 0;
        let mut index = 0;
        while index < 81 {
            mask |= 1 << index;
            index += 9;
        }
        Self { inner: mask }
    };

    const I_FILE: Self = {
        let mut mask = 0;
        let mut index = 8;
        while index < 81 {
            mask |= 1 << index;
            index += 9;
        }
        Self { inner: mask }
    };

    /// Creates a new empty `SquareSet`.
    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    /// Creates a new `SquareSet` with the given square set.
    const fn with(inner: u128) -> Self {
        Self { inner }
    }

    /// Returns whether the given square is occupied.
    pub const fn contains(self, square: Square) -> bool {
        self.inner & (1 << square.index()) != 0
    }

    /// Adds the given square to the set.
    pub const fn add(self, square: Square) -> Self {
        Self {
            inner: self.inner | 1 << square.index(),
        }
    }

    /// Removes the given square from the set.
    pub const fn remove(self, square: Square) -> Self {
        Self {
            inner: self.inner & !(1 << square.index()),
        }
    }

    /// Shift the squares up by one.
    /// Squares on the top row are removed.
    pub const fn north_one(self) -> Self {
        Self {
            inner: (self.inner << 9) & Self::ALL_MASK,
        }
    }

    /// Shift the squares down by one.
    /// Squares on the bottom row are removed.
    pub const fn south_one(self) -> Self {
        Self {
            inner: self.inner >> 9,
        }
    }

    /// Shift the squares left by one.
    /// Squares on the leftmost column are removed.
    pub const fn west_one(self) -> Self {
        Self {
            inner: self.inner >> 1,
        }
        .intersection(Self::I_FILE.complement())
    }

    /// Shift the squares right by one.
    /// Squares on the rightmost column are removed.
    pub const fn east_one(self) -> Self {
        Self {
            inner: self.inner << 1,
        }
        .intersection(Self::A_FILE.complement())
    }

    pub const fn complement(self) -> Self {
        Self {
            inner: !self.inner & Self::ALL_MASK,
        }
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self {
            inner: self.inner & other.inner,
        }
    }
}

impl BitOr for SquareSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner | rhs.inner,
        }
    }
}

impl BitOrAssign for SquareSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner |= rhs.inner;
    }
}

impl BitAnd for SquareSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner & rhs.inner,
        }
    }
}

impl BitAndAssign for SquareSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner &= rhs.inner;
    }
}

impl BitXor for SquareSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner ^ rhs.inner & Self::ALL_MASK,
        }
    }
}

impl BitXorAssign for SquareSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner ^= rhs.inner;
        self.inner &= Self::ALL_MASK;
    }
}

impl Sub for SquareSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner & !rhs.inner,
        }
    }
}

impl SubAssign for SquareSet {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner &= !rhs.inner;
    }
}

impl Not for SquareSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            inner: !self.inner & Self::ALL_MASK,
        }
    }
}

impl Shr<u8> for SquareSet {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            inner: self.inner >> rhs,
        }
    }
}

impl Shl<u8> for SquareSet {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            inner: (self.inner << rhs) & Self::ALL_MASK,
        }
    }
}

impl std::fmt::Display for SquareSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..9).rev() {
            for file in 0..9 {
                let square = Square::from_file_rank(file, rank).unwrap();
                if self.contains(square) {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        assert!(
            self.inner & !Self::ALL_MASK == 0,
            "Squares outside the board are occupied."
        );
        Ok(())
    }
}

impl IntoIterator for SquareSet {
    type Item = Square;
    type IntoIter = SquareSetIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareSetIter { inner: self.inner }
    }
}

pub struct SquareSetIter {
    inner: u128,
}

impl Iterator for SquareSetIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner == 0 {
            None
        } else {
            let index = self.inner.trailing_zeros() as u8;
            self.inner &= self.inner - 1;
            Some(Square(index))
        }
    }
}

/// Represents a move.
pub enum Move {
    Pawn {
        to_square: Square,
    },
    Wall {
        to_square: Square,
        orientation: WallOrientation,
    },
}

#[allow(clippy::assertions_on_constants)]
const _A_FILE_SENSIBLE: () = assert!(SquareSet::A_FILE.inner & !SquareSet::ALL_MASK == 0);
#[allow(clippy::assertions_on_constants)]
const _I_FILE_SENSIBLE: () = assert!(SquareSet::I_FILE.inner & !SquareSet::ALL_MASK == 0);

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

    #[test]
    fn squareset_add_remove() {
        use super::{Square, SquareSet};

        let a1: Square = "a1".parse().unwrap();
        let mut set = SquareSet::new();
        assert!(!set.contains(a1));
        set = set.add(a1);
        assert!(set.contains(a1));
        set = set.remove(a1);
        assert!(!set.contains(a1));
    }

    #[test]
    fn squareset_north_one() {
        use super::SquareSet;
        use std::str::FromStr;

        let corners = ["a1", "i1", "a9", "i9"];
        let corners = corners
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        let north_one = corners.north_one();

        // no "a10" or "i10", these would be off the board.
        let expected = ["a2", "i2"];
        let expected = expected
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        assert_eq!(
            north_one, expected,
            "expected \n{expected} got \n{north_one}"
        );
    }

    #[test]
    fn squareset_south_one() {
        use super::SquareSet;
        use std::str::FromStr;

        let corners = ["a1", "i1", "a9", "i9"];
        let corners = corners
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        let south_one = corners.south_one();

        // no "a0" or "i0", these would be off the board.
        let expected = ["a8", "i8"];
        let expected = expected
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        assert_eq!(
            south_one, expected,
            "expected \n{expected} got \n{south_one}"
        );
    }

    #[test]
    fn squareset_west_one() {
        use super::SquareSet;
        use std::str::FromStr;

        let corners = ["a1", "i1", "a9", "i9"];
        let corners = corners
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        let west_one = corners.west_one();

        let expected = ["h1", "h9"];
        let expected = expected
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        assert_eq!(west_one, expected, "expected \n{expected} got \n{west_one}");
    }

    #[test]
    fn squareset_east_one() {
        use super::SquareSet;
        use std::str::FromStr;

        let corners = ["a1", "i1", "a9", "i9"];
        let corners = corners
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        let east_one = corners.east_one();

        // no "j1" or "j9", these would be off the board.
        let expected = ["b1", "b9"];
        let expected = expected
            .into_iter()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .fold(SquareSet::new(), SquareSet::add);

        assert_eq!(east_one, expected, "expected \n{expected} got \n{east_one}");
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
