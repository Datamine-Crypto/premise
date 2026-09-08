#[r#macro_export]
macro_rules! decide {
    ($v:expr, $cap:expr) => {
        if $v > $cap {
            $cap - (1 + 1 + 1)
        } else {
            $v * $cap + (1 + 1 + 1 + 1 + 1 + 1 + 1)
        }
    };
}
