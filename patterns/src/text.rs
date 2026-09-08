use crate::day::{civil_day, day_start};
use crate::money::none;
use crate::render::{digits, fixed, grouped};
use patterns_macros::{because, source};

pub struct ReaderConventions;
source!(ReaderConventions, "the ways the dashboard's readers are used to seeing a figure, a date and an age written, carried over from the retired client so a number reads the same on both");

const POINT: u8 = 46;
because!(POINT, ReaderConventions, "the decimal point, written as its code point because a character literal is a value from nowhere in this zone");

const MINUS: u8 = 45;
because!(MINUS, ReaderConventions, "the sign a negative figure carries, written as its code point since a character literal is a value from nowhere in this zone");

const DASH: u8 = 45;
because!(DASH, ReaderConventions, "the separator between the parts of a machine-readable date, the same code point as the sign, which it is");

const SLASH: u8 = 47;
because!(SLASH, ReaderConventions, "the separator between the parts of a date as the tables show it");

const ZERO: u8 = 48;
because!(ZERO, ReaderConventions, "the digit that pads a month or a day to two places");

const DOLLAR_MARK: &str = "$ ";
because!(DOLLAR_MARK, ReaderConventions, "the sign and the space that lead every dollar figure");

const PERCENT_MARK: &str = "%";
because!(PERCENT_MARK, ReaderConventions, "the sign that follows every percentage");

const USD_MIN_PLACES: usize = 2;
because!(USD_MIN_PLACES, ReaderConventions, "the places a dollar figure always shows, cents, unless the caller asks for fewer");

const DATE_PLACES: usize = 2;
because!(DATE_PLACES, ReaderConventions, "the width a month or day number is padded to in a date");

const DAY_SECONDS: u64 = 86_400;
because!(DAY_SECONDS, ReaderConventions, "the seconds in a day, which an age counts days by and a date is floored to");

const HOUR_SECONDS: u64 = 3_600;
because!(HOUR_SECONDS, ReaderConventions, "the seconds in an hour, which an age counts hours by");

const MINUTE_SECONDS: u64 = 60;
because!(MINUTE_SECONDS, ReaderConventions, "the seconds in the smallest unit an age is shown in");

const YEAR_DAYS: u64 = 365;
because!(YEAR_DAYS, ReaderConventions, "the days in a year as an age counts them, without leap days, the way the retired client did");

const BYTE_BITS: u32 = 8;
because!(BYTE_BITS, ReaderConventions, "the bits one colour channel takes in a packed colour");

const BYTE_MASK: u32 = 0xff;
because!(BYTE_MASK, ReaderConventions, "the mask that keeps one colour channel of a packed colour");

const CHANNELS_BELOW_RED: u32 = 2;
because!(CHANNELS_BELOW_RED, ReaderConventions, "the channels packed below the red one, green and blue, which red is shifted past");

const RED_SHIFT: u32 = BYTE_BITS * CHANNELS_BELOW_RED;

fn padded(value: u32) -> String {
    let plain = digits(value);
    let mut out = String::new();
    for _ in plain.len()..DATE_PLACES {
        out.push(ZERO as char);
    }
    out.push_str(&plain);
    out
}

pub fn day_text(day: u64) -> String {
    let (year, month, d) = civil_day(day, DAY_SECONDS);
    let mut out = digits(year);
    out.push(SLASH as char);
    out.push_str(&padded(month));
    out.push(SLASH as char);
    out.push_str(&padded(d));
    out
}
because!(day_text, "a day as the tables show it, year then month then day with slashes between");

pub fn iso_day_text(day: u64) -> String {
    let (year, month, d) = civil_day(day, DAY_SECONDS);
    let parts = [digits(year), padded(month), padded(d)];
    parts.join(&(DASH as char).to_string())
}
because!(iso_day_text, "a day in the machine-readable form a date input and a spreadsheet read, year then month then day with dashes between");

pub fn day_from_iso(text: &str) -> Option<u64> {
    let mut parts = text.split(DASH as char);
    let year: i64 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    Some(day_start(year, month, day, DAY_SECONDS))
}
because!(day_from_iso, "the moment a date input names, the start of that day, read back through the same civil calendar the day was written with");

pub fn grouped_places(value: f64, min_places: usize, max_places: usize) -> String {
    let text = fixed(value.abs(), max_places);
    let (whole, fraction) = match text.split_once(POINT as char) {
        Some((w, f)) => (String::from(w), String::from(f)),
        None => (text.clone(), String::new()),
    };
    let mut kept = fraction.trim_end_matches(ZERO as char).to_string();
    while kept.len() < min_places {
        kept.push(ZERO as char);
    }
    let whole_value = whole.parse::<u64>().unwrap_or(0);
    let mut out = String::new();
    if value < none() && (whole_value > 0 || !kept.chars().all(|c| c == ZERO as char)) {
        out.push(MINUS as char);
    }
    out.push_str(&grouped(whole_value));
    if !kept.is_empty() {
        out.push(POINT as char);
        out.push_str(&kept);
    }
    out
}
because!(grouped_places, "a figure with separators in its whole part and between a floor and a ceiling of places after the point, trailing zeros dropped down to the floor, the way the retired client's locale formatting wrote every number");

pub fn usd_text(value: f64, max_places: usize) -> String {
    let mut out = String::from(DOLLAR_MARK);
    out.push_str(&grouped_places(value, USD_MIN_PLACES.min(max_places), max_places));
    out
}
because!(usd_text, "a dollar figure with its sign, always showing cents unless fewer places were asked for");

