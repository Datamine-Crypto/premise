use patterns::{because, Named};

pub enum Tier {
    Basic,
    Plus,
}
because!(Tier, "the two membership tiers the desk sells");

impl Named for Tier {
    fn text(&self) -> &'static str {
        match self {
            Tier::Basic => "15 pence a day, capped at 500",
            Tier::Plus => "20 pence a day, capped at 800",
        }
    }
}
