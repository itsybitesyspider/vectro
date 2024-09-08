use super::Rle;

/// An iterator over the runs of a Rle. It returns one result per run.
pub struct RleRunIterator<'a,Value> {
    rle: &'a Rle<Value>,
    value_idx: usize,
    length_idx: usize,
}

impl<'a,Value> RleRunIterator<'a,Value> {
    pub(super) fn new(rle: &'a Rle<Value>) -> Self {
        RleRunIterator {
            rle,
            value_idx: 0,
            length_idx: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.length_idx >= self.rle.lengths.len()
    }

    fn is_next_value(&self) -> bool {
        self.rle.lengths[self.length_idx].is_next_value()
    }
}

impl<'a,Value> Iterator for RleRunIterator<'a,Value> {
    type Item=(&'a Value,u128);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }

        assert!(self.is_next_value(),"should always begin a run on a next-value token");
        self.length_idx += 1;

        let value = &self.rle.values[self.value_idx];
        let mut length = 0;

        while !self.is_done() && !self.is_next_value() {
            length += self.rle.lengths[self.length_idx].unpack();
            self.length_idx += 1;
        }

        Some((value,length))
    }
}