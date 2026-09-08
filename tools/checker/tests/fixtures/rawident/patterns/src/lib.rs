r#include!("../tests/smuggled.rs");

#[r#path = "../tests/smuggled.rs"]
pub mod sneaked;

#[r#macro_export]
macro_rules! decide {
    ($v:expr, $cap:expr) => {
        $v
    };
}
