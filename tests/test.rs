use {
    bingo::{
        Caller,
        Card,
    },
    rand::{
        thread_rng,
        Rng as _,
    },
    std::convert::identity,
};

/// Tests [`Card::info`].
#[test]
fn test1() {
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
        assert!(Card::new([0; 25], marked).info() == (ready, complete));
    }
}

/// Tests generation of cards.
#[test]
fn test2() {
    let mut rng = thread_rng();

    for _ in 0..0xFF {
        let card: Card = rng.gen();

        for (column, numbers) in card.numbers().chunks_exact(5).enumerate() {
            let mut generated = [false; 15];

            for (row, &number) in numbers.iter().enumerate() {
                let mask = 1 << (5 * column + row);

                match number {
                    0 => {
                        assert!(column == 2);
                        assert!(row == 2);
                        assert!(card.marked() & mask == mask);
                    },
                    _ => {
                        let index = number as usize - 15 * column - 1;
                        assert!(card.marked() & mask == 0);
                        assert!(!generated[index]);
                        generated[index] = true;
                    },
                }
            }
        }
    }
}

/// Tests generation of callers.
#[test]
fn test3() {
    let mut rng = thread_rng();

    for _ in 0..0xFF {
        let mut caller: Caller<75> = rng.gen();
        let mut generated = [false; 75];

        while let Option::Some(number) = caller.call() {
            let index = number as usize - 1;
            assert!(!generated[index]);
            generated[index] = true;
        }

        assert!(generated.into_iter().all(identity));
    }
}
