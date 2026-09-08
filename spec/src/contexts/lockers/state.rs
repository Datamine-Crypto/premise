use crate::contexts::lockers::vocabulary::{
    ByAbandoned, ByContents, ByHolder, ByRenter, BySize, Command, Contents, Moved, Size, LOCKER_BAYS,
};
use patterns::{either, holds, is_full, is_same, reduce_by, Advance, Facet, Keyed};

#[derive(Clone)]
pub struct Rental {
    pub locker: u32,
    pub renter: u32,
    pub size: Size,
    pub left: u32,
    pub grace: u32,
    pub moved: Moved,
}

#[derive(Clone)]
pub struct Salvage {
    pub locker: u32,
    pub renter: u32,
    pub contents: Contents,
}

#[derive(Clone)]
pub struct State {
    pub rentals: Vec<Rental>,
    pub log: Vec<Salvage>,
}

impl Keyed for Rental {
    type Key = u32;

    fn key(&self) -> u32 {
        self.locker
    }
}

impl Advance for Rental {
    type By = u32;

    fn advanced(self, by: u32) -> Rental {
        Rental {
            locker: self.locker,
            renter: self.renter,
            size: self.size,
            left: reduce_by(self.left, by),
            grace: either(
                is_same(self.left, 0),
                reduce_by(self.grace, by),
                self.grace,
            ),
            moved: self.moved,
        }
    }
}

impl Facet<ByRenter> for Rental {
    type Value = (u32, u32);

    fn facet(&self) -> (u32, u32) {
        (self.locker, self.renter)
    }
}

impl Facet<ByHolder> for Rental {
    type Value = (u32, u32, Moved);

    fn facet(&self) -> (u32, u32, Moved) {
        (self.locker, self.renter, self.moved)
    }
}

impl Facet<ByAbandoned> for Rental {
    type Value = (u32, u32);

    fn facet(&self) -> (u32, u32) {
        (self.locker, self.grace)
    }
}

impl Facet<BySize> for Rental {
    type Value = Size;

    fn facet(&self) -> Size {
        self.size
    }
}

impl Keyed for Salvage {
    type Key = u32;

    fn key(&self) -> u32 {
        self.locker
    }
}

impl Facet<ByContents> for Salvage {
    type Value = Contents;

    fn facet(&self) -> Contents {
        self.contents
    }
}

pub trait Guards {
    fn taken(&self, command: &Command) -> bool;
    fn full(&self, command: &Command) -> bool;
}

impl Guards for State {
    fn taken(&self, command: &Command) -> bool {
        match command {
            Command::Rent { locker, .. } => holds(&self.rentals, *locker),
            Command::Transfer { .. } => false,
            Command::Reclaim { .. } => false,
            Command::PassDay => false,
        }
    }

    fn full(&self, command: &Command) -> bool {
        match command {
            Command::Rent { .. } => is_full(&self.rentals, LOCKER_BAYS),
            Command::Transfer { .. } => false,
            Command::Reclaim { .. } => false,
            Command::PassDay => false,
        }
    }
}
