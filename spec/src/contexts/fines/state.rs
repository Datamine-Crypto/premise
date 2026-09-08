use crate::contexts::fines::vocabulary::{ByBorrower, ByCharge, BySettle, Grade};
use patterns::{either, is_same, raise_by, reduce_by, Facet, Keyed, Ranked, Shift};

#[derive(Clone, Copy)]
pub struct Member {
    pub id: u32,
    pub grade: Grade,
    pub owed: u32,
}

#[derive(Clone, Copy)]
pub struct Loan {
    pub id: u32,
    pub member: u32,
    pub due: u32,
}

#[derive(Clone)]
pub struct State {
    pub members: Vec<Member>,
    pub loans: Vec<Loan>,
    pub today: u32,
}

#[derive(Clone, Copy)]
pub struct Charge {
    pub member: u32,
    pub fine: u32,
}

#[derive(Clone, Copy)]
pub struct Settle {
    pub member: u32,
    pub amount: u32,
}

impl Keyed for Member {
    type Key = u32;

    fn key(&self) -> u32 {
        self.id
    }
}

impl Keyed for Loan {
    type Key = u32;

    fn key(&self) -> u32 {
        self.id
    }
}

impl Facet<ByBorrower> for Loan {
    type Value = u32;

    fn facet(&self) -> u32 {
        self.member
    }
}

impl Ranked for Loan {
    type Rank = u32;

    fn rank(&self) -> u32 {
        self.due
    }
}

impl Shift<ByCharge> for Member {
    type By = Charge;

    fn shifted(self, by: Charge) -> Member {
        either(
            is_same(self.id, by.member),
            Member {
                id: self.id,
                grade: self.grade,
                owed: raise_by(self.owed, by.fine),
            },
            self,
        )
    }
}

impl Shift<BySettle> for Member {
    type By = Settle;

    fn shifted(self, by: Settle) -> Member {
        either(
            is_same(self.id, by.member),
            Member {
                id: self.id,
                grade: self.grade,
                owed: reduce_by(self.owed, by.amount),
            },
            self,
        )
    }
}
