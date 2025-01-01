use crate::block::{AlignedBitfield, BlockFetch, BlockStore, DefaultValue, IndexedBlock, SparseVec};

/// Implementation of a sparse bitset.
#[derive(Default)]
pub struct SparseBitset<T> 
{
    bitset: SparseVec<AlignedBitfield<T>,DefaultValue>
}

impl<T> IndexedBlock for SparseBitset<T>
where
AlignedBitfield<T>: IndexedBlock,
<AlignedBitfield<T> as IndexedBlock>::Item: Default,
{
    type Index = T;

    type Item = bool;
}

impl BlockFetch for SparseBitset<usize>
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        self.bitset.fetch(index)
    }
}

impl BlockStore for SparseBitset<usize>
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        self.bitset.store(index,item);
    }
}

impl BlockFetch for SparseBitset<u64>
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        self.bitset.fetch(index)
    }
}

impl BlockStore for SparseBitset<u64>
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        self.bitset.store(index,item);
    }
}

impl BlockFetch for SparseBitset<u128>
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        self.bitset.fetch(index)
    }
}

impl BlockStore for SparseBitset<u128>
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        self.bitset.store(index,item);
    }
}

#[cfg(test)]
mod test {
    use crate::block::{BlockFetch, BlockStore};

    use super::SparseBitset;

    #[test]
    fn test_fetch_store_usize() {
        let mut bs: SparseBitset<usize> = SparseBitset::default();
        bs.store(221,true);

        assert_eq!(bs.fetch(220),false);
        assert_eq!(bs.fetch(221),true);
    }

    #[test]
    fn test_fetch_store_u64() {
        let mut bs: SparseBitset<u64> = SparseBitset::default();
        bs.store(221,true);

        assert_eq!(bs.fetch(220),false);
        assert_eq!(bs.fetch(221),true);
    }

    #[test]
    fn test_fetch_store_u128() {
        let mut bs: SparseBitset<u128> = SparseBitset::default();
        bs.store(221,true);

        assert_eq!(bs.fetch(220),false);
        assert_eq!(bs.fetch(221),true);
    }
}