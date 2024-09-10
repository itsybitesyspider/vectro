use super::aligned_block::{AlignedBlock, BlockGet, BlockSet};

impl<Item> AlignedBlock for (usize,Item) {
    type Index = usize;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> AlignedBlock for (u16,Item) {
    type Index = u16;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> AlignedBlock for (u32,Item) {
    type Index = u32;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0.clone() }
}

impl<Item> AlignedBlock for (u64,Item) {
    type Index = u64;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> AlignedBlock for (u128,Item) {
    type Index = u128;
    type Item = Item;

    fn alignment() -> Self::Index { 1 }
    fn position(&self) -> Self::Index { self.0 }
}

impl<Item> BlockGet for (usize,Item)
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for (u16,Item)
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for (u32,Item)
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for (u64,Item)
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockGet for (u128,Item)
where Item: Copy
{
    fn get(&self, index: Self::Index) -> Self::Item { 
        assert_eq!(index,self.0);
        self.1
    }
}

impl<Item> BlockSet for (usize,Item)
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for (u16,Item)
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for (u32,Item)
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for (u64,Item)
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

impl<Item> BlockSet for (u128,Item)
{
    fn set(&mut self, index: Self::Index, item: Self::Item) { 
        assert_eq!(index,self.0);
        self.1 = item;
    }
}

#[cfg(test)]
mod test {
    use crate::block::{AlignedBlock, BlockGet, BlockSet};

    #[test]
    pub fn test_singleton_usize() {
        let mut x: (usize,&str) = (500,"hello");

        assert_eq!(<(usize,&str)>::alignment(),1);
        assert_eq!(x.position(),500);
        assert_eq!(x.get(500),"hello");
        x.set(500,"world");
        assert_eq!(x.get(500),"world");
    }

    #[test]
    pub fn test_singleton_u16() {
        let mut x: (u16,&str) = (500,"hello");

        assert_eq!(<(u16,&str)>::alignment(),1);
        assert_eq!(x.position(),500);
        assert_eq!(x.get(500),"hello");
        x.set(500,"world");
        assert_eq!(x.get(500),"world");
    }

    #[test]
    pub fn test_singleton_u32() {
        let mut x: (u32,&str) = (500,"hello");

        assert_eq!(<(u32,&str)>::alignment(),1);
        assert_eq!(x.position(),500);
        assert_eq!(x.get(500),"hello");
        x.set(500,"world");
        assert_eq!(x.get(500),"world");
    }

    #[test]
    pub fn test_singleton_u64() {
        let mut x: (u64,&str) = (500,"hello");

        assert_eq!(<(u64,&str)>::alignment(),1);
        assert_eq!(x.position(),500);
        assert_eq!(x.get(500),"hello");
        x.set(500,"world");
        assert_eq!(x.get(500),"world");
    }

    #[test]
    pub fn test_singleton_u128() {
        let mut x: (u128,&str) = (500,"hello");

        assert_eq!(<(u128,&str)>::alignment(),1);
        assert_eq!(x.position(),500);
        assert_eq!(x.get(500),"hello");
        x.set(500,"world");
        assert_eq!(x.get(500),"world");
    }
}
