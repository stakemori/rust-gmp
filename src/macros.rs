macro_rules! nonzero_assert {
    ($is_zero: expr) => {
        assert!(! $is_zero, "divide by zero");
    };
}
