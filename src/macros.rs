macro_rules! nonzero_assert {
    ($is_zero: expr) => {
        assert!(! $is_zero, "divide by zero");
    };
}

macro_rules! int_to_ord {
    ($cmp: expr) => {
        {
            let cmp = $cmp;
            if cmp == 0 {
                Equal
            } else if cmp < 0 {
                Less
            } else {
                Greater
            }
        }
    }
}

macro_rules! impl_part_eq {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialEq<$c_type> for $t {
            fn eq(&self, other: &$c_type) -> bool {
                unsafe { $c_func(self.inner(), *other) == 0 }
            }
        }
    };
}

macro_rules! impl_part_cmp {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialOrd<$c_type> for $t {
            fn partial_cmp(&self, other: &$c_type) -> Option<Ordering> {
                Some(int_to_ord!(unsafe { $c_func(self.inner(), *other) }))
            }
        }
    }
}
