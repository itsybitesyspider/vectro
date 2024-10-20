use super::{aligned_block::{AlignedBlock, BlockGet}, BlockSet};

/// An aligned block of booleans.
pub struct AlignedBitfield<T>((T,T));

impl AlignedBlock for AlignedBitfield<usize> {
    type Index = usize;
    type Item = bool;

    fn alignment() -> Self::Index {
        8*std::mem::size_of::<Self::Index>()
    }

    fn position(&self) -> Self::Index {
        self.0.position()
    }
}

impl AlignedBlock for AlignedBitfield<u64> {
    type Index = u64;
    type Item = bool;

    fn alignment() -> Self::Index {
        8*std::mem::size_of::<Self::Index>() as u64
    }

    fn position(&self) -> Self::Index {
        self.0.position()
    }
}

impl BlockGet for AlignedBitfield<usize> {
    fn get(&self, index: Self::Index) -> bool {
        let index = index - self.0.position();
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        (self.0.get(self.0.position()) >> index) & 0x01 != 0
    }
}

impl BlockGet for AlignedBitfield<u64> {
    fn get(&self, index: Self::Index) -> bool {
        let index = index - self.0.position();
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        (self.0.get(self.0.position()) >> index) & 0x01 != 0
    }
}

impl BlockSet for AlignedBitfield<usize> {
    fn set(&mut self, index: Self::Index, item: Self::Item) {
        let index = index - self.0.position();
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        if item {
            self.0.set(self.0.position(), self.0.get(self.0.position()) | 0x01 << index);
        } else {
            self.0.set(self.0.position(), self.0.get(self.0.position()) & !(0x01 << index));
        }
    }
}

impl BlockSet for AlignedBitfield<u64> {
    fn set(&mut self, index: Self::Index, item: Self::Item) {
        let index = index - self.0.position();
        assert!(index < 8*std::mem::size_of::<Self::Index>() as Self::Index);
        if item {
            self.0.set(self.0.position(), self.0.get(self.0.position()) | 0x01 << index);
        } else {
            self.0.set(self.0.position(), self.0.get(self.0.position()) & !(0x01 << index));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::block::{AlignedBlock, BlockGet, BlockSet};

    use super::AlignedBitfield;

    #[test]
    fn test_bitfield_usize() {

        assert_eq!(AlignedBitfield::<usize>::alignment(),8*std::mem::size_of::<usize>());
        
        let b = AlignedBitfield::<usize>((128,0));
        assert_eq!(b.position(), 128);
        assert_eq!(b.get(130), false);
    }

    #[test]
    fn test_bitfield_u64() {
        assert_eq!(AlignedBitfield::<u64>::alignment() as usize,8*std::mem::size_of::<u64>());
        
        let b = AlignedBitfield::<u64>((128,0));
        assert_eq!(b.position(), 128);
        assert_eq!(b.get(130), false);
    }

    #[test]
    fn test_bitfield_usize_set() {
        assert_eq!(AlignedBitfield::<usize>::alignment(),8*std::mem::size_of::<usize>());
        assert!(std::mem::size_of::<usize>() == 4 || std::mem::size_of::<usize>() == 8, "expecting either 32 bit or 64 bit usize");
        
        let mut b = AlignedBitfield::<usize>((128,0));
        b.set(130,true);
        b.set(128,true);
        b.set(140,true);
        b.set(128, false);

        if std::mem::size_of::<usize>() == 8 {
            b.set(191,true);
        }

        assert_eq!(b.get(128), false);
        assert_eq!(b.get(129), false);
        assert_eq!(b.get(130), true);
        assert_eq!(b.get(131), false);
        assert_eq!(b.get(135), false);
        assert_eq!(b.get(140), true);
        assert_eq!(b.get(145), false);

        if std::mem::size_of::<usize>() == 8 {
            assert_eq!(b.get(191), true);
        }
    }

    #[test]
    fn test_bitfield_u64_set() {
        assert_eq!(AlignedBitfield::<usize>::alignment(),64);
        
        let mut b = AlignedBitfield::<u64>((128,0));
        b.set(130,true);
        b.set(128,true);
        b.set(140,true);
        b.set(191,true);
        b.set(128, false);

        assert_eq!(b.get(128), false);
        assert_eq!(b.get(129), false);
        assert_eq!(b.get(130), true);
        assert_eq!(b.get(131), false);
        assert_eq!(b.get(135), false);
        assert_eq!(b.get(140), true);
        assert_eq!(b.get(145), false);
        assert_eq!(b.get(191), true);
    }
}