use std::ops::Index;

use crate::numerical_index::NumericalIndex;

use super::{AlignedBlock, BlockFetch};

/// An iterator over the indices of an AlignedBlock (not values).
pub struct BlockIndexIterator<B> 
where B: AlignedBlock
{
    start_index: B::Index,
    next_index: B::Index,
}

impl<B> Iterator for BlockIndexIterator<B>
where B: AlignedBlock,
B::Index : NumericalIndex
{
    type Item = B::Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index.block(B::alignment()) == self.start_index.block(B::alignment()) {
            let i = self.next_index;
            self.next_index = self.next_index.next();
            Some(i)
        } else {
            None
        }

    }
}

impl<B> BlockIndexIterator<B>
where B: AlignedBlock
{
    /// Make a new BlockIterator for a block
    pub fn new<'a>(block: &'a B) -> Self {
        BlockIndexIterator {
            start_index: block.position(),
            next_index: block.position(),
        }
    }
}

/// An iterator over a BlockFetch.
pub struct BlockFetchIterator<'b, B> 
where B: AlignedBlock
{
    iter: BlockIndexIterator<B>,
    block: &'b B,
}

impl<'b, B> Iterator for BlockFetchIterator<'b, B>
where B: AlignedBlock + BlockFetch,
B::Index : NumericalIndex
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.block.fetch(self.iter.next()?))
    }
}

impl<'b,B> BlockFetchIterator<'b,B>
where B: AlignedBlock
{
    /// Make a new BlockIterator for a block
    pub fn new(block: &'b B) -> Self {
        BlockFetchIterator {
            iter: BlockIndexIterator::new(block),
            block,
        }
    }
}

/// An iterator over an AlignedBlock that is also Index.
pub struct BlockRefIterator<'b, B> 
where B: AlignedBlock + Index<B::Index,Output=B::Item>
{
    iter: BlockIndexIterator<B>,
    block: &'b B,
}

impl<'b, B> Iterator for BlockRefIterator<'b, B>
where B: AlignedBlock + Index<B::Index,Output=B::Item>,
B::Index : NumericalIndex
{
    type Item = &'b B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.block[self.iter.next()?])
    }
}

impl<'b,B> BlockRefIterator<'b,B>
where B: AlignedBlock + Index<B::Index,Output=B::Item>,
{
    /// Make a new BlockIterator for a block
    pub fn new(block: &'b B) -> Self {
        BlockRefIterator {
            iter: BlockIndexIterator::new(block),
            block,
        }
    }
}