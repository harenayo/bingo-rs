use super::Card;
use rand::{
    thread_rng,
    Rng as _,
};

/// Tests whether cards are generated correctly.
#[test]
fn test() {
    let mut rng = thread_rng();

    for _ in 0..0xFF {
        let card: Card = rng.gen();

        for (column, numbers) in card.numbers.chunks_exact(5).enumerate() {
            let mut generated = [false; 15];

            for (row, &number) in numbers.iter().enumerate() {
                let mask = 1 << (5 * column + row);

                match number {
                    0 => {
                        assert!(column == 2);
                        assert!(row == 2);
                        assert!(card.marked & mask == mask);
                    },
                    _ => {
                        let index = number as usize - 15 * column - 1;
                        assert!(card.marked & mask == 0);
                        assert!(!generated[index]);
                        generated[index] = true;
                    },
                }
            }
        }
    }
}
