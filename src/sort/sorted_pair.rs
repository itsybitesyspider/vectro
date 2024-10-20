use std::{fmt::Debug, ops::{Index, IndexMut}};

use smallvec::SmallVec;

use super::Pair;

/// A sorted pair of half-slices.
/// As part of the sort, we allocate a temporary index vector of size N. N should ideally be the sum of the expected sizes of the two half-slices.
pub struct SortedPair<'x,T,const N: usize> {
    pair: &'x mut Pair<'x,'x,T>,
    sort: SmallVec<[usize;N]>
}

impl<'x,T,const N: usize> SortedPair<'x,T,N> {
    /// Construct a new SortedPair based on Pair.
    pub fn new(pair: &'x mut Pair<'x,'x,T>) -> Self {
        let sort: SmallVec<[usize;N]> = (0..pair.len()).collect();

        Self {
            pair,
            sort
        }
    }

    /// Iterate over this SortedPair in sort order.
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.sort.iter().map(|i| self.pair.get(*i))
    }

    /// Cause the ordering in this SortedPair to be applied to the underlying pair.
    /// A sorted pair has a sort order that may be different from the underlying storage.
    /// This method is the way to apply the sort order so that it persists after this SortedPair is dropped.
    pub fn apply_order(&mut self)
    where T:Clone+Debug {
        self.debug();
        for i in 0..self.sort.len() {
            let mut j = self.sort[i];
            while j < i {
                j = self.sort[j];
            }

            self.pair.swap(i,j);
            self.debug();
        }
    }

    pub(crate) fn debug(&self) 
    where T:Clone+Debug 
    {
        println!("sort: {:?}", self.sort.iter().cloned().collect::<Vec::<_>>());
        println!("pair: {:?}", self.pair.iter().cloned().collect::<Vec::<_>>());
        println!("self: {:?}", self.iter().cloned().collect::<Vec::<_>>());
    }

    /// Sort all elements.
    /// This sort order is temporary during this SortedPair's lifetime. To apply the sort order permanently, call `apply_order`.
    pub fn sort(&mut self) 
    where T: Copy + Ord
    {
        self.sort_by_key(|x| *x);
    }

    /// Sort all elements by key.
    /// This sort order is temporary during this SortedPair's lifetime. To apply the sort order permanently, call `apply_order`.
    pub fn sort_by_key<K:Ord>(&mut self, f: impl Fn(&T) -> K) {
        self.sort.sort_by_key(|i| f(self.pair.get(*i)));
    }
}

impl<'x,T,const N: usize> Index<usize> for SortedPair<'x,T,N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.pair.get(self.sort[index])
    }
}

impl<'x,T,const N: usize> IndexMut<usize> for SortedPair<'x,T,N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.pair.get_mut(self.sort[index])
    }
}

#[cfg(test)]
mod test {
    use crate::sort::Pair;

    use super::SortedPair;

    #[test]
    fn test_get_set() {
        let mut a = [1,2,3,4];
        let mut b = [5,6,7,8];
        let mut pair = Pair::new(&mut a, &mut b);
        let mut sorter = SortedPair::<_,8>::new(&mut pair);

        assert_eq!(sorter[0], 1);
        assert_eq!(sorter[2], 3);
        assert_eq!(sorter[5], 6);
        assert_eq!(sorter[7], 8);

        sorter[3] = 1000;

        assert_eq!(sorter[3], 1000);
    }

    #[test]
    fn test_sort() {
        let mut a = [4,3,2,8];
        let mut b = [1,7,6,5];
        let mut pair = Pair::new(&mut a, &mut b);
        let mut sorter = SortedPair::<_,8>::new(&mut pair);

        sorter.sort();

        assert_eq!(sorter.iter().copied().collect::<Vec<_>>(), [1,2,3,4,5,6,7,8]);
    }

    #[test]
    fn test_sort_then_mutate() {
        let mut a = [4,3,2,8];
        let mut b = [1,7,6,5];
        let mut pair = Pair::new(&mut a, &mut b);
        let mut sorter = SortedPair::<_,8>::new(&mut pair);

        sorter.sort();

        sorter[5] = 1000;

        assert_eq!(sorter.iter().copied().collect::<Vec<_>>(), [1,2,3,4,5,1000,7,8]);
    }

    #[test]
    fn test_apply_order() {
        let mut a = [4,3,2,8];
        let mut b = [1,7,6,5];

        {
            let mut pair = Pair::new(&mut a, &mut b);
            let mut sorter = SortedPair::<_,8>::new(&mut pair);
    
            sorter.sort();
            sorter.apply_order();
        }

        assert_eq!(a, [1,2,3,4]);
        assert_eq!(b, [5,6,7,8]);
    }
}