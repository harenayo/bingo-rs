//! A bingo library.
//!
//! # Data Structures
//!
//! Cards are represented by `[u8; 25]` and squares are indexed as:
//!
//! |  b  |  i  |  n  |  g  |  o  |
//! | :-: | :-: | :-: | :-: | :-: |
//! |  0  |  5  |  10 |  15 |  20 |
//! |  1  |  6  |  11 |  16 |  21 |
//! |  2  |  7  |  12 |  17 |  22 |
//! |  3  |  8  |  13 |  18 |  23 |
//! |  4  |  9  |  14 |  19 |  24 |
//!
//! This also applies to `u32` bit flags which represent information of a card.

#[cfg(feature = "rand")]
use {
    rand::{
        seq::SliceRandom as _,
        Rng,
    },
    std::array::from_fn as array,
};

/// Marks squares having the number.
pub fn mark(numbers: &[u8; 25], marked: &mut u32, number: u8) {
    for (index, &square) in numbers.iter().enumerate() {
        if square == number {
            *marked |= 1 << index;
        }
    }
}

/// Returns two sets of bit flags: `ready` and `complete`.
///
/// | name | description |
/// | :-: | - |
/// | `ready` | Squares in a row or a column which have four marked squares |
/// | `complete` | Squares in a row or a column which have five marked squares |
pub fn info(marked: u32) -> (u32, u32) {
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
        match (marked & line).count_ones() {
            4 => ready |= line,
            5 => complete |= line,
            _ => (),
        }
    }

    (ready, complete)
}

/// Creates a card at random and returns a tuple of numbers and bit flags representing marked squares.
///
/// Each column has values in a range:
///
/// | column | range |
/// | :-: | :-: |
/// | b | `1..=15` |
/// | i | `16..=30` |
/// | n | `31..=45` |
/// | g | `46..=60` |
/// | o | `61..=75` |
///
/// But the center is always `0` and marked.
#[cfg(feature = "rand")]
pub fn card<R: Rng + ?Sized>(rng: &mut R) -> ([u8; 25], u32) {
    let mut numbers = [0; 25];

    for (column, numbers) in numbers.chunks_exact_mut(5).enumerate() {
        numbers.copy_from_slice(
            array::<_, 15, _>(|index| (15 * column + index + 1) as u8)
                .partial_shuffle(rng, 5)
                .0,
        );
    }

    numbers[12] = 0;
    (numbers, 1 << 12)
}

/// Creates a caller at random; in other words, returns a shuffled array containing all integers between `1` and `75`.
#[cfg(feature = "rand")]
pub fn caller<R: Rng + ?Sized>(rng: &mut R) -> [u8; 75] {
    let mut numbers = array(|index| index as u8 + 1);
    numbers.shuffle(rng);
    numbers
}
