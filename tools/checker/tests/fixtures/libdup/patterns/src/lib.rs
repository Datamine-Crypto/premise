use patterns_macros::because;

pub fn head<T: Copy>(x: T, y: T) -> T {
    x
}
because!(head, "the first of two values, kept so a caller can name which one it wanted");