pub fn percent_text(value: f64, places: usize) -> String {
    let mut out = fixed(value, places);
    out.push_str(PERCENT_MARK);
    out
}
because!(percent_text, "a percentage with a fixed count of places and its sign");

pub fn grouped_percent_text(value: f64, places: usize) -> String {
    let mut out = grouped_places(value, 0, places);
    out.push_str(PERCENT_MARK);
    out
}
because!(grouped_percent_text, "a percentage with separators and up to a count of places, the form a history column of kind percent takes");

pub fn count_text(value: u64) -> String {
    grouped(value)
}
because!(count_text, "a count with separators, the form every row count and rank is shown in");

pub fn duration_parts(now: u64, then: u64) -> (u64, u64, u64, u64) {
    let total = now.saturating_sub(then);
    let days = total / DAY_SECONDS;
    let hours = (total - days * DAY_SECONDS) / HOUR_SECONDS;
    let smallest = (total - days * DAY_SECONDS - hours * HOUR_SECONDS) / MINUTE_SECONDS;
    let years = days / YEAR_DAYS;
    (years, days, hours, smallest)
}
because!(duration_parts, "an age split into whole years, days, hours and the smallest unit shown, the parts an age text is chosen from");

pub fn duration_text(now: u64, then: u64, year_word: &str, years_word: &str, days_word: &str, hours_word: &str, smallest_word: &str) -> String {
    if then == 0 {
        return String::new();
    }
    let (years, days, hours, smallest) = duration_parts(now, then);
    let mut out = String::new();
    match (years > 0, days > 0) {
        (true, _) => {
            out.push_str(&digits(years));
            out.push_str(match years > 1 {
                true => years_word,
                false => year_word,
            });
            out.push_str(&digits(days - years * YEAR_DAYS));
            out.push_str(days_word);
        }
        (false, true) => {
            out.push_str(&digits(days));
            out.push_str(days_word);
            out.push_str(&digits(hours));
            out.push_str(hours_word);
        }
        (false, false) => {
            out.push_str(&digits(hours));
            out.push_str(hours_word);
            out.push_str(&digits(smallest));
            out.push_str(smallest_word);
        }
    }
    out.trim_end().to_string()
}
because!(duration_text, "an age as the retired client wrote it: years and days past a year, days and hours past a day, hours and the smallest unit under a day, with the unit words handed in by the spec");

const QUOTE: u8 = 34;
because!(QUOTE, ReaderConventions, "the mark that wraps a spreadsheet cell holding a separator, a quote or a line break");

const COMMA: u8 = 44;
because!(COMMA, ReaderConventions, "the separator between spreadsheet cells");

const LINE_BREAK: u8 = 10;
because!(LINE_BREAK, ReaderConventions, "the separator between spreadsheet rows");

pub fn csv_cell(text: &str) -> String {
    let needs_quotes = text.chars().any(|c| c == QUOTE as char || c == COMMA as char || c == LINE_BREAK as char);
    match needs_quotes {
        true => {
            let mut out = String::new();
            out.push(QUOTE as char);
            for c in text.chars() {
                if c == QUOTE as char {
                    out.push(QUOTE as char);
                }
                out.push(c);
            }
            out.push(QUOTE as char);
            out
        }
        false => String::from(text),
    }
}
because!(csv_cell, "one spreadsheet cell, quoted and with its quotes doubled only when it holds a character the format reserves");

pub fn csv_text(header: &[String], rows: &[Vec<String>]) -> String {
    let mut lines: Vec<String> = Vec::with_capacity(rows.len() + 1);
    let line = |cells: &[String]| cells.iter().map(|c| csv_cell(c)).collect::<Vec<String>>().join(&(COMMA as char).to_string());
    lines.push(line(header));
    for row in rows {
        lines.push(line(row));
    }
    lines.join(&(LINE_BREAK as char).to_string())
}
because!(csv_text, "a header and rows as one spreadsheet text, the download a report offers");

pub fn hex_color(ink: u32) -> String {
    format!("#{:06x}", ink)
}
because!(hex_color, "a colour written the way a style sheet reads it, from the number the spec states");

pub fn rgba_text(ink: u32, alpha: f64) -> String {
    let red = (ink >> RED_SHIFT) & BYTE_MASK;
    let green = (ink >> BYTE_BITS) & BYTE_MASK;
    let blue = ink & BYTE_MASK;
    format!("rgba({}, {}, {}, {})", red, green, blue, alpha)
}
because!(rgba_text, "a colour with its opacity written the way a style sheet reads it, for the fill under a chart line");

pub fn points_table(columns: &[(String, Vec<(u64, f64)>)], day_text: fn(u64) -> String) -> (Vec<String>, Vec<Vec<String>>) {
    let mut days: Vec<u64> = columns.iter().flat_map(|(_, points)| points.iter().map(|(day, _)| *day)).collect();
    days.sort_unstable();
    days.dedup();
    let header: Vec<String> = columns.iter().map(|(name, _)| name.clone()).collect();
    let rows: Vec<Vec<String>> = days
        .iter()
        .map(|day| {
            let mut row = vec![day_text(*day)];
            for (_, points) in columns {
                row.push(points.iter().find(|(d, _)| d == day).map(|(_, v)| digits(v)).unwrap_or_default());
            }
            row
        })
        .collect();
    (header, rows)
}
because!(points_table, "several named lines as one table with a row per day and an empty cell where a line has no point, the daily spreadsheet the report offers");
