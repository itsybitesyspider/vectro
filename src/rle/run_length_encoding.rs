use smallvec::SmallVec;
use crate::rle::instruction::RleInstruction;

use super::{decode_consecutive_runs::DecodeConsecutiveRuns, iterator::RleRunIterator, EncodeConsecutiveRuns};

/// A run-length-encoded vector.
pub struct Rle<T> {
    pub(super) values: SmallVec<[T;1]>,
    pub(super) lengths: SmallVec<[RleInstruction;std::mem::size_of::<usize>()]>,
}

impl<Value> Rle<Value>  {
    /// Push a run with the given value and run-length.
    pub fn push_run(&mut self, run: (Value,u128)) {
        self.values.push(run.0);
        self.lengths.extend(RleInstruction::new_run(run.1));
    }

    /// Pop the last run and return it.
    pub fn pop_run(&mut self) -> Option<(Value,u128)> {
        let mut length = 0;

        let mut instruction = self.lengths.pop()?;
        length += instruction.unpack();

        while !instruction.is_next_value() {
            instruction = self.lengths.pop().expect("when removing the trailing rle instructions, we should always quickly find a next-value marker");
            length += instruction.unpack();
        }

        let value = self.values.pop().expect("if next-value marker is present, a value should also be present.");
        return Some((value,length))
    }

    /// Push a run with the given value and run-length.
    /// If the run is an extension of the run that is already trailing, simply grows the trailing run.
    /// Always prefer this over `push_run` if the element value is Eq.
    pub fn append_run(&mut self, run: (Value,u128))
    where Value: Eq 
    {
        if Some(&run.0) == self.values.last() {
            let mut trailing = self.pop_run().expect("if any values are present, a trailing run should also be present");
            trailing.1 += run.1;
            self.push_run(trailing);
        } else {
            self.push_run(run);
        }
    }

    /// Extend this vector with the given elements.
    pub fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item=Value>,
          Value: Eq
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
    pub fn iterator(&self) -> DecodeConsecutiveRuns<RleRunIterator<Value>,&Value> {
        DecodeConsecutiveRuns::new(self.run_iterator())
    }
}