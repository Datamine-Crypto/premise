use patterns_macros::{because, source};

pub fn day_of(time: u64, day_seconds: u64) -> u64 {
    match day_seconds == 0 {
        true => time,
        false => (time / day_seconds) * day_seconds,
    }
}
because!(day_of, "a moment floored to the start of its day, the bucket every series is kept in, so two events on one day land on one row");

pub fn window_end(from: u64, span: u64, cap: u64) -> u64 {
    from.saturating_add(span).min(cap)
}
because!(window_end, "the far edge of a fetch window, a span past its start but never past the cap, which is how a request for a run of days stays inside what a source will answer and inside today");

pub fn days_between(from: u64, to: u64, day_seconds: u64) -> u64 {
    match day_seconds == 0 {
        true => 0,
        false => to.saturating_sub(from) / day_seconds,
    }
}
because!(days_between, "the whole days from one moment to a later one, the count a fetch window is sized by and an age is shown by");

pub struct CivilCalendar;
source!(CivilCalendar, "the proleptic Gregorian calendar as the epoch counts it, and the arithmetic that turns a day count into a year and month and back, taken from the well known days-from-civil derivation");

const EPOCH_SHIFT_DAYS: i64 = 719_468;
because!(EPOCH_SHIFT_DAYS, CivilCalendar, "the days from the start of the four hundred year cycle the derivation counts from to the epoch");

const ERA_DAYS: i64 = 146_097;
because!(ERA_DAYS, CivilCalendar, "the days in one four hundred year cycle of the calendar, after which leap years repeat exactly");

const ERA_YEARS: i64 = 400;
because!(ERA_YEARS, CivilCalendar, "the years in one cycle of the calendar");

const LEAP_EVERY: i64 = 4;
because!(LEAP_EVERY, CivilCalendar, "the years between leap years inside a century");

const CENTURY: i64 = 100;
because!(CENTURY, CivilCalendar, "the years in a century, at which a leap year is skipped");

const MARCH_SHIFT_MONTHS: i64 = 3;
because!(MARCH_SHIFT_MONTHS, CivilCalendar, "the months the derivation shifts the year by so that the leap day falls at the year's end and the month lengths follow one rule");

const MONTH_DAY_SCALE: i64 = 153;
because!(MONTH_DAY_SCALE, CivilCalendar, "the numerator of the five month rule the derivation uses, since five months from March hold this many days");

const MONTH_DAY_OFFSET: i64 = 2;
because!(MONTH_DAY_OFFSET, CivilCalendar, "the offset in the same five month rule");

const MONTH_DAY_DIVISOR: i64 = 5;
because!(MONTH_DAY_DIVISOR, CivilCalendar, "what the scaled day count is divided by in the five month rule, the months that rule spans");

const MONTHS_IN_YEAR: i64 = 12;
because!(MONTHS_IN_YEAR, CivilCalendar, "the months in a year, where the shifted month wraps");

const MONTHS_BEFORE_WRAP: i64 = 10;
because!(MONTHS_BEFORE_WRAP, CivilCalendar, "the shifted month at which the calendar year turns over, since the derivation counts from March");

const YEAR_DAYS_WHOLE: i64 = 365;
because!(YEAR_DAYS_WHOLE, CivilCalendar, "how many days a year holds when it has no leap day, the base the leap corrections add to");

pub fn civil_day(day_seconds_since_epoch: u64, day_seconds: u64) -> (i64, u32, u32) {
    let days = match day_seconds == 0 {
        true => 0,
        false => (day_seconds_since_epoch / day_seconds) as i64,
    };
    let z = days + EPOCH_SHIFT_DAYS;
    let era = z.div_euclid(ERA_DAYS);
    let doe = z.rem_euclid(ERA_DAYS);
    let yoe = (doe - doe / (ERA_DAYS / ERA_YEARS * LEAP_EVERY - 1) + doe / (YEAR_DAYS_WHOLE * CENTURY + CENTURY / LEAP_EVERY - 1) - doe / (ERA_DAYS - 1)) / YEAR_DAYS_WHOLE;
    let y = yoe + era * ERA_YEARS;
    let doy = doe - (YEAR_DAYS_WHOLE * yoe + yoe / LEAP_EVERY - yoe / CENTURY);
    let mp = (MONTH_DAY_DIVISOR * doy + MONTH_DAY_OFFSET) / MONTH_DAY_SCALE;
    let d = doy - (MONTH_DAY_SCALE * mp + MONTH_DAY_OFFSET) / MONTH_DAY_DIVISOR + 1;
    let m = match mp < MONTHS_BEFORE_WRAP {
        true => mp + MARCH_SHIFT_MONTHS,
        false => mp - (MONTHS_IN_YEAR - MARCH_SHIFT_MONTHS),
    };
    let year = match m <= MONTH_DAY_OFFSET {
        true => y + 1,
        false => y,
    };
    (year, m as u32, d as u32)
}
because!(civil_day, "the calendar year, month and day a moment falls in, by the closed form derivation rather than a table, so a date is written without a calendar library");

pub fn civil_of(day_seconds_since_epoch: u64, day_seconds: u64) -> (i64, u32) {
    let (year, month, _) = civil_day(day_seconds_since_epoch, day_seconds);
    (year, month)
}
because!(civil_of, "the calendar year and month alone, the pair a month end is found from");

pub fn day_start(year: i64, month: u32, day: u32, day_seconds: u64) -> u64 {
    let (y, m) = match month <= MONTH_DAY_OFFSET as u32 {
        true => (year - 1, month as i64),
        false => (year, month as i64),
    };
    let era = y.div_euclid(ERA_YEARS);
    let yoe = y.rem_euclid(ERA_YEARS);
    let shifted = match m > MONTH_DAY_OFFSET {
        true => m - MARCH_SHIFT_MONTHS,
        false => m + (MONTHS_IN_YEAR - MARCH_SHIFT_MONTHS),
    };
    let doy = (MONTH_DAY_SCALE * shifted + MONTH_DAY_OFFSET) / MONTH_DAY_DIVISOR + i64::from(day) - 1;
    let doe = yoe * YEAR_DAYS_WHOLE + yoe / LEAP_EVERY - yoe / CENTURY + doy;
    let days = era * ERA_DAYS + doe - EPOCH_SHIFT_DAYS;
    (days.max(0) as u64).saturating_mul(day_seconds)
}
because!(day_start, "the first moment of a calendar day, the reverse of civil_day, which a date input names and a month end is one day back from");

pub fn month_start(year: i64, month: u32, day_seconds: u64) -> u64 {
    day_start(year, month, 1, day_seconds)
}
because!(month_start, "the first moment of a calendar month, the first day of it");

pub fn month_ends(report_day: u64, count: u64, day_seconds: u64) -> Vec<u64> {
    let (year, month) = civil_of(report_day, day_seconds);
    let mut out = vec![report_day];
    for back in 0..count as i64 {
        let total = year * MONTHS_IN_YEAR + (month as i64 - 1) - back;
        let y = total.div_euclid(MONTHS_IN_YEAR);
        let m = total.rem_euclid(MONTHS_IN_YEAR) + 1;
        let end = month_start(y, m as u32, day_seconds).saturating_sub(day_seconds);
        if end < report_day {
            out.push(end);
        }
    }
    out
}
because!(month_ends, "the report day followed by the last day of each of the months before it, newest first, so a report reads its figures at the same moments a reader would tick off on a calendar");
