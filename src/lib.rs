//! A crate for bingo.

#[cfg(feature = "rand")]
use {
    rand::{
        distributions::{
            Distribution,
            Standard,
        },
        seq::SliceRandom as _,
        Rng,
    },
    std::array::from_fn as array,
};

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

#[cfg(feature = "rand")]
impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        let mut numbers = [0; 25];

        for (column, numbers) in numbers.chunks_exact_mut(5).enumerate() {
            numbers.copy_from_slice(
                array::<_, 15, _>(|index| (15 * column + index + 1) as u8)
                    .partial_shuffle(rng, 5)
                    .0,
            );
        }

        numbers[12] = 0;

        Card {
            numbers,
            marked: 1 << 12,
        }
    }
}

/// Creates a caller at random; in other words, returns a shuffled array containing all integers between `1` and `75`.
#[cfg(feature = "rand")]
pub fn caller<R: Rng + ?Sized>(rng: &mut R) -> [u8; 75] {
    let mut numbers = array(|index| index as u8 + 1);
    numbers.shuffle(rng);
    numbers
}
