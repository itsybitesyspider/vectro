/// An iterator over the elements of an `Rle``. It unpacks runs into multiple repetitions of a value.
/// This iterator is used specifically to iterate over `Rle`s, but it can also unpack any iterator of runs (elements of type (Value,u128)) into an iterator of one-at-a-time values.
pub struct DecodeConsecutiveRuns<Iter,Value>
where Iter:Iterator<Item=(Value,u128)>
{
    iter: Iter,
    current_run: Option<(Value,u128)>,
}

impl<Iter,Value> DecodeConsecutiveRuns<Iter,Value>
where Iter:Iterator<Item=(Value,u128)>
{
    /// Wrap an iterator to decode it.
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            current_run: None,
        }
    }
}

impl<Iter,Value> Iterator for DecodeConsecutiveRuns<Iter,Value> 
where Iter:Iterator<Item=(Value,u128)>,
      Value: Copy
{
    type Item=Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_run = self.current_run.or_else(|| self.iter.next());

        let (next_run, result) = match self.current_run {
            None => (None,None),
            Some((value,1)) => (None,Some(value)),
            Some((value,n)) => (Some((value,n-1)),Some(value))
        };

        self.current_run = next_run;
        result
    }
}