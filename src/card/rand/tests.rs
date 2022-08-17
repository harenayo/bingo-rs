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
        for (column, numbers) in rng.gen::<Card>().numbers.chunks_exact(5).enumerate() {
            let base = 15 * column;
            let mut generated = [false; 15];

            for &number in numbers {
                match number {
                    0 => (),
                    _ => {
                        let index = number as usize - 1;
                        assert!((base..base + 15).contains(&index));
                        assert!(!generated[index - base]);
                        generated[index - base] = true;
                    },
                }
            }
        }
    }
}
