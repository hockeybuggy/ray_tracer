// One tricky thing about the macros is that the `assert_approx_eq` crate doesn't support messages
// when something fails. This means that it can be hard to tell which of the many sub assertions is
// the one that failed.

#[macro_export]
macro_rules! assert_tuple_approx_eq {
    ($a:expr, $b:expr) => {{
        use assert_approx_eq::assert_approx_eq;
        assert_approx_eq!($a.x, $b.x, 1e-5f64);
        assert_approx_eq!($a.y, $b.y, 1e-5f64);
        assert_approx_eq!($a.z, $b.z, 1e-5f64);
        assert_approx_eq!($a.w, $b.w, 1e-5f64);
    }};
}

#[macro_export]
macro_rules! assert_color_approx_eq {
    ($a:expr, $b:expr) => {{
        use assert_approx_eq::assert_approx_eq;
        assert_approx_eq!($a.r, $b.r, 1e-5f64);
        assert_approx_eq!($a.g, $b.g, 1e-5f64);
        assert_approx_eq!($a.b, $b.b, 1e-5f64);
    }};
}

#[macro_export]
macro_rules! assert_matrix_approx_eq {
    ( $a:expr, $b:expr) => {{
        use assert_approx_eq::assert_approx_eq;
        for x in 0..4 {
            for y in 0..4 {
                assert_approx_eq!($a[(x, y)], $b[(x, y)], 1e-5f64);
            }
        }
    }};
}
