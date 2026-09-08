use patterns::because;

pub const MONTHLY_CENTS: u32 = 1200;
because!(MONTHLY_CENTS, "the price the 2026 pricing study put at peak conversion");

pub const MONTHS_PER_YEAR: u32 = 12;
because!(MONTHS_PER_YEAR, "a calendar year, fixed by the Gregorian calendar and not by us");

pub const ANNUAL_LIST_CENTS: u32 = MONTHLY_CENTS * MONTHS_PER_YEAR;

pub const ANNUAL_CENTS: u32 = 12000;
because!(ANNUAL_CENTS, "ANNUAL_LIST_CENTS less two months, the discount sales needs to close annual deals");

pub const FREE_SEATS: u32 = 3;
because!(FREE_SEATS, "the largest team that never converted in the 2026 trial cohort");

pub const TEAM_SEATS: u32 = 25;
because!(TEAM_SEATS, "the point where support load per account starts to need an owner");

pub const ENTERPRISE_SEATS: u32 = 500;
because!(ENTERPRISE_SEATS, "the largest seat count the provisioning service can allocate in one call");

pub const TIER_LABEL: &str = "tier";
because!(TIER_LABEL, "the column heading the finance export writes for each account row");

pub const SEATS_LABEL: &str = "seats";
because!(SEATS_LABEL, "the key the admin console reads when it redraws the seat meter");

pub const OWED_LABEL: &str = "owed";
because!(OWED_LABEL, "the key the dunning job reads to decide whether to send a notice");

pub const SEPARATOR: &str = " ";
because!(SEPARATOR, "a single space is what the fixed width console row uses");
