#![macro_use]

#[cfg(test)]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr) => {{
        let message = format!(
            "assertion failed: `(left == right)`\n  left: `${:0<len_left$X?}`\n right: `${:0<len_right$X?}`",
            $left,
            $right,
            len_left = std::mem::size_of_val(&($left)) * 2,
            len_right = std::mem::size_of_val(&($right)) * 2
        );
        assert!(&($left) == &($right), message);
    }};
}
