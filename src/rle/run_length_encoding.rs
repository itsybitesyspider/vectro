use crate::rle::instruction::RleInstruction;
use smallvec::{smallvec, SmallVec};

use super::{
    decode_consecutive_runs::DecodeConsecutiveRuns, iterator::RleRunIterator, EncodeConsecutiveRuns,
};

/// A run-length-encoded vector.
pub struct Rle<T> {
    pub(super) values: SmallVec<[T; 1]>,
    pub(super) lengths: SmallVec<[RleInstruction; std::mem::size_of::<usize>()]>,
}

impl<Value> Default for Rle<Value> {
    fn default() -> Self {
        Rle {
            values: smallvec![],
            lengths: smallvec![],
        }
    }
}

impl<Value> Rle<Value> {
    /// Push a run with the given value and run-length.
    pub fn push_run(&mut self, run: (Value, u128)) {
        self.values.push(run.0);
        self.lengths.extend(RleInstruction::pack(run.1));
    }

    /// Pop the last run and return it.
    pub fn pop_run(&mut self) -> Option<(Value, u128)> {
        let mut length = 0;

        let mut instruction = self.lengths.pop()?;
        while !instruction.is_next_value() {
            length += instruction.unpack();
            instruction = self.lengths.pop().expect("when removing the trailing rle instructions, we should always quickly find a next-value marker");
        }

        let value = self
            .values
            .pop()
            .expect("if next-value marker is present, a value should also be present.");
        return Some((value, length));
    }

    /// Push a run with the given value and run-length.
    /// If the run is an extension of the run that is already trailing, simply grows the trailing run.
    /// Always prefer this over `push_run` if the element value is Eq.
    pub fn append_run(&mut self, run: (Value, u128))
    where
        Value: Eq,
    {
        if Some(&run.0) == self.values.last() {
            let mut trailing = self
                .pop_run()
                .expect("if any values are present, a trailing run should also be present");
            trailing.1 += run.1;
            self.push_run(trailing);
        } else {
            self.push_run(run);
        }
    }

    /// Extend this vector with the given elements.
    pub fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Value>,
        Value: Eq,
    {
        let iter = EncodeConsecutiveRuns::new(iter.into_iter());
        for item in iter {
            self.append_run(item);
        }
    }

    /// Iterate over every run (get an iterator that returns a single (value,length) pair for each run).
    pub fn run_iterator(&self) -> RleRunIterator<Value> {
        RleRunIterator::new(&self)
    }

    /// Iterate over every value (get an iterator that returns values, with each value repeated as many times as necessary to complete its run).
    pub fn iterator(&self) -> DecodeConsecutiveRuns<RleRunIterator<Value>, &Value> {
        DecodeConsecutiveRuns::new(self.run_iterator())
    }
}

#[cfg(test)]
mod test {
    use super::Rle;
    use proptest::prelude::*;

    #[test]
    pub fn test_push_pop() {
        let mut rle = Rle::default();
        rle.push_run(("a", 50));
        rle.push_run(("b", 25));

        assert_eq!(rle.pop_run(), Some(("b", 25)));
        assert_eq!(rle.pop_run(), Some(("a", 50)));
        assert_eq!(rle.pop_run(), None);
    }

    #[test]
    pub fn test_append_pop() {
        let mut rle = Rle::default();
        rle.append_run(("a", 50));
        rle.append_run(("b", 15));
        rle.append_run(("b", 10));

        assert_eq!(rle.pop_run(), Some(("b", 25)));
        assert_eq!(rle.pop_run(), Some(("a", 50)));
        assert_eq!(rle.pop_run(), None);
    }

    #[test]
    pub fn test_append_pop_large() {
        let mut rle = Rle::default();
        rle.append_run(("a", 50_000_000));
        rle.append_run(("b", 15_000_000));
        rle.append_run(("b", 10_000_000));

        assert_eq!(rle.pop_run(), Some(("b", 25_000_000)));
        assert_eq!(rle.pop_run(), Some(("a", 50_000_000)));
        assert_eq!(rle.pop_run(), None);
    }

    #[test]
    pub fn test_extend_pop() {
        let mut rle = Rle::default();
        rle.extend(vec![
            "a", "a", "a", "a", "a", "a", "b", "a", "a", "a", "b", "b", "b",
        ]);
        assert_eq!(
            vec![("a", 6), ("b", 1), ("a", 3), ("b", 3)],
            rle.run_iterator().map(|(v, n)| (*v, n)).collect::<Vec<_>>()
        );
    }

    proptest! {
        #[test]
        fn test_iterator_u8(original: Vec<u8>) {
            let mut rle = Rle::default();
            rle.extend(original.iter().copied());
            assert_eq!(original, rle.iterator().copied().collect::<Vec<_>>());
        }

        #[test]
        fn test_iterator_boolean(original: Vec<bool>) {
            let mut rle = Rle::default();
            rle.extend(original.iter().copied());
            assert_eq!(original, rle.iterator().copied().collect::<Vec<_>>());
        }
    }
}
