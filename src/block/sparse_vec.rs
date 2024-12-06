use crate::numerical_index::NumericalIndex;

use super::{AlignedBlock, AlignedDefault, BlockFetch, BlockStore, IndexedBlock, NewByIndex};

/// A vector of items that are themselves AlignedBlocks.
pub struct SparseVec<T: IndexedBlock, D: AlignedDefault<T::Index, T::Item>> {
    default_value: D,
    vec: Vec<T>,
}

impl<T, D> SparseVec<T, D>
where
    T: AlignedBlock,
    D: AlignedDefault<T::Index, T::Item>,
    T::Index: NumericalIndex,
{
    /// Construct a new SparseVec from an existing Vec.
    /// This is a fast operation.
    pub fn new_from(default_value: D, vec: Vec<T>) -> Self {
        SparseVec { default_value, vec }.assert_well_formed()
    }

    /// Unwrap a SparseVec back into a Vec.
    /// This is a fast operation.
    pub fn into_vec(self) -> Vec<T> {
        self.vec
    }

    /// Validate that a SparseVec is well-formed.
    /// Each block of the SparseVec must be aligned, uniquely-positioned, and in sorted order by position.
    fn assert_well_formed(self) -> Self {
        let mut running_minimum: Option<T::Index> = None;

        for value in self.vec.iter() {
            assert!(
                value.position().modulo(T::alignment()).is_zero(),
                "blocks must be aligned"
            );
            assert!(
                if let Some(x) = running_minimum {
                    value.position() > x
                } else {
                    true
                },
                "block positions must monotonically increase"
            );
            running_minimum = Some(value.position());
        }
        self
    }

    /// Given an index, calculate the index of the containing AlignedBlock.
    /// Behaves the same as binary_search_by_key.
    fn index_of(&self, index: T::Index) -> Result<usize, usize>
    where
        T::Index: NumericalIndex,
    {
        let index = index.block(T::alignment());
        let search_position = index.block(T::alignment());
        self.vec
            .binary_search_by_key(&search_position, |item| item.position())
    }

    /// Push an entire block onto the end of this DenseVec.
    pub fn push_block(mut self, t: T) -> Self
    where
        T::Index: NumericalIndex,
    {
        self.vec.push(t);
        self.assert_well_formed()
    }

    /// Pop an entire block off of the end of this DenseVec.
    pub fn pop_block(mut self) -> (Self, Option<T>) {
        let result = self.vec.pop();
        (self, result)
    }

    fn ensure_index_exists(&mut self, index: T::Index) -> usize
    where
        T: NewByIndex,
        T::Item: Default,
        T::Index: NumericalIndex,
    {
        let index = index.block(T::alignment());
        let big = self.index_of(index);
        match big {
            Ok(exists) => exists,
            Err(does_not_exist) => {
                self.vec.insert(
                    does_not_exist,
                    T::new_per_index(index, |_| T::Item::default()),
                );
                does_not_exist
            }
        }
    }
}

impl<T, D> IndexedBlock for SparseVec<T, D>
where
    T: AlignedBlock,
    D: AlignedDefault<T::Index, T::Item>,
{
    type Index = T::Index;
    type Item = T::Item;
}

impl<T, D> BlockFetch for SparseVec<T, D>
where
    T: AlignedBlock + BlockFetch,
    T::Index: NumericalIndex,
    T::Item: Copy,
    D: AlignedDefault<T::Index, T::Item>,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        let big = self.index_of(index);
        if let Ok(i) = big {
            self.vec[i].fetch(index)
        } else {
            self.default_value.default_at_index(index)
        }
    }
}

impl<T, D> BlockStore for SparseVec<T, D>
where
    T: AlignedBlock + BlockStore + NewByIndex,
    T::Index: NumericalIndex,
    T::Item: Default,
    D: AlignedDefault<T::Index, T::Item>,
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        let big = self.ensure_index_exists(index);
        self.vec[big].store(index, item);
    }
}

#[cfg(test)]
mod test {
    use crate::block::{AlignedVec, BlockFetch, BlockStore, DefaultValue};

    use super::SparseVec;

    #[test]
    fn test_to_from_vec() {
        let v: Vec<(u32, &str)> = vec![(3, "hello"), (7, "world"), (25, ".")];
        let vv = SparseVec::new_from(DefaultValue, v.clone());
        let vvv = vv.into_vec();

        assert_eq!(v, vvv);
    }

    #[test]
    #[should_panic(expected = "block positions must monotonically increase")]
    fn test_out_of_order() {
        let v: Vec<(u32, &str)> = vec![(3, "hello"), (7, "world"), (0, ".")];
        SparseVec::new_from(DefaultValue, v);
    }

    #[test]
    fn test_push_pop() {
        let v: SparseVec<AlignedVec<u64, 10>, DefaultValue> =
            SparseVec::new_from(DefaultValue, vec![]);
        let v = v.push_block(AlignedVec::new_from(0, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
        let v = v.push_block(AlignedVec::new_from(
            20,
            vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
        ));

        assert_eq!(v.fetch(0), 0);
        assert_eq!(v.fetch(9), 9);
        assert_eq!(v.fetch(10), 0);
        assert_eq!(v.fetch(15), 0);
        assert_eq!(v.fetch(19), 0);
        assert_eq!(v.fetch(20), 20);
        assert_eq!(v.fetch(29), 29);
        assert_eq!(v.fetch(30), 0);

        let (v, rest) = v.pop_block();
        assert_eq!(
            rest.map(|x| x.into_vec()),
            Some(vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29])
        );

        let (_, rest) = v.pop_block();
        assert_eq!(
            rest.map(|x| x.into_vec()),
            Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }

    #[test]
    fn test_sparse_store() {
        let mut v: SparseVec<AlignedVec<u64, 10>, DefaultValue> =
            SparseVec::new_from(DefaultValue, vec![]);
        v.store(25, 25);
    }
}
