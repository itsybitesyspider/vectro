/// A pair of half-slices.
pub struct Pair<'a, 'b, T> {
    a: &'a mut [T],
    b: &'b mut [T],
}

impl<T> Default for Pair<'static, 'static, T> {
    fn default() -> Self {
        Pair {
            a: &mut [],
            b: &mut [],
        }
    }
}

impl<'a, 'b, T> Pair<'a, 'b, T> {
    /// Construct a new pair with the given left and right half-slices.
    pub fn new(a: &'a mut [T], b: &'b mut [T]) -> Self {
        Pair { a, b }
    }

    /// Replace the left half-slice of the pair.
    pub fn first<'x>(self, a: &'x mut [T]) -> Pair<'x, 'b, T> {
        Pair::<'x, 'b, T>::new(a, self.b)
    }

    /// Replace the right helf-slice of the pair.
    pub fn second<'y>(self, b: &'y mut [T]) -> Pair<'a, 'y, T> {
        Pair::<'a, 'y, T>::new(self.a, b)
    }

    /// The length of the combined pair.
    pub fn len(&self) -> usize {
        self.a.len() + self.b.len()
    }

    /// Given an index into the combined pair, calculate
    /// (1) which half-slice the index falls within, and
    /// (2) the corresponding index within that half-slice.
    fn index_of(&self, i: usize) -> (usize, usize) {
        if i < self.a.len() {
            (0, i)
        } else if i < self.a.len() + self.b.len() {
            (1, i - self.a.len())
        } else {
            panic!("index out of bounds")
        }
    }

    /// Get the left or right half-slices, based on the given index.
    /// The index must be 0 (left half-slice) or 1 (right half-slice).
    fn get_slice(&self, i: usize) -> &[T] {
        if i == 0 {
            self.a
        } else if i == 1 {
            self.b
        } else {
            panic!("index out of bounds")
        }
    }

    /// Get a mutable reference to the left or right half-slices, based on the given index.
    /// Same behavior as `get_slice`.
    fn get_slice_mut(&mut self, i: usize) -> &mut [T] {
        if i == 0 {
            self.a
        } else if i == 1 {
            self.b
        } else {
            panic!("index out of bounds")
        }
    }

    /// Get a reference to the element at the given index into the combined pair.
    pub fn get(&self, i: usize) -> &T {
        let (k, i) = self.index_of(i);
        &self.get_slice(k)[i]
    }

    /// Get a mutable reference to the element at the given index into the combined pair.
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        let (k, i) = self.index_of(i);
        &mut self.get_slice_mut(k)[i]
    }

    /// Iterator over all elements of the pair.
    pub fn iter<'x: 'a + 'b>(&'x self) -> impl Iterator<Item = &'x T> {
        self.a.iter().chain(self.b.iter())
    }

    /// Exchange the elements at the two given positions.
    pub fn swap(&mut self, i: usize, j: usize) {
        let mut i = self.index_of(i);
        let mut j = self.index_of(j);

        if i.0 == j.0 {
            self.get_slice_mut(i.0).swap(i.1, j.1);
        } else {
            if i.0 > j.0 {
                std::mem::swap(&mut i, &mut j);
            }

            let a = &mut self.a[i.1];
            let b = &mut self.b[j.1];
            std::mem::swap(a, b);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Pair;

    #[test]
    fn test_len() {
        let mut a = [1, 2, 3, 4, 5];
        let mut b = [1, 2, 3, 4, 5, 6, 7, 8];

        let pair = Pair::default().first(&mut a).second(&mut b);

        assert_eq!(pair.len(), 13);
    }

    #[test]
    fn test_get() {
        let mut a = [1, 2, 3, 4, 5];
        let mut b = [10, 20, 30, 40, 50, 60, 70, 80];

        let pair = Pair::default().first(&mut a).second(&mut b);

        assert_eq!(pair.get(0), &1);
        assert_eq!(pair.get(3), &4);
        assert_eq!(pair.get(4), &5);
        assert_eq!(pair.get(5), &10);
        assert_eq!(pair.get(8), &40);
        assert_eq!(pair.get(12), &80);
    }

    #[test]
    fn test_get_mut() {
        let mut a = [1, 2, 3, 4, 5];
        let mut b = [10, 20, 30, 40, 50, 60, 70, 80];

        let mut pair = Pair::default().first(&mut a).second(&mut b);

        *pair.get_mut(2) = -1;
        *pair.get_mut(8) = -1;
        assert_eq!(
            pair.iter().copied().collect::<Vec<_>>(),
            vec![1, 2, -1, 4, 5, 10, 20, 30, -1, 50, 60, 70, 80]
        );
    }

    #[test]
    fn test_swap() {
        let mut a = [1, 2, 3, 4, 5];
        let mut b = [10, 20, 30, 40, 50];

        let mut pair = Pair::default().first(&mut a).second(&mut b);

        pair.swap(2, 7);

        assert_eq!(
            pair.iter().copied().collect::<Vec<_>>(),
            vec![1, 2, 30, 4, 5, 10, 20, 3, 40, 50]
        );
    }

    #[test]
    fn test_reverse_swap() {
        let mut a = [1, 2, 3, 4, 5];
        let mut b = [10, 20, 30, 40, 50];

        let mut pair = Pair::default().first(&mut a).second(&mut b);

        pair.swap(7, 2);

        assert_eq!(
            pair.iter().copied().collect::<Vec<_>>(),
            vec![1, 2, 30, 4, 5, 10, 20, 3, 40, 50]
        );
    }
}
