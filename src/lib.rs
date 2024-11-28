//! This crate's `lib.rs` is:
//! 
//! ```
//! pub fn add(left: u64, right: u64) -> u64 {
//!     left + right
//! }
//! 
//! #[cfg(debug_assertions)]
//! pub fn debug_add(left: u64, right: u64) {
//!     dbg!(add(left, right));
//! }
//! 
//! #[cfg(test)]
//! pub fn test_add(left: u64, right: u64, expected: u64) {
//!     assert_eq!(add(left, right), expected);
//! }
//! ```
//! 
//! Check what items are available by user.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(debug_assertions)]
pub fn debug_add(left: u64, right: u64) {
    dbg!(add(left, right));
}

#[cfg(test)]
pub fn test_add(left: u64, right: u64, expected: u64) {
    assert_eq!(add(left, right), expected);
}
