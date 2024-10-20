use std::ops::{Index, IndexMut};

use super::AlignedBlock;

/// A vector as an AlignedBlock.
pub struct AlignedVec<const N: usize, T>((usize,Vec<T>));

impl<const N: usize, T> AlignedBlock for AlignedVec<N,T> {
    type Index = usize;
    type Item = T;

    fn alignment() -> Self::Index {
        N
    }

    fn position(&self) -> Self::Index {
        self.0.position()
    }
}

impl<const N: usize, T> AlignedVec<N, T> {
    /// Construct a new AlignedVec, starting at the given position.
    /// The position must be aligned with (divisble by) N.
    /// The length of the vector must be exactly N.
    pub fn new_from(position: usize, vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), Self::alignment());
        assert!(position % Self::alignment() == 0);
        AlignedVec((position, vec))
    }

    /// Turn this AlignedVec back into the original Vec.
    pub fn into_vec(self) -> Vec<T> {
        self.0.1
    }
}

impl<const N: usize, T> Index<usize> for AlignedVec<N,T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.1[index-self.position()]
    }
}

impl<const N: usize, T> IndexMut<usize> for AlignedVec<N,T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let position = self.position();
        &mut self.0.1[index-position]
    }
}

#[cfg(test)]
mod test {
    use super::AlignedVec;

    #[test]
    pub fn test_index() {
        let v = vec![1,2,3,4,5,6,7,8];
        let av: AlignedVec<8,i64> = AlignedVec::new_from(32,v);

        assert_eq!(av[32], 1);
        assert_eq!(av[36], 5);
        assert_eq!(av[39], 8);
    }

    #[test]
    pub fn test_write() {
        let v = vec![1,2,3,4,5,6,7,8];
        let mut av: AlignedVec<8,i64> = AlignedVec::new_from(32,v);

        av[36] = 500;

        assert_eq!(av[32], 1);
        assert_eq!(av[36], 500);
        assert_eq!(av[39], 8);
    }

    #[test]
    fn test_into_vec() {
        let v1 = vec![5,3,7,9,1,1,90,3];
        let av: AlignedVec<8,i64> = AlignedVec::new_from(32, v1.clone());
        let v2 = av.into_vec();

        assert_eq!(v1,v2);
    }
}