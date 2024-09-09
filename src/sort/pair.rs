/// A pair of half-slices.
pub struct Pair<'a,'b, T> {
    a: &'a mut [T],
    b: &'b mut [T],
}

impl<T> Default for Pair<'static,'static,T> {
    fn default() -> Self {
        Pair {
            a: &mut [],
            b: &mut [],
        }
    }
}

impl<'a,'b,T> Pair<'a,'b,T> {
    /// Construct a new pair with the given left and right half-slices.
    pub fn new(a: &'a mut [T], b: &'b mut [T]) -> Self {
        Pair {
            a,
            b,
        }
    }

    /// Replace the left half-slice of the pair.
    pub fn first<'x>(self, a: &'x mut [T]) -> Pair<'x,'b,T> {
        Pair::<'x,'b,T>::new(a,self.b)
    }

    /// Replace the right helf-slice of the pair.
    pub fn second<'y>(self, b: &'y mut [T]) -> Pair<'a,'y,T> {
        Pair::<'a,'y,T>::new(self.a,b)
    }

    /// The length of the combined pair.
    pub fn len(&self) -> usize {
        self.a.len()+self.b.len()
    }

    /// Given an index into the combined pair, calculate
    /// (1) which half-slice the index falls within, and
    /// (2) the corresponding index within that half-slice.
    pub fn index_of(&self, i: usize) -> (usize,usize) {
        if i < self.a.len() {
            (0,i)
        } else if i < self.a.len() + self.b.len() {
            (1,i-self.a.len())
        } else {
            panic!("index out of bounds")
        }
    }

    /// Get the left or right half-slices, based on the given index.
    /// The index must be 0 (left half-slice) or 1 (right half-slice).
    pub fn get_slice(&self, i: usize) -> &[T] {
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
    pub fn get_slice_mut(&mut self, i: usize) -> &mut [T] {
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
        let (k,i) = self.index_of(i);
        &self.get_slice(k)[i]
    }

    /// Get a mutable reference to the element at the given index into the combined pair.
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        let (k,i) = self.index_of(i);
        &mut self.get_slice_mut(k)[i]
    }
}