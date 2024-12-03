/// An iterator over the elements of an `Rle``. It unpacks runs into multiple repetitions of a value.
/// This iterator is used specifically to iterate over `Rle`s, but it can also unpack any iterator of runs (elements of type (Value,u128)) into an iterator of one-at-a-time values.
pub struct DecodeConsecutiveRuns<Iter, Value>
where
    Iter: Iterator<Item = (Value, u128)>,
{
    iter: Iter,
    current_run: Option<(Value, u128)>,
}

impl<Iter, Value> DecodeConsecutiveRuns<Iter, Value>
where
    Iter: Iterator<Item = (Value, u128)>,
{
    /// Wrap an iterator to decode it.
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            current_run: None,
        }
    }
}

impl<Iter, Value> Iterator for DecodeConsecutiveRuns<Iter, Value>
where
    Iter: Iterator<Item = (Value, u128)>,
    Value: Copy,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_run = self.current_run.or_else(|| self.iter.next());

        let (next_run, result) = match self.current_run {
            None => (None, None),
            Some((value, 1)) => (None, Some(value)),
            Some((value, n)) => (Some((value, n - 1)), Some(value)),
        };

        self.current_run = next_run;
        result
    }
}

#[cfg(test)]
mod test {
    use super::DecodeConsecutiveRuns;

    #[test]
    fn test_empty() {
        let empty: [(i32, u128); 0] = [];
        let iter = DecodeConsecutiveRuns::new(empty.iter().copied());
        let result: Vec<_> = iter.collect();

        assert_eq!(&result, &[]);
    }

    #[test]
    fn test_single() {
        let runs: [(i32, u128); 1] = [(42, 1)];
        let iter = DecodeConsecutiveRuns::new(runs.iter().copied());
        let result: Vec<_> = iter.collect();

        assert_eq!(&result, &[42]);
    }

    #[test]
    fn test_single_run() {
        let runs: [(i32, u128); 1] = [(42, 12)];
        let iter = DecodeConsecutiveRuns::new(runs.iter().copied());
        let result: Vec<_> = iter.collect();

        assert_eq!(&result, &[42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_twin_runs() {
        let runs: [(i32, u128); 2] = [(42, 2), (7, 10)];
        let iter = DecodeConsecutiveRuns::new(runs.iter().copied());
        let result: Vec<_> = iter.collect();

        assert_eq!(&result, &[42, 42, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_extremely_large_run() {
        let runs: [(i32, u128); 3] = [(1, 1), (2, 7000000), (3, 1)];
        let mut iter = DecodeConsecutiveRuns::new(runs.iter().copied());

        assert_eq!(iter.next(), Some(1));
        for _ in 0_i64..7000000 {
            assert_eq!(iter.next(), Some(2));
        }
        assert_eq!(iter.next(), Some(3));
    }
}
