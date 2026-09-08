use patterns::{because, source};

pub struct TariffReview;
source!(TariffReview, "the review that set the tariff and nobody cites");

pub const RATE: u32 = 15;
because!(RATE, "what an overdue day costs, set against the cost of chasing it");
