/// A bingo card.
///
/// Each square has an index:
///
/// |  b  |  i  |  n  |  g  |  o  |
/// | :-: | :-: | :-: | :-: | :-: |
/// |  0  |  5  |  10 |  15 |  20 |
/// |  1  |  6  |  11 |  16 |  21 |
/// |  2  |  7  |  12 |  17 |  22 |
/// |  3  |  8  |  13 |  18 |  23 |
/// |  4  |  9  |  14 |  19 |  24 |
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    numbers: [u8; 25],
    marked: u32,
}

impl Card {
    /// Creates a new card.
    pub const fn new(numbers: [u8; 25], marked: u32) -> Self {
        Self { numbers, marked }
    }

    /// Marks squares having the number.
    pub fn mark(&mut self, number: u8) {
        for (index, &space) in self.numbers.iter().enumerate() {
            if space == number {
                self.marked |= 1 << index;
            }
        }
    }

    /// Returns an array of the numbers.
    pub const fn numbers(&self) -> &[u8; 25] {
        &self.numbers
    }

    /// Returns bit flags indicating whether the squares are marked.
    pub const fn marked(&self) -> u32 {
        self.marked
    }

    /// Returns two sets of bit flags: `ready` and `complete`.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests whether `ready` and `complete` are calculated correctly.
    #[test]
    fn test() {
        #[allow(clippy::unusual_byte_groupings)]
        const CASES: [(u32, u32, u32); 9] = [
            (
                0b00000_00000_00000_00000_00000,
                0b00000_00000_00000_00000_00000,
                0b00000_00000_00000_00000_00000,
            ),
            (
                0b00000_00000_00000_00000_11111,
                0b00000_00000_00000_00000_00000,
                0b00000_00000_00000_00000_11111,
            ),
            (
                0b00000_00000_00000_00000_01111,
                0b00000_00000_00000_00000_11111,
                0b00000_00000_00000_00000_00000,
            ),
            (
                0b00001_00001_00001_00001_00001,
                0b00000_00000_00000_00000_00000,
                0b00001_00001_00001_00001_00001,
            ),
            (
                0b00001_00001_00000_00001_00001,
                0b00001_00001_00001_00001_00001,
                0b00000_00000_00000_00000_00000,
            ),
            (
                0b10000_01000_00100_00010_00001,
                0b00000_00000_00000_00000_00000,
                0b10000_01000_00100_00010_00001,
            ),
            (
                0b00001_00010_00100_01000_10000,
                0b00000_00000_00000_00000_00000,
                0b00001_00010_00100_01000_10000,
            ),
            (
                0b10001_01010_00000_01010_10001,
                0b10001_01010_00100_01010_10001,
                0b00000_00000_00000_00000_00000,
            ),
            (
                0b11111_00001_00000_00001_00001,
                0b00001_00001_00001_00001_00001,
                0b11111_00000_00000_00000_00000,
            ),
        ];

        for (marked, ready, complete) in CASES {
            assert_eq!(Card::new([0; 25], marked).info(), (ready, complete));
        }
    }
}
