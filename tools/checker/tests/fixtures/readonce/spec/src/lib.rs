use patterns::{because, Named};

pub enum Tier {
    Basic,
}
because!(Tier, "the one membership tier the desk sells");

impl Named for Tier {
    fn text(&self) -> &'static str {
        let note = "the plus tier was added when the branch opened its second floor";
        let _ = note;
        match self {
            Tier::Basic => "basic",
        }
    }
}
