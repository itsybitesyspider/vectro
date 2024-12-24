use std::sync::Arc;

use super::{
    AlignedBlock, AlignedBlockFromIterator, BlockFetch, BlockStore, DefaultPerIndex, IndexedBlock,
};

impl<B> IndexedBlock for Arc<B>
where
    B: IndexedBlock,
{
    type Index = B::Index;
    type Item = B::Item;
}

impl<B> AlignedBlock for Arc<B>
where
    B: AlignedBlock,
{
    fn alignment() -> Self::Index {
        B::alignment()
    }

    fn position(&self) -> Self::Index {
        self.as_ref().position()
    }
}

impl<B> BlockFetch for Arc<B>
where
    B: BlockFetch,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        self.as_ref().fetch(index)
    }
}

impl<B> BlockStore for Arc<B>
where
    B: BlockStore,
    B: Clone,
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        Arc::make_mut(self).store(index, item);
    }
}

impl<B> AlignedBlockFromIterator for Arc<B>
where
    B: AlignedBlockFromIterator,
{
    fn from_iterator<I>(position: Self::Index, iter: &mut I) -> Self
    where
        I: Iterator<Item = Self::Item>,
    {
        Arc::new(B::from_iterator(position, iter))
    }
}

impl<D, Index, Item> DefaultPerIndex<Index, Item> for Arc<D>
where
    D: DefaultPerIndex<Index, Item>,
{
    fn default_at_index(&self, i: Index) -> Item {
        Arc::as_ref(self).default_at_index(i)
    }
}
