use super::{aligned_block::{AlignedBlock, BlockGet}};

/// An aligned block of booleans.
pub struct AlignedBitfield<T>((T,T));

impl AlignedBlock for AlignedBitfield<usize> {
    type Index = usize;
    type Item = bool;

    fn alignment() -> Self::Index {
        std::mem::size_of::<Self::Index>()
    }

    fn position(&self) -> Self::Index {
        self.0.position()
    }
}

impl AlignedBlock for AlignedBitfield<u64> {
    type Index = u64;
    type Item = bool;

    fn alignment() -> Self::Index {
        std::mem::size_of::<Self::Index>() as u64
    }

    fn position(&self) -> Self::Index {
        self.0.position()
    }
}

impl BlockGet for AlignedBitfield<usize> {
    fn get(&self, index: Self::Index) -> bool {
        let index = index - self.0.position();
        assert!(index < std::mem::size_of::<Self::Index>() as Self::Index);
        (self.0.get(self.0.position()) >> index) & 0x01 != 0
    }
}

impl BlockGet for AlignedBitfield<u64> {
    fn get(&self, index: Self::Index) -> bool {
        let index = index - self.0.position();
        assert!(index < std::mem::size_of::<Self::Index>() as Self::Index);
        (self.0.get(self.0.position()) >> index) & 0x01 != 0
    }
}

#[cfg(test)]
mod test {
    use crate::block::{AlignedBlock, BlockGet};

    use super::AlignedBitfield;

    #[test]
    fn test_bitfield_usize() {
        assert_eq!(AlignedBitfield::<usize>::alignment(),std::mem::size_of::<usize>());
        
        let b = AlignedBitfield::<usize>((128,0));
        assert_eq!(b.position(), 128);
        assert_eq!(b.get(130), false);
    }

    #[test]
    fn test_bitfield_u64() {
        assert_eq!(AlignedBitfield::<u64>::alignment() as usize,std::mem::size_of::<u64>());
        
        let b = AlignedBitfield::<u64>((128,0));
        assert_eq!(b.position(), 128);
        assert_eq!(b.get(130), false);
    }
}