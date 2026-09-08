use patterns_macros::because;

pub fn is_after<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}
because!(is_after, "whether one moment falls later than another, named in the mould of is_above");

pub fn is_before<T: PartialOrd>(a: T, b: T) -> bool {
    a < b
}
because!(is_before, "whether one moment falls earlier than another, named in the mould of is_below");
