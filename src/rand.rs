use {
    crate::{
        Caller,
        Card,
    },
    core::array::from_fn as array,
    rand::{
        distributions::{
            Distribution,
            Standard,
        },
        seq::SliceRandom as _,
        Rng,
    },
};

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
        Card::new(numbers, 1 << 12)
    }
}

impl<const N: usize> Distribution<Caller<N>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Caller<N> {
        let mut numbers = array(|index| index as u8 + 1);
        numbers.shuffle(rng);
        Caller::new(numbers, 0)
    }
}
