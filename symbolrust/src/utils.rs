#[allow(unused_macros)]
macro_rules! assert_eps {
    ($left:expr, $right:expr, $eps:expr) => {{
        let left = $left;
        let right = $right;
        let eps = $eps;
        let delta = (left - right).abs();
        if delta > eps {
            // The reborrows below are intentional. Without them, the stack slot for the
            // borrow is initialized even before the values are compared, leading to a
            // noticeable slow down.
            panic!(
                r#"assertion failed: `(left ~= right (esp))`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`"#,
                left, right, eps
            );
        }
    }};
    ($left:expr, $right:expr) => {
        assert_eps!($left, $right, 1e-4);
    };
}
