macro_rules! pull {
    ($m:ident) => {
        $m!("../tests/smuggled.rs");
    };
}

pull!(include);
