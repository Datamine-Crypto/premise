macro_rules! pull {
    () => {
        include!("../tests/smuggled.rs");
    };
}

pull!();
