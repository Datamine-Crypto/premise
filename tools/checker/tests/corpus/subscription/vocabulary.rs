use patterns::Named;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tier {
    Free,
    Team,
    Enterprise,
}

impl Named for Tier {
    fn text(&self) -> &'static str {
        match self {
            Tier::Free => "free",
            Tier::Team => "team",
            Tier::Enterprise => "enterprise",
        }
    }
}

pub enum Command {
    Add(u32),
    Remove(u32),
    Upgrade(Tier),
}

pub enum Event {
    Added(u32),
    Removed(u32),
    Upgraded(Tier),
}

#[derive(Debug, PartialEq)]
pub enum Fault {
    OverCap,
    NoSeats,
}
