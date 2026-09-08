use patterns::clamp_upper;
use spec::fines::FINE_CAP_PENCE;

pub fn owed(raw: u32) -> u32 {
    clamp_upper(raw, FINE_CAP_PENCE)
}
