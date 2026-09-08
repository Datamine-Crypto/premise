use crate::contexts::alpha::vocabulary::{Alpha, Peekable, Seen};

pub struct Peek {
    pub a: Seen,
    pub b: <Alpha as Peekable>::Inner,
}
