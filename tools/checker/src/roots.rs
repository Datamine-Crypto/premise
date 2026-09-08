use std::sync::RwLock;

static ROOTS: RwLock<Vec<String>> = RwLock::new(Vec::new());
patterns_macros::because!(
    ROOTS,
    "the crate names a governed path may start from to reach a pattern, set once per report from the zones file, because the scanners see one file at a time and a file does not say which library its project takes"
);

pub fn set(names: Vec<String>) {
    if let Ok(mut held) = ROOTS.write() {
        *held = names;
    }
}

pub fn is(name: &str) -> bool {
    if name == crate::config::HOME_LIBRARY {
        return true;
    }
    match ROOTS.read() {
        Ok(held) => held.iter().any(|n| n == name),
        Err(_) => false,
    }
}
