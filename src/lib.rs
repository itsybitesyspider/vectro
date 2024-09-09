#![deny(unused)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![allow(clippy::style)]
#![allow(clippy::complexity)]

//! Very large collections.

/// Utilities for working with blocks of data.
pub mod block;
/// Run-length encoding.
pub mod rle;
/// Utilities for sorting.
pub mod sort;
