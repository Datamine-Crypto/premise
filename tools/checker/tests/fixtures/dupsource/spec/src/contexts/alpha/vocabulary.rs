use patterns::{because, source};

pub struct TariffReview;
source!(TariffReview, "the review that set the season ticket tariff");

pub const RATE: u32 = 15;
because!(RATE, TariffReview, "the daily rate the review chose");
