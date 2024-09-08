use std::iter::Peekable;

/// Take an iterator and encode it according to its consecutive runs.
/// For example, `[1,1,1,2,2,7]` would become `[(1,3),(2,2),(7,1))]`.
pub struct EncodeConsecutiveRuns<Iter>
where Iter:Iterator 
{
    iter: Peekable<Iter>,
}

impl<Iter> EncodeConsecutiveRuns<Iter>
where Iter:Iterator,
      Iter::Item:Eq,
{
    /// Wrap an iterator to encode it.
    pub fn new(iter: Iter) -> Self {
        let iter = iter.peekable();

        Self { iter }
    }
}

impl<Iter> Iterator for EncodeConsecutiveRuns<Iter>
where Iter:Iterator,
      Iter::Item:Eq,
{
    type Item = (Iter::Item,u128);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.next()?;
        let mut count = 1;

        while self.iter.next_if_eq(&value).is_some() {
            count += 1;
        }

        Some((value,count))
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use super::EncodeConsecutiveRuns;

    #[test]
    fn test_empty() {
        let empty: [i32;0] = [];
        let iter = EncodeConsecutiveRuns::new(empty.iter().copied());
        let result : Vec<_> = iter.collect();

        assert_eq!(&result, &[]);
    }

    #[test]
    fn test_123() {
        let ott = [1,2,3];
        let iter = EncodeConsecutiveRuns::new(ott.iter().copied());
        let result : Vec<_> = iter.collect();

        assert_eq!(&result, &[(1,1),(2,1),(3,1)]);
    }

    #[test]
    fn test_medium() {
        let big = [1,8,8,8,8,8,8,8,8,2,2,4,4,4,4,6,6,6,6,6,6,7,7,7,7,7,7,7,3,3,3,5,5,5,5,5];
        let iter = EncodeConsecutiveRuns::new(big.iter().copied());
        let result : Vec<_> = iter.collect();

        assert_eq!(&result, &[(1,1),(8,8),(2,2),(4,4),(6,6),(7,7),(3,3),(5,5)]);
    }

    #[test]
    fn test_big() {
        let big = vec![7;7000000];
        let iter = EncodeConsecutiveRuns::new(big.iter().copied());
        let result : Vec<_> = iter.collect();

        assert_eq!(&result, &[(7,7000000)]);
    }

    proptest! {
        #[test]
        fn test_pack_unpack_u16(original: Vec<u16>) {
            // Because it's a u16, there will be very short runs (probably all 1-length)
            use super::super::DecodeConsecutiveRuns;
            let result : Vec<u16> = DecodeConsecutiveRuns::new(EncodeConsecutiveRuns::new(original.iter().copied())).collect();
            assert_eq!(original,result);
        }

        #[test]
        fn test_pack_unpack_u8(original: Vec<u8>) {
            // Because it's a u8, runs will be short but more likely to have consecutive repeats
            use super::super::DecodeConsecutiveRuns;
            let result : Vec<u8> = DecodeConsecutiveRuns::new(EncodeConsecutiveRuns::new(original.iter().copied())).collect();
            assert_eq!(original,result);
        }

        #[test]
        fn test_pack_unpack_bool(original: Vec<bool>) {
            // Because it's a bool, consecutive repeats are virtually guaranteed
            use super::super::DecodeConsecutiveRuns;
            let result : Vec<bool> = DecodeConsecutiveRuns::new(EncodeConsecutiveRuns::new(original.iter().copied())).collect();
            assert_eq!(original,result);
        }
    }
}