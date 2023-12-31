use std::{ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub, SubAssign,
}, fmt::Display};

use crate::types::Square8x8;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SquareSet {
    inner: u64,
}

pub static BB_RANKS: [SquareSet; 8] = [
    SquareSet::RANK_1,
    SquareSet::RANK_2,
    SquareSet::RANK_3,
    SquareSet::RANK_4,
    SquareSet::RANK_5,
    SquareSet::RANK_6,
    SquareSet::RANK_7,
    SquareSet::RANK_8,
];

pub static BB_FILES: [SquareSet; 8] = [
    SquareSet::FILE_A,
    SquareSet::FILE_B,
    SquareSet::FILE_C,
    SquareSet::FILE_D,
    SquareSet::FILE_E,
    SquareSet::FILE_F,
    SquareSet::FILE_G,
    SquareSet::FILE_H,
];

impl SquareSet {
    pub const EMPTY: Self = Self { inner: 0 };
    pub const FULL: Self = Self { inner: !0 };

    pub const RANK_1: Self = Self { inner: 0x0000_0000_0000_00FF };
    pub const RANK_2: Self = Self { inner: 0x0000_0000_0000_FF00 };
    pub const RANK_3: Self = Self { inner: 0x0000_0000_00FF_0000 };
    pub const RANK_4: Self = Self { inner: 0x0000_0000_FF00_0000 };
    pub const RANK_5: Self = Self { inner: 0x0000_00FF_0000_0000 };
    pub const RANK_6: Self = Self { inner: 0x0000_FF00_0000_0000 };
    pub const RANK_7: Self = Self { inner: 0x00FF_0000_0000_0000 };
    pub const RANK_8: Self = Self { inner: 0xFF00_0000_0000_0000 };
    pub const FILE_A: Self = Self { inner: 0x0101_0101_0101_0101 };
    pub const FILE_B: Self = Self { inner: 0x0202_0202_0202_0202 };
    pub const FILE_C: Self = Self { inner: 0x0404_0404_0404_0404 };
    pub const FILE_D: Self = Self { inner: 0x0808_0808_0808_0808 };
    pub const FILE_E: Self = Self { inner: 0x1010_1010_1010_1010 };
    pub const FILE_F: Self = Self { inner: 0x2020_2020_2020_2020 };
    pub const FILE_G: Self = Self { inner: 0x4040_4040_4040_4040 };
    pub const FILE_H: Self = Self { inner: 0x8080_8080_8080_8080 };
    pub const LIGHT_SQUARES: Self = Self { inner: 0x55AA_55AA_55AA_55AA };
    pub const DARK_SQUARES: Self = Self { inner: 0xAA55_AA55_AA55_AA55 };

    pub const fn from_inner(inner: u64) -> Self {
        Self { inner }
    }

    pub const fn inner(self) -> u64 {
        self.inner
    }

    pub const fn count(self) -> u32 {
        self.inner.count_ones()
    }

    pub const fn is_empty(self) -> bool {
        self.inner == 0
    }

    pub const fn is_full(self) -> bool {
        self.inner == !0
    }

    pub const fn non_empty(self) -> bool {
        self.inner != 0
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self { inner: self.inner & other.inner }
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.inner & other.inner) == other.inner
    }

    pub const fn contains_square(self, square: Square8x8) -> bool {
        (self.inner & (1 << square.index())) != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self { inner: self.inner | other.inner }
    }

    pub const fn add_square(self, square: Square8x8) -> Self {
        Self { inner: self.inner | (1 << square.index()) }
    }

    pub const fn remove(self, other: Self) -> Self {
        Self { inner: self.inner & !other.inner }
    }

    pub const fn remove_square(self, square: Square8x8) -> Self {
        Self { inner: self.inner & !(1 << square.index()) }
    }

    pub const fn toggle(self, other: Self) -> Self {
        Self { inner: self.inner ^ other.inner }
    }

    pub const fn toggle_square(self, square: Square8x8) -> Self {
        Self { inner: self.inner ^ (1 << square.index()) }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub const fn first(self) -> Square8x8 {
        debug_assert!(self.inner != 0, "Tried to get first square of empty bitboard");
        unsafe { Square8x8::from_index_unchecked(self.inner.trailing_zeros() as u8) }
    }

    pub const fn from_square(square: Square8x8) -> Self {
        Self { inner: 1 << square.index() }
    }

    pub fn north_east_one(self) -> Self {
        Self { inner: self.inner << 9 } & !Self::FILE_A
    }
    pub fn north_west_one(self) -> Self {
        Self { inner: self.inner << 7 } & !Self::FILE_H
    }
    pub fn south_east_one(self) -> Self {
        Self { inner: self.inner >> 7 } & !Self::FILE_A
    }
    pub fn south_west_one(self) -> Self {
        Self { inner: self.inner >> 9 } & !Self::FILE_H
    }
    pub fn east_one(self) -> Self {
        Self { inner: self.inner << 1 } & !Self::FILE_A
    }
    pub fn west_one(self) -> Self {
        Self { inner: self.inner >> 1 } & !Self::FILE_H
    }
    pub const fn north_one(self) -> Self {
        Self { inner: self.inner << 8 }
    }
    pub const fn south_one(self) -> Self {
        Self { inner: self.inner >> 8 }
    }
}

impl BitOr for SquareSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner | rhs.inner }
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
        Self { inner: self.inner & rhs.inner }
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
        Self { inner: self.inner ^ rhs.inner }
    }
}

impl BitXorAssign for SquareSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner ^= rhs.inner;
    }
}

impl Sub for SquareSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner & !rhs.inner }
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
        Self { inner: !self.inner }
    }
}

impl IntoIterator for SquareSet {
    type Item = Square8x8;
    type IntoIter = SquareSetIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareSetIter { inner: self.inner }
    }
}

pub struct SquareSetIter {
    inner: u64,
}

impl Iterator for SquareSetIter {
    type Item = Square8x8;

    fn next(&mut self) -> Option<Self::Item> {
        #![allow(clippy::cast_possible_truncation)]
        if self.inner == 0 {
            None
        } else {
            let index = self.inner.trailing_zeros() as u8;
            self.inner &= self.inner - 1;
            let rank = index / 8;
            let file = index % 8;
            Some(unsafe { Square8x8::from_file_rank(file, rank).unwrap_unchecked() })
        }
    }
}

impl Default for SquareSet {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Display for SquareSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::with_capacity(64 * 2 + 8);
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square8x8::from_file_rank(file, rank).unwrap();
                if self.contains_square(square) {
                    builder.push('X');
                } else {
                    builder.push('.');
                }
                builder.push(' ');
            }
            builder.push('\n');
        }
        write!(f, "{builder}")
    }
}