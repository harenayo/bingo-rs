#[cfg(test)]
mod tests;

use super::ArrayCaller;
use rand::{
    distributions::{
        Distribution,
        Standard,
    },
    seq::SliceRandom as _,
    Rng,
};
use std::array::from_fn as array;

impl Distribution<ArrayCaller<75>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ArrayCaller<75> {
        let mut numbers = array(|index| index as u8 + 1);
        numbers.shuffle(rng);
        ArrayCaller::new(numbers)
    }
}
