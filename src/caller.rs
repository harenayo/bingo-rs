use std::ops::Deref;

/// A bingo caller.
pub trait Caller {
    /// Calls a number.
    /// Returns [`None`](Option::None) if all numbers have already been called.
    fn call(&mut self) -> Option<u8>;

    /// Returns the numbers already called in the order they were called.
    fn history(&self) -> &[u8];
}

/// The simplest caller.
#[derive(Clone, Copy, Debug)]
pub struct SimpleCaller<N> {
    /// Numbers stored in the order they will be called.
    numbers: N,
    /// An index of the next called number.
    next: usize,
}

impl<N> SimpleCaller<N> {
    /// Creates a new caller.
    pub const fn new(numbers: N) -> Self {
        Self { numbers, next: 0 }
    }
}

impl<N: Deref<Target = [u8]>> Caller for SimpleCaller<N> {
    fn call(&mut self) -> Option<u8> {
        if self.next == self.numbers.len() {
            Option::None
        } else {
            let result = self.numbers[self.next];
            self.next += 1;
            Option::Some(result)
        }
    }

    fn history(&self) -> &[u8] {
        &self.numbers[0..self.next]
    }
}
