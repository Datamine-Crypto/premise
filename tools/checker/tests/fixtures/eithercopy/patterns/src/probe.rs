use crate::arith::raise_by;
use crate::select::either;
use patterns_macros::because;

pub fn plus(a: u32, b: u32) -> u32 {
    raise_by(a, b)
}
because!(plus, "adds two amounts through the library's own addition, to see whether a copy is noticed");

pub fn plus_via(a: u32, b: u32) -> u32 {
    either(true, raise_by(a, b), raise_by(a, b))
}
because!(plus_via, "the same addition chosen between two copies of itself, a copy a no-op choice disguises");
