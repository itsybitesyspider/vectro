/// A NumericalIndex, needed to index many kinds of sparse collections.
pub trait NumericalIndex: Copy + Eq + Ord {
    /// True iff the index is zero.
    fn is_zero(self) -> bool;
    /// Modulo division of an index.
    fn modulo(self, divisor: Self) -> Self;
    /// Division of an index.
    fn divide(self, divisor: Self) -> Self;
    /// Get the beginning of the index's block, given an alignment.
    fn block(self, alignment: Self) -> Self;
}

impl NumericalIndex for usize {
    fn is_zero(self) -> bool {
        self == 0
    }

    fn modulo(self, divisor: Self) -> Self {
        self % divisor
    }

    fn divide(self, divisor: Self) -> Self {
        self / divisor
    }

    fn block(self, alignment: Self) -> Self {
        (self / alignment) * alignment
    }
}

impl NumericalIndex for u64 {
    fn is_zero(self) -> bool {
        self == 0
    }

    fn modulo(self, divisor: Self) -> Self {
        self % divisor
    }

    fn divide(self, divisor: Self) -> Self {
        self / divisor
    }

    fn block(self, alignment: Self) -> Self {
        (self / alignment) * alignment
    }
}

impl NumericalIndex for u128 {
    fn is_zero(self) -> bool {
        self == 0
    }

    fn modulo(self, divisor: Self) -> Self {
        self % divisor
    }

    fn divide(self, divisor: Self) -> Self {
        self / divisor
    }

    fn block(self, alignment: Self) -> Self {
        (self / alignment) * alignment
    }
}

impl NumericalIndex for u32 {
    fn is_zero(self) -> bool {
        self == 0
    }

    fn modulo(self, divisor: Self) -> Self {
        self % divisor
    }

    fn divide(self, divisor: Self) -> Self {
        self / divisor
    }

    fn block(self, alignment: Self) -> Self {
        (self / alignment) * alignment
    }
}

impl NumericalIndex for u16 {
    fn is_zero(self) -> bool {
        self == 0
    }

    fn modulo(self, divisor: Self) -> Self {
        self % divisor
    }

    fn divide(self, divisor: Self) -> Self {
        self / divisor
    }

    fn block(self, alignment: Self) -> Self {
        (self / alignment) * alignment
    }
}

impl NumericalIndex for u8 {
    fn is_zero(self) -> bool {
        self == 0
    }

    fn modulo(self, divisor: Self) -> Self {
        self % divisor
    }

    fn divide(self, divisor: Self) -> Self {
        self / divisor
    }

    fn block(self, alignment: Self) -> Self {
        (self / alignment) * alignment
    }
}
