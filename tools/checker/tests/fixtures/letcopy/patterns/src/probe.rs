use crate::arith::raise_by;
use patterns_macros::because;

pub fn plus(a: u32, b: u32) -> u32 {
    raise_by(a, b)
}
because!(plus, "adds two amounts through the library's own addition, to see whether a copy is noticed");

pub fn plus_let(a: u32, b: u32) -> u32 {
    let out = raise_by(a, b);
    out
}
because!(plus_let, "the same addition with its result bound first, a copy that a let disguises");

pub fn plus_block(a: u32, b: u32) -> u32 {
    {
        raise_by(a, b)
    }
}
because!(plus_block, "the same addition wrapped in a block, a copy that two braces disguise");
