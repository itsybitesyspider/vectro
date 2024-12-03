use super::{aligned_block::{AlignedBlock, BlockFetch}, BlockStore, IndexedBlock};

/// An aligned block of booleans.
pub struct AlignedBitfield<T> {
    position: T,
    bits: T
}

impl IndexedBlock for AlignedBitfield<usize> {
    type Index = usize;
    type Item = bool;

}

impl AlignedBlock for AlignedBitfield<usize> {
    fn alignment() -> Self::Index {
        8*std::mem::size_of::<Self::Index>()
    }

    fn position(&self) -> Self::Index {
        self.position
    }
}

impl IndexedBlock for AlignedBitfield<u64> {
    type Index = u64;
    type Item = bool;
}

impl AlignedBlock for AlignedBitfield<u64> {
    fn alignment() -> Self::Index {
        8*std::mem::size_of::<Self::Index>() as u64
    }

    fn position(&self) -> Self::Index {
        self.position
    }
}

impl BlockFetch for AlignedBitfield<usize> {
    fn fetch(&self, index: Self::Index) -> bool {
        let index = index - self.position;
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        (self.bits >> index) & 0x01 != 0
    }
}

impl BlockFetch for AlignedBitfield<u64> {
    fn fetch(&self, index: Self::Index) -> bool {
        let index = index - self.position;
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        (self.bits >> index) & 0x01 != 0
    }
}

impl BlockStore for AlignedBitfield<usize> {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        let index = index - self.position;
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        if item {
            self.bits = self.bits | 0x01 << index;
        } else {
            self.bits = self.bits & !(0x01 << index);
        }
    }
}

impl BlockStore for AlignedBitfield<u64> {
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        let index = index - self.position;
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        if item {
            self.bits = self.bits | 0x01 << index;
        } else {
            self.bits = self.bits & !(0x01 << index);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::block::{AlignedBlock, BlockFetch, BlockStore};

    use super::AlignedBitfield;

    #[test]
    fn test_bitfield_usize() {

        assert_eq!(AlignedBitfield::<usize>::alignment(),8*std::mem::size_of::<usize>());
        
        let b = AlignedBitfield::<usize> { position: 128, bits: 0 };
        assert_eq!(b.position(), 128);
        assert_eq!(b.fetch(130), false);
    }

    #[test]
    fn test_bitfield_u64() {
        assert_eq!(AlignedBitfield::<u64>::alignment() as usize,8*std::mem::size_of::<u64>());
        
        let b = AlignedBitfield::<u64> { position: 128,bits: 0 };
        assert_eq!(b.position(), 128);
        assert_eq!(b.fetch(130), false);
    }

    #[test]
    fn test_bitfield_usize_set() {
        assert_eq!(AlignedBitfield::<usize>::alignment(),8*std::mem::size_of::<usize>());
        assert!(std::mem::size_of::<usize>() == 4 || std::mem::size_of::<usize>() == 8, "expecting either 32 bit or 64 bit usize");
        
        let mut b = AlignedBitfield::<usize>{ position: 128, bits: 0};
        b.store(130,true);
        b.store(128,true);
        b.store(140,true);
        b.store(128, false);

        if std::mem::size_of::<usize>() == 8 {
            b.store(191,true);
        }

        assert_eq!(b.fetch(128), false);
        assert_eq!(b.fetch(129), false);
        assert_eq!(b.fetch(130), true);
        assert_eq!(b.fetch(131), false);
        assert_eq!(b.fetch(135), false);
        assert_eq!(b.fetch(140), true);
        assert_eq!(b.fetch(145), false);

        if std::mem::size_of::<usize>() == 8 {
            assert_eq!(b.fetch(191), true);
        }
    }

    #[test]
    fn test_bitfield_u64_set() {
        assert_eq!(AlignedBitfield::<usize>::alignment(),64);
        
        let mut b = AlignedBitfield::<u64> { position: 128, bits: 0 };
        b.store(130,true);
        b.store(128,true);
        b.store(140,true);
        b.store(191,true);
        b.store(128, false);

        assert_eq!(b.fetch(128), false);
        assert_eq!(b.fetch(129), false);
        assert_eq!(b.fetch(130), true);
        assert_eq!(b.fetch(131), false);
        assert_eq!(b.fetch(135), false);
        assert_eq!(b.fetch(140), true);
        assert_eq!(b.fetch(145), false);
        assert_eq!(b.fetch(191), true);
    }
}