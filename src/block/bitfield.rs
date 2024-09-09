use super::{aligned_block::{AlignedBlock, BlockGet}, singleton::Singleton};

/// An aligned block of booleans.
pub struct AlignedBitfield<T>(Singleton<T,T>);

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
        assert!(index < std::mem::size_of::<Self::Item>() as Self::Index);
        (self.0.get(self.0.position()) >> index) & 0x01 != 0
    }
}

impl BlockGet for AlignedBitfield<u64> {
    fn get(&self, index: Self::Index) -> bool {
        let index = index - self.0.position();
        assert!(index < std::mem::size_of::<Self::Item>() as Self::Index);
        (self.0.get(self.0.position()) >> index) & 0x01 != 0
    }
}