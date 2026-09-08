use crate::contexts::seasonticket::vocabulary::{ByBay, Cover, Tier};
use patterns::{either, is_same, raise_by, Advance, Facet, Keyed, Ranked};

#[derive(Clone, Copy)]
pub struct Ticket {
    pub id: u32,
    pub bay: u32,
    pub tier: Tier,
    pub bought: u32,
    pub expires: u32,
    pub used: u32,
    pub since: u32,
    pub cover: Cover,
    pub owed: u32,
}

#[derive(Clone)]
pub struct State {
    pub tickets: Vec<Ticket>,
    pub today: u32,
}

#[derive(Clone, Copy)]
pub enum Step {
    Hold { ticket: u32, since: u32 },
    Release { ticket: u32, granted: u32 },
    Move { ticket: u32, bay: u32, fee: u32 },
}

impl Keyed for Ticket {
    type Key = u32;

    fn key(&self) -> u32 {
        self.id
    }
}

impl Ranked for Ticket {
    type Rank = u32;

    fn rank(&self) -> u32 {
        self.expires
    }
}

impl Facet<ByBay> for Ticket {
    type Value = u32;

    fn facet(&self) -> u32 {
        self.bay
    }
}

impl Advance for Ticket {
    type By = Step;

    fn advanced(self, by: Step) -> Ticket {
        match by {
            Step::Hold { ticket, since } => either(
                is_same(self.id, ticket),
                Ticket {
                    id: self.id,
                    bay: self.bay,
                    tier: self.tier,
                    bought: self.bought,
                    expires: self.expires,
                    used: self.used,
                    since,
                    cover: Cover::Suspended,
                    owed: self.owed,
                },
                self,
            ),
            Step::Release { ticket, granted } => either(
                is_same(self.id, ticket),
                Ticket {
                    id: self.id,
                    bay: self.bay,
                    tier: self.tier,
                    bought: self.bought,
                    expires: raise_by(self.expires, granted),
                    used: raise_by(self.used, granted),
                    since: self.since,
                    cover: Cover::Active,
                    owed: self.owed,
                },
                self,
            ),
            Step::Move { ticket, bay, fee } => either(
                is_same(self.id, ticket),
                Ticket {
                    id: self.id,
                    bay,
                    tier: self.tier,
                    bought: self.bought,
                    expires: self.expires,
                    used: self.used,
                    since: self.since,
                    cover: self.cover,
                    owed: raise_by(self.owed, fee),
                },
                self,
            ),
        }
    }
}
