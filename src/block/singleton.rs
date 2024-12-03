use super::{
    aligned_block::{AlignedBlock, BlockFetch, BlockStore},
    IndexedBlock,
};

impl<Item> IndexedBlock for (usize, Item) {
    type Index = usize;
    type Item = Item;
}

impl<Item> AlignedBlock for (usize, Item) {
    fn alignment() -> Self::Index {
        1
    }
    fn position(&self) -> Self::Index {
        self.0
    }
}

impl<Item> IndexedBlock for (u16, Item) {
    type Index = u16;
    type Item = Item;
}

impl<Item> AlignedBlock for (u16, Item) {
    fn alignment() -> Self::Index {
        1
    }
    fn position(&self) -> Self::Index {
        self.0
    }
}

impl<Item> IndexedBlock for (u32, Item) {
    type Index = u32;
    type Item = Item;
}

impl<Item> AlignedBlock for (u32, Item) {
    fn alignment() -> Self::Index {
        1
    }
    fn position(&self) -> Self::Index {
        self.0.clone()
    }
}

impl<Item> IndexedBlock for (u64, Item) {
    type Index = u64;
    type Item = Item;
}

impl<Item> AlignedBlock for (u64, Item) {
    fn alignment() -> Self::Index {
        1
    }
    fn position(&self) -> Self::Index {
        self.0
    }
}

impl<Item> IndexedBlock for (u128, Item) {
    type Index = u128;
    type Item = Item;
}

impl<Item> AlignedBlock for (u128, Item) {
    fn alignment() -> Self::Index {
        1
    }
    fn position(&self) -> Self::Index {
        self.0
    }
}

impl<Item> BlockFetch for (usize, Item)
where
    Item: Copy,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        assert_eq!(index, self.0);
        self.1
    }
}

impl<Item> BlockFetch for (u16, Item)
where
    Item: Copy,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        assert_eq!(index, self.0);
        self.1
    }
}

impl<Item> BlockFetch for (u32, Item)
where
    Item: Copy,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        assert_eq!(index, self.0);
        self.1
    }
}

impl<Item> BlockFetch for (u64, Item)
where
    Item: Copy,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        assert_eq!(index, self.0);
        self.1
    }
}

impl<Item> BlockFetch for (u128, Item)
where
    Item: Copy,
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        assert_eq!(index, self.0);
        self.1
    }
}

impl<Item> BlockStore for (usize, Item) {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        assert_eq!(index, self.0);
        self.1 = item;
    }
}

impl<Item> BlockStore for (u16, Item) {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        assert_eq!(index, self.0);
        self.1 = item;
    }
}

impl<Item> BlockStore for (u32, Item) {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        assert_eq!(index, self.0);
        self.1 = item;
    }
}

impl<Item> BlockStore for (u64, Item) {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        assert_eq!(index, self.0);
        self.1 = item;
    }
}

impl<Item> BlockStore for (u128, Item) {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        assert_eq!(index, self.0);
        self.1 = item;
    }
}

#[cfg(test)]
mod test {
    use crate::block::{AlignedBlock, BlockFetch, BlockStore};

    #[test]
    pub fn test_singleton_usize() {
        let mut x: (usize, &str) = (500, "hello");

        assert_eq!(<(usize, &str)>::alignment(), 1);
        assert_eq!(x.position(), 500);
        assert_eq!(x.fetch(500), "hello");
        x.store(500, "world");
        assert_eq!(x.fetch(500), "world");
    }

    #[test]
    pub fn test_singleton_u16() {
        let mut x: (u16, &str) = (500, "hello");

        assert_eq!(<(u16, &str)>::alignment(), 1);
        assert_eq!(x.position(), 500);
        assert_eq!(x.fetch(500), "hello");
        x.store(500, "world");
        assert_eq!(x.fetch(500), "world");
    }

    #[test]
    pub fn test_singleton_u32() {
        let mut x: (u32, &str) = (500, "hello");

        assert_eq!(<(u32, &str)>::alignment(), 1);
        assert_eq!(x.position(), 500);
        assert_eq!(x.fetch(500), "hello");
        x.store(500, "world");
        assert_eq!(x.fetch(500), "world");
    }

    #[test]
    pub fn test_singleton_u64() {
        let mut x: (u64, &str) = (500, "hello");

        assert_eq!(<(u64, &str)>::alignment(), 1);
        assert_eq!(x.position(), 500);
        assert_eq!(x.fetch(500), "hello");
        x.store(500, "world");
        assert_eq!(x.fetch(500), "world");
    }

    #[test]
    pub fn test_singleton_u128() {
        let mut x: (u128, &str) = (500, "hello");

        assert_eq!(<(u128, &str)>::alignment(), 1);
        assert_eq!(x.position(), 500);
        assert_eq!(x.fetch(500), "hello");
        x.store(500, "world");
        assert_eq!(x.fetch(500), "world");
    }
}
