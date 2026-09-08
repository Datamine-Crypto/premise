use crate::arith::raise_by;
use patterns_macros::because;

pub fn plus(a: u32, b: u32) -> u32 {
    raise_by(a, b)
}
because!(plus, "adds two amounts through the library's own addition, to see whether a copy is noticed");

pub fn plus_ret(a: u32, b: u32) -> u32 {
    return raise_by(a, b);
}
because!(plus_ret, "the same addition returned explicitly, a copy that one keyword disguises");

pub fn plus_paren(a: u32, b: u32) -> u32 {
    (raise_by(a, b))
}
because!(plus_paren, "the same addition in parentheses, a copy that two brackets disguise");
