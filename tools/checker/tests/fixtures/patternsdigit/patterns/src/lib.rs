use patterns_macros::because;

pub fn tariff_line() -> &'static str {
    "15 pence a day, capped at 500"
}
because!(tariff_line, "the line the desk prints, kept in the library where no string rule read it");
