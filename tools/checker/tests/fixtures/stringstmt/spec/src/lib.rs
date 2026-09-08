use patterns::{because, Named};

pub enum Tier {
    Basic,
    Plus,
}
because!(Tier, "the two membership tiers the desk sells");

impl Named for Tier {
    fn text(&self) -> &'static str {
        "the plus tier was added when the branch opened its second floor";
        match self {
            Tier::Basic => ("the basic tier is the one the desk offers unless asked", "basic").1,
            Tier::Plus => "plus",
        }
    }
}
