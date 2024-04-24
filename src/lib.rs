//! # optional_transpose
//! A small crate that adds `.transpose()` to `Option<Option<T>>`
//! It allows you to reversibly swap the inner and outer options in the pair, so that you can use `?` on the inner option
//!
//! Example:
//! ```rust
//! use optional_transpose::OptionTranspose;
//!
//! fn example() -> Option<i32> {
//!     let x: Option<Option<i32>> = Some(None);
//!     let y: Option<i32> = x.transpose()?; // Returns None, as the inner option is None
//!     Some(1)
//! }
//!
//! assert_eq!(example(), None);
//! ```
//!

/// Transpose an `Option` of an `Option` into an `Option` of an `Option`.
pub trait OptionTranspose<T> {
    /// Reverses the inner and outer `Option`s.
    /// ``````
    fn transpose(self) -> Option<Option<T>>;
}

impl<T> OptionTranspose<T> for Option<Option<T>> {
    fn transpose(self) -> Option<Option<T>> {
        match self {
            Some(Some(x)) => Some(Some(x)),
            Some(None) => None,
            _ => Some(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let x: Option<Option<i32>> = Some(Some(1));
        assert_eq!(x.transpose(), Some(Some(1)));

        let x: Option<Option<i32>> = Some(None);
        assert_eq!(x.transpose(), None);

        let x: Option<Option<i32>> = None;
        assert_eq!(x.transpose(), Some(None));
    }
}
