use patterns::{because, Named};

pub enum Tier {
    Plus,
}
because!(Tier, "the one membership tier the desk sells");

impl Named for Tier {
    fn text(&self) -> &'static str {
        match self {
            Tier::Plus => "tier u32 costs f64 pence",
        }
    }
}
