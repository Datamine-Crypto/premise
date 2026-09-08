use crate::contexts::holdqueue::vocabulary::{ByClaim, ByMember, ByQueue, Ready};
use patterns::{either, is_same, reduce_by, Advance, Facet, Keyed, Ranked};

#[derive(Clone)]
pub struct Hold {
    pub title: u32,
    pub member: u32,
    pub ready: Ready,
    pub left: u32,
}

#[derive(Clone)]
pub struct State {
    pub queue: Vec<Hold>,
}

impl Keyed for Hold {
    type Key = (u32, u32);

    fn key(&self) -> (u32, u32) {
        (self.title, self.member)
    }
}

impl Ranked for Hold {
    type Rank = u32;

    fn rank(&self) -> u32 {
        self.left
    }
}

impl Advance for Hold {
    type By = u32;

    fn advanced(self, by: u32) -> Hold {
        Hold {
            title: self.title,
            member: self.member,
            ready: self.ready,
            left: either(
                is_same(self.ready, Ready::Notified),
                reduce_by(self.left, by),
                self.left,
            ),
        }
    }
}

impl Facet<ByMember> for Hold {
    type Value = u32;

    fn facet(&self) -> u32 {
        self.member
    }
}

impl Facet<ByQueue> for Hold {
    type Value = (u32, Ready);

    fn facet(&self) -> (u32, Ready) {
        (self.title, self.ready)
    }
}

impl Facet<ByClaim> for Hold {
    type Value = (u32, u32, Ready);

    fn facet(&self) -> (u32, u32, Ready) {
        (self.title, self.member, self.ready)
    }
}
