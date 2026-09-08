use patterns_macros::because;

pub fn either<T>(when: bool, yes: T, no: T) -> T {
    match when {
        true => yes,
        false => no,
    }
}
because!(either, "one choice between two built values, written as a call so a decide arm reads as a table");
