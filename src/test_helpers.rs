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
