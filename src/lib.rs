//! A crate for bingo.

#![no_std]

#[cfg(feature = "rand")]
mod rand;

/// A card.
///
/// Squares are indexed as:
///
/// |  b  |  i  |  n  |  g  |  o  |
/// | :-: | :-: | :-: | :-: | :-: |
/// |  0  |  5  |  10 |  15 |  20 |
/// |  1  |  6  |  11 |  16 |  21 |
/// |  2  |  7  |  12 |  17 |  22 |
/// |  3  |  8  |  13 |  18 |  23 |
/// |  4  |  9  |  14 |  19 |  24 |
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Card {
    numbers: [u8; 25],
    marked: u32,
}

impl Card {
    /// Creates a new card.
    pub const fn new(numbers: [u8; 25], marked: u32) -> Self {
        Self { numbers, marked }
    }

    /// Returns the numbers.
    pub const fn numbers(&self) -> &[u8; 25] {
        &self.numbers
    }

    /// Returns bit flags representing marked squares.
    pub const fn marked(&self) -> u32 {
        self.marked
    }

    /// Calculates two sets of bit flags: `ready` and `complete`.
    ///
    /// | name | description |
    /// | :-: | - |
    /// | `ready` | Squares in a row or a column which have four marked squares |
    /// | `complete` | Squares in a row or a column which have five marked squares |
    pub fn info(&self) -> (u32, u32) {
        #[allow(clippy::unusual_byte_groupings)]
        const LINES: [u32; 12] = [
            0b00000_00000_00000_00000_11111,
            0b00000_00000_00000_11111_00000,
            0b00000_00000_11111_00000_00000,
            0b00000_11111_00000_00000_00000,
            0b11111_00000_00000_00000_00000,
            0b00001_00001_00001_00001_00001,
            0b00010_00010_00010_00010_00010,
            0b00100_00100_00100_00100_00100,
            0b01000_01000_01000_01000_01000,
            0b10000_10000_10000_10000_10000,
            0b10000_01000_00100_00010_00001,
            0b00001_00010_00100_01000_10000,
        ];

        let mut ready = 0;
        let mut complete = 0;

        for line in LINES {
            match (self.marked & line).count_ones() {
                4 => ready |= line,
                5 => complete |= line,
                _ => (),
            }
        }

        (ready, complete)
    }

    /// Marks squares having the number.
    pub fn mark(&mut self, number: u8) {
        for (index, &square) in self.numbers.iter().enumerate() {
            if square == number {
                self.marked |= 1 << index;
            }
        }
    }
}

/// A caller.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Caller<const N: usize> {
    numbers: [u8; N],
    next: usize,
}

impl<const N: usize> Caller<N> {
    /// Creates a new caller.
    ///
    /// # Panics
    ///
    /// Panics if `N` is [`usize::MAX`] or `next` is greater than `N`.
    pub const fn new(numbers: [u8; N], next: usize) -> Self {
        if N == usize::MAX {
            panic!("`N` must not be `usize::MAX`")
        } else if next > N {
            panic!("`next` must be `N` or less")
        } else {
            Self { numbers, next }
        }
    }

    /// Returns the numbers.
    pub const fn numbers(&self) -> &[u8; N] {
        &self.numbers
    }

    /// Returns an index of the next number.
    pub const fn next(&self) -> usize {
        self.next
    }

    /// Returns the history.
    pub fn history(&self) -> &[u8] {
        &self.numbers[0..self.next]
    }

    /// Calls a number.
    pub fn call(&mut self) -> Option<u8> {
        match self.next == N {
            true => Option::None,
            false => {
                let number = self.numbers[self.next];
                self.next += 1;
                Option::Some(number)
            },
        }
    }
}
