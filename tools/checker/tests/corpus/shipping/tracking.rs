use patterns::because;

pub const STAGE_LABEL: &str = "stage";
because!(STAGE_LABEL, "the key the warehouse terminal prints on its status row");

pub const TRACKING_LABEL: &str = "tracking";
because!(TRACKING_LABEL, "the key the carrier API expects on a handoff payload");

pub const SEPARATOR: &str = " ";
because!(SEPARATOR, "a single space is what the terminal's fixed width row uses");

pub const TRACKING_BASE: u64 = 11400714819323198486;
because!(TRACKING_BASE, "the carrier allocated us this block start for 2026 volume");

pub const TRACKING_SPAN: u64 = 1000;
because!(TRACKING_SPAN, "the carrier block the 2026 contract reserves for us");

pub const TRACKING_END: u64 = TRACKING_BASE + TRACKING_SPAN;
