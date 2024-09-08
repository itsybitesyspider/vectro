use std::num::NonZeroU8;

/// A single encoding element, representing a value and the number of repetitions.
#[derive(Copy,Clone,Debug,Eq,Ord,Hash,PartialEq,PartialOrd)]
pub(super) enum RleInstruction {
    NextValue, Run(NonZeroU8)
}

static_assertions::assert_eq_size!(RleInstruction,u8);

/// We pack run-lengths into u8's.
/// For values from 1-127, we use the exact value.
/// For values from 128-255, we take it as an exponent.
/// This allows compact representation of extremely long runs.
/// This function unpacks an encoded run-length into it's actual value.
pub(super) fn unpack_length(length: NonZeroU8) -> u128 {
    if length.get() < 128 {
        length.get() as u128
    } else {
        1 << (length.get()-128)
    }
}

/// Inverse operation of `unpack_length`.
pub(super) fn pack_length(length: u128) -> (NonZeroU8,u128) {
    if length < 128 {
        (NonZeroU8::new(length as u8).expect("length must not be zero"),0)
    } else {
        let result = NonZeroU8::new(128 + length.ilog2() as u8).expect("this would be a bug in run length encoding");
        let remainder = length - unpack_length(result);
        (result,remainder)
    }
}

/// Pack length as an iterator.
pub(super) fn pack_length_iter(mut length: u128) -> impl Iterator<Item=NonZeroU8> {
    std::iter::from_fn(move || {
        match length {
            0 => None,
            _ => {
                let (packed, remainder) = pack_length(length);
                length = remainder;
                Some(packed)
            }
        }
    })
}

impl RleInstruction {
    /// Construct a new run of RleInstructions with the given length.
    pub(super) fn new_run(length: u128) -> impl Iterator<Item=RleInstruction> {
        std::iter::once(RleInstruction::NextValue).chain(pack_length_iter(length).map(|x| RleInstruction::Run(x)))
    }

    /// Unpack this instruction into a length.
    pub(super) fn unpack(&self) -> u128 {
        match self {
            RleInstruction::Run(length) => unpack_length(*length),
            RleInstruction::NextValue => panic!("tried to unpack the length of a value token"),
        }
    }

    pub(super) fn is_next_value(&self) -> bool {
        match self {
            RleInstruction::NextValue => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    
}