use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub, SubAssign,
};

use crate::types::Square;

/// Represents an occupancy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SquareSet {
    /// The inner representation of the occupancy.
    /// Must be 128 bits, as there are 81 squares on the board.
    inner: u128,
}

impl SquareSet {
    pub const ALL_MASK: u128 = {
        let mut mask = 0;
        let mut index = 0;
        while index < 81 {
            mask |= 1 << index;
            index += 1;
        }
        mask
    };

    pub const A_FILE: Self = {
        let mut mask = 0;
        let mut index = 0;
        while index < 81 {
            mask |= 1 << index;
            index += 9;
        }
        Self { inner: mask }
    };

    pub const I_FILE: Self = {
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
        #![allow(clippy::cast_possible_truncation)]
        if self.inner == 0 {
            None
        } else {
            let index = self.inner.trailing_zeros() as u8;
            self.inner &= self.inner - 1;
            Some(unsafe { Square::from_index_unchecked(index) })
        }
    }
}

#[allow(clippy::assertions_on_constants)]
const _A_FILE_SENSIBLE: () = assert!(SquareSet::A_FILE.inner & !SquareSet::ALL_MASK == 0);
#[allow(clippy::assertions_on_constants)]
const _I_FILE_SENSIBLE: () = assert!(SquareSet::I_FILE.inner & !SquareSet::ALL_MASK == 0);

mod tests {
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
}