use std::ops::{Index, IndexMut};

use super::{AlignedBlock, BlockFetch, BlockStore, IndexedBlock};

/// A vector as an AlignedBlock.s
pub struct AlignedVec<T,const N: usize> {
    position: usize,
    vec: Vec<T>
}

impl<T, const N: usize> IndexedBlock for AlignedVec<T,N> {
    type Index = usize;
    type Item = T;
}

impl<T, const N: usize> AlignedBlock for AlignedVec<T,N> {
    fn alignment() -> Self::Index {
        N
    }

    fn position(&self) -> Self::Index {
        self.position
    }
}

impl<T, const N: usize> AlignedVec<T, N> {
    /// Construct a new AlignedVec, starting at the given position.
    /// The position must be aligned with (divisble by) N.
    /// The length of the vector must be exactly N.
    pub fn new_from(position: usize, vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), Self::alignment());
        assert!(position % Self::alignment() == 0);
        AlignedVec {
            position, 
            vec
        }
    }

    /// Turn this AlignedVec back into the original Vec.
    pub fn into_vec(self) -> Vec<T> {
        self.vec
    }

    fn index_of(&self, index: usize) -> usize {
        assert!(index >= self.position());
        let index = index - self.position();
        assert!(index < Self::alignment());
        index
    }
}

impl<T,const N: usize> BlockFetch for AlignedVec<T,N> 
where T: Copy
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        self[index]
    }
}

impl<T,const N: usize> BlockStore for AlignedVec<T,N> 
where T: Copy
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        self[index] = item;
    }
}

impl<T,const N: usize> Index<usize> for AlignedVec<T,N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[self.index_of(index)]
    }
}

impl<T, const N: usize> IndexMut<usize> for AlignedVec<T,N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = self.index_of(index);
        &mut self.vec[index]
    }
}

#[cfg(test)]
mod test {
    use super::AlignedVec;

    #[test]
    pub fn test_index() {
        let v = vec![1,2,3,4,5,6,7,8];
        let av: AlignedVec<i64,8> = AlignedVec::new_from(32,v);

        assert_eq!(av[32], 1);
        assert_eq!(av[36], 5);
        assert_eq!(av[39], 8);
    }

    #[test]
    pub fn test_write() {
        let v = vec![1,2,3,4,5,6,7,8];
        let mut av: AlignedVec<i64, 8> = AlignedVec::new_from(32,v);

        av[36] = 500;

        assert_eq!(av[32], 1);
        assert_eq!(av[36], 500);
        assert_eq!(av[39], 8);
    }

    #[test]
    fn test_into_vec() {
        let v1 = vec![5,3,7,9,1,1,90,3];
        let av: AlignedVec<i64, 8> = AlignedVec::new_from(32, v1.clone());
        let v2 = av.into_vec();

        assert_eq!(v1,v2);
    }
}