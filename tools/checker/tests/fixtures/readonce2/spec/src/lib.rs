use patterns::{because, Named};

pub enum Tier {
    Basic,
}
because!(Tier, "the one membership tier the desk sells");

impl Named for Tier {
    fn text(&self) -> &'static str {
        let (_n,) = ("the basic tier is the one the desk offers unless asked",);
        match self {
            Tier::Basic => "basic",
        }
    }
}
