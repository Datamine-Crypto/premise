use std::cell::RefCell;

thread_local! {
    static ROOTS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}
patterns_macros::because!(
    ROOTS,
    "the crate names a governed path may start from to reach a pattern, set once per report from the zones file, held per thread because a report is scanned on one thread and two reports running beside each other read different zones files"
);

pub fn set(names: Vec<String>) {
    ROOTS.with(|held| {
        *held.borrow_mut() = names;
    });
}

pub fn is(name: &str) -> bool {
    if name == crate::config::HOME_LIBRARY {
        return true;
    }
    ROOTS.with(|held| held.borrow().iter().any(|n| n == name))
}
