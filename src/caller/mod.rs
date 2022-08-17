#[cfg(feature = "rand")]
mod rand;

/// A bingo caller.
pub trait Caller {
    /// Calls a number.
    /// Returns [`None`](Option::None) if all numbers have already been called.
    fn call(&mut self) -> Option<u8>;

    /// Returns the numbers already called in the order they were called.
    fn history(&self) -> &[u8];
}

#[derive(Clone, Copy, Debug)]
/// A caller using an array.
pub struct ArrayCaller<const N: usize> {
    /// Numbers stored in the order they will be called.
    numbers: [u8; N],
    /// An index of the next called number.
    next: usize,
}

impl<const N: usize> ArrayCaller<N> {
    pub const fn new(numbers: [u8; N]) -> Self {
        Self { numbers, next: 0 }
    }
}

impl<const N: usize> Caller for ArrayCaller<N> {
    fn call(&mut self) -> Option<u8> {
        match self.next == N {
            true => Option::None,
            false => {
                let result = self.numbers[self.next];
                self.next += 1;
                Option::Some(result)
            },
        }
    }

    fn history(&self) -> &[u8] {
        &self.numbers[0..self.next]
    }
}
