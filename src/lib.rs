#![cfg_attr(not(feature = "std"), no_std)]

#[doc(inline)]
pub use smooth_operator_impl::checked;

/// Checked arithmetics error.
pub struct Error {
    /// The original expression given to the [`checked`] macro.
    pub expr: &'static str,
    // Index of the operator that has failed within the `expr`.
    #[doc(hidden)]
    pub __op_ix: usize,
    // Length of the operator that has failed within the `expr`.
    #[doc(hidden)]
    pub __op_len: usize,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Error")
            .field("expr", &self.expr)
            .finish_non_exhaustive()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Error {
            expr,
            __op_ix: op_ix,
            __op_len: op_len,
        } = self;
        let (prefix, rest) = expr.split_at(op_ix.saturating_sub(*op_len));
        let (op, suffix) = rest.split_at(*op_len);
        write!(f, "Failure in: {prefix} 》{op}《 {suffix}")
    }
}

#[cfg(test)]
mod test {
    use super::checked;

    #[test]
    fn test_basic_use() {
        let result = checked!(1_u64 + 2 + 3).unwrap();
        assert_eq!(result, 6);

        let x = u64::MAX - 1;
        let result = checked!(x + 1 + 1).unwrap_err().to_string();
        assert_eq!(result, "Failure in: x + 1  》+《  1");

        let result = checked!(-1_i64).unwrap();
        assert_eq!(result, -1);

        let result = checked!(-i64::MIN).unwrap_err().to_string();
        assert_eq!(result, "Failure in:  》-《  i64 :: MIN");

        let result = checked!(u64::MAX << 123).unwrap_err().to_string();
        assert_eq!(result, "Failure in: u64 :: MAX  》<<《  123");
    }
}
