use patterns::is_below;
use spec::{ADULT_FINE_CAP_PENCE, BORROW_BAR_PENCE};

pub fn under_bar(owed: u32) -> bool {
    let bar = *&BORROW_BAR_PENCE;
    is_below(owed, bar)
}

pub fn under_cap(owed: u32) -> bool {
    is_below(owed, ADULT_FINE_CAP_PENCE)
}
