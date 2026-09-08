use patterns_macros::because;

pub fn raise_by(a: u32, b: u32) -> u32 {
    a.saturating_add(b)
}
because!(raise_by, "adds two amounts and stops at the top of the type rather than wrapping");
