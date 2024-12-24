use super::{AlignedBlock, BlockFetch, BlockStore, DefaultPerIndex, IndexedBlock};

/// A vector of items that are themselves AlignedBlocks.
pub struct DenseVec<T> {
    vec: Vec<T>,
}

impl<T> DenseVec<T>
where
    T: AlignedBlock<Index = usize>,
{
    /// Construct a new DenseVec from an existing Vec.
    /// This is a fast operation.
    pub fn new_from(vec: Vec<T>) -> Self {
        DenseVec { vec }.assert_well_formed()
    }

    /// Unwrap a DenseVec back into a Vec.
    /// This is a fast operation.
    pub fn into_vec(self) -> Vec<T> {
        self.vec
    }

    /// Validate that a DenseVec is well-formed.
    /// Each block within a DenseVec must be densely packed, consecutive, and properly aligned.
    /// It should not be possible to construct a poorly-formed DenseVec.
    fn assert_well_formed(self) -> Self {
        for (i, value) in self.vec.iter().enumerate() {
            assert_eq!(
                i * T::alignment(),
                value.position(),
                "blocks must be densely packed and aligned"
            );
        }
        self
    }

    /// Given an index, calculate the index of the containing AlignedBlock.
    fn index_of(&self, index: usize) -> usize {
        index / T::alignment()
    }

    /// Push an entire block onto the end of this DenseVec.
    pub fn push_block(mut self, t: T) -> Self {
        self.vec.push(t);
        self.assert_well_formed()
    }

    /// Pop an entire block off of the end of this DenseVec.
    pub fn pop_block(mut self) -> (Self, Option<T>) {
        let result = self.vec.pop();
        (self, result)
    }
}

impl<T> IndexedBlock for DenseVec<T>
where
    T: AlignedBlock<Index = usize>,
{
    type Index = usize;
    type Item = T::Item;
}

impl<T> BlockFetch for DenseVec<T>
where
    T: AlignedBlock<Index = usize> + BlockFetch,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        let big = self.index_of(index);
        self.vec[big].fetch(index)
    }
}

impl<T> BlockStore for DenseVec<T>
where
    T: AlignedBlock<Index = usize> + BlockStore,
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        let big = self.index_of(index);
        self.vec[big].store(index, item);
    }
}

impl<T> DefaultPerIndex<T::Index, Option<T::Item>> for DenseVec<T>
where
    T: AlignedBlock<Index = usize> + BlockFetch,
{
    fn default_at_index(&self, i: T::Index) -> Option<T::Item> {
        if self.index_of(i) < self.vec.len() {
            Some(self.fetch(i))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::DenseVec;
    use crate::block::{AlignedVec, BlockFetch, BlockStore};

    #[test]
    pub fn test_from_into_vec() {
        let v: Vec<(usize, &str)> = vec![(0, "hello"), (1, "world"), (2, "!")];
        let vv = DenseVec::new_from(v.clone());
        let vvv = vv.into_vec();

        assert_eq!(v, vvv);
    }

    #[test]
    #[should_panic(expected = "blocks must be densely packed and aligned")]
    pub fn test_construct_malformed() {
        let v: Vec<(usize, &str)> = vec![(0, "hello"), (3, "world"), (2, "!")];
        DenseVec::new_from(v.clone());
    }

    #[test]
    pub fn test_push_into_empty() {
        let v: DenseVec<(usize, &str)> = DenseVec::new_from(vec![]);
        let v = v.push_block((0, "hello"));
        let v = v.push_block((1, "world"));
        let v = v.push_block((2, "."));

        let (v, a) = v.pop_block();
        assert_eq!(a, Some((2, ".")));
        let (v, b) = v.pop_block();
        assert_eq!(b, Some((1, "world")));
        let (v, c) = v.pop_block();
        assert_eq!(c, Some((0, "hello")));

        let (_v, empty) = v.pop_block();
        assert_eq!(empty, None);
    }

    #[test]
    #[should_panic(expected = "blocks must be densely packed and aligned")]
    pub fn test_bad_push() {
        let v: DenseVec<(usize, &str)> = DenseVec::new_from(vec![]);
        let _ = v.push_block((1, "hello"));
    }

    #[test]
    pub fn test_set_and_get() {
        let v: DenseVec<AlignedVec<i16, 16>> = DenseVec::new_from(vec![]);
        let v = v.push_block(AlignedVec::new_from(
            0,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        ));
        let mut v = v.push_block(AlignedVec::new_from(
            16,
            vec![
                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
        ));

        assert_eq!(v.fetch(7), 7);
        assert_eq!(v.fetch(17), 17);

        v.store(17, 1070);
        v.store(0, 1000);
        v.store(7, 1007);
        v.store(15, 1015);
        v.store(31, 1031);
        v.store(16, 1016);

        assert_eq!(v.fetch(17), 1070);
        assert_eq!(v.fetch(0), 1000);
        assert_eq!(v.fetch(7), 1007);
        assert_eq!(v.fetch(15), 1015);
        assert_eq!(v.fetch(31), 1031);
        assert_eq!(v.fetch(16), 1016);
    }
}
