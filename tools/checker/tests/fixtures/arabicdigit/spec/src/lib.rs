use patterns::{because, Named};

pub enum Tier {
    Basic,
}
because!(Tier, "the one membership tier the desk sells");

impl Named for Tier {
    fn text(&self) -> &'static str {
        match self {
            Tier::Basic => "١٥ pence a day",
        }
    }
}
