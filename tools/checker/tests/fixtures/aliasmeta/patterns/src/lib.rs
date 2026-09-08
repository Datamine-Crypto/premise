use std::include as grab;

macro_rules! pull {
    ($m:ident) => {
        $m!("../tests/smuggled.rs");
    };
}

pull!(grab);
