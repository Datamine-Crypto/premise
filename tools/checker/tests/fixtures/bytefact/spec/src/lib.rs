use patterns::{because, Named};

pub enum Grade {
    Lo,
}
because!(Grade, "the one grade the probe knows");

impl Named for Grade {
    fn text(&self) -> &'static [u8] {
        match self {
            Grade::Lo => b"\x0f",
        }
    }
}
