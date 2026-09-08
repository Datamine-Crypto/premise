use patterns_macros::because;

pub fn is_nought(n: u32) -> bool {
    n == 0
}
because!(is_nought, "a zero test as a call so the comparison is written once");

pub fn is_nil(n: u32) -> bool {
    n == 0
}
because!(is_nil, "the same zero test under a second name, which is the duplicate");
