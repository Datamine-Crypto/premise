use patterns::clamp_upper;
use spec::lockers::LOCKER_CAP_PENCE;

pub fn owed(raw: u32) -> u32 {
    clamp_upper(raw, LOCKER_CAP_PENCE)
}
