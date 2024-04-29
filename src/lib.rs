pub use smooth_operator_impl::checked;

#[cfg(test)]
mod test {
    use super::checked;

    #[test]
    pub fn test_basic_use() {
        let result = checked!(1_u64 + 2 + 3).unwrap();
        assert_eq!(result, 6);

        let x = u64::MAX - 1;
        let result = checked!(x + 1 + 1).unwrap_err();
        println!("{result}");
        assert_eq!(result, "Failure in: x + 1  》+《  1");

        let result = checked!(-1_i64).unwrap();
        assert_eq!(result, -1);

        let result = checked!(-i64::MIN).unwrap_err();
        assert_eq!(result, "Failure in:  》-《  i64 :: MIN");

        let result = checked!(u64::MAX << 123).unwrap_err();
        assert_eq!(result, "Failure in: u64 :: MAX  》<<《  123");
    }
}
