use super::aligned_block::{AlignedBlock, BlockGet, BlockSet};

/// A kind of AlignedBlock that is just a single item at a single index.
pub struct Singleton<Index,Item>(Index,Item);

impl<Item> AlignedBlock for Singleton<usize,Item> {
    type Index = usize;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> AlignedBlock for Singleton<u16,Item> {
    type Index = u16;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> AlignedBlock for Singleton<u32,Item> {
    type Index = u32;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0.clone() }
}

impl<Item> AlignedBlock for Singleton<u64,Item> {
    type Index = u64;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> AlignedBlock for Singleton<u128,Item> {
    type Index = u128;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> BlockGet for Singleton<usize,Item>
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for Singleton<u16,Item>
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for Singleton<u32,Item>
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for Singleton<u64,Item>
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for Singleton<u128,Item>
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockSet for Singleton<usize,Item>
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for Singleton<u16,Item>
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for Singleton<u32,Item>
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for Singleton<u64,Item>
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for Singleton<u128,Item>
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}
