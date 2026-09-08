use patterns_macros::because;

pub fn size<T>(items: &[T]) -> usize {
    items.len()
}
because!(size, "length as a call, so a length comparison never appears in spec or app");
