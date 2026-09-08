use patterns::{because, source};

pub struct TariffReview;
source!(TariffReview, "the review that set the bike dock tariff");

pub const PENCE: u32 = 22;
because!(PENCE, TariffReview, "the per minute rate the review chose");
