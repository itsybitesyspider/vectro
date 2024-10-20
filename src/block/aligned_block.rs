/// A block of data (an array) which has an alignment and a (starting) position.
/// The position must be a multiple of the alignment.
pub trait AlignedBlock {
    /// How the block is indexed (for example, by usize)
    type Index;
    /// Type of the item in the collection.
    type Item;

    /// The required alignemt for this kind of block.
    fn alignment() -> Self::Index;
    /// The position of the starting element of this block.
    fn position(&self) -> Self::Index;
}

/// A block where it is possible to get any individual element.
pub trait BlockGet : AlignedBlock {
    /// Get an element of a block.
    fn get(&self, index: Self::Index) -> Self::Item;
}

/// A block where it is possible to set any individual element.
pub trait BlockSet : AlignedBlock {
    /// Set an element of a block.
    fn set(&mut self, index: Self::Index, item: Self::Item);
}

