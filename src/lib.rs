#![deny(unused)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![allow(clippy::style)]
#![allow(clippy::complexity)]

//! Very large collections.

/// Bitsets
pub mod bitset;
/// Utilities for working with blocks of data.
pub mod block;
/// Index types
pub mod numerical_index;
/// Run-length encoding.
pub mod rle;
/// Utilities for sorting.
pub mod sort;
