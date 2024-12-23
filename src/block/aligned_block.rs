use crate::numerical_index::NumericalIndex;

/// A block of data (an array) which has an item type and index type.
pub trait IndexedBlock {
    /// How the block is indexed (for example, by usize)
    type Index;
    /// Type of the item in the collection.
    type Item;
}

/// A block of data (an array) which has an alignment and a (starting) position.
/// This is different from a normal vector which always begins at zero and has an arbitrary length.
/// The starting position must be a multiple of the alignment, and the length must also be exactly the alignment.
pub trait AlignedBlock: IndexedBlock {
    /// The required alignment for this kind of block.
    fn alignment() -> Self::Index;
    /// The position of the starting element of this block.
    fn position(&self) -> Self::Index;
}

/// A block where it is possible to construct a default value at a given position.s
pub trait AlignedBlockFromDefault: AlignedBlock {
    /// Construct a block of data with a default value and position.
    fn default_per_index(position: Self::Index, value: impl Fn(Self::Index) -> Self::Item) -> Self;
}

/// Construct default values for a given item at a given index.
pub trait DefaultPerIndex<Index, Item> {
    /// Determine the default value at the given index.
    fn default_at_index(&self, i: Index) -> Item;
}

/// A block where it is possible to get any individual element.
pub trait BlockFetch: IndexedBlock {
    /// Get an element of a block.
    fn fetch(&self, index: Self::Index) -> Self::Item;
}

/// A block where it is possible to set any individual element.
pub trait BlockStore: IndexedBlock {
    /// Set an element of a block.
    fn store(&mut self, index: Self::Index, item: Self::Item);
}

/// Aligned block that can be constructed from the leading elements of an iterator.
pub trait AlignedBlockFromIterator : AlignedBlock + Sized {
    /// Construct an AlignedBlock from a function that takes an index and returns a value.
    fn from_function(position: Self::Index, value: impl Fn(Self::Index) -> Self::Item) -> Self
    where Self::Index: NumericalIndex
    {
        Self::from_iterator(position,&mut (position.range(Self::alignment())).map(value))
    }

    /// Consume as many items from the iterator as needed to construct a new AlignedBlock.
    /// If there are not enough items in the iterator to construct the entire block, panic.
    fn from_iterator<I>(position: Self::Index, iter: &mut I) -> Self
    where I: Iterator<Item=Self::Item>;
}

/// Initialize an AlignedBlock using its Default impl.
pub struct DefaultValue;

impl<Index, Item: Default> DefaultPerIndex<Index, Item> for DefaultValue {
    fn default_at_index(&self, _: Index) -> Item {
        Item::default()
    }
}