macro_rules! nonzero_assert {
    ($is_zero: expr) => {
        assert!(! $is_zero, "divide by zero");
    };
}

macro_rules! int_to_ord {
    ($cmp: expr) => {{
        let cmp = $cmp;
        if cmp == 0 {
            Equal
        } else if cmp < 0 {
            Less
        } else {
            Greater
        }
    }}
}
