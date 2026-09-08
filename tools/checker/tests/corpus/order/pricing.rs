use patterns::because;

pub const DOMESTIC_SHIPPING: u32 = 500;
because!(DOMESTIC_SHIPPING, "flat rate the carrier quoted for zone one in 2026");

pub const INTERNATIONAL_SHIPPING: u32 = 2400;
because!(INTERNATIONAL_SHIPPING, "carrier zone four rate plus the customs handling fee");

pub const DOMESTIC_TAX: u32 = 200;
because!(DOMESTIC_TAX, "the standard rate the 2026 finance act sets for home sales");

pub const INTERNATIONAL_TAX: u32 = 0;
because!(INTERNATIONAL_TAX, "exports are zero rated so the buyer settles duty on arrival");

pub const REGION_LABEL: &str = "region";
because!(REGION_LABEL, "the column heading the finance export writes for each row");

pub const POSTAGE_LABEL: &str = "postage";
because!(POSTAGE_LABEL, "the carrier reconciliation file keys the charge on this word");

pub const STAGE_LABEL: &str = "stage";
because!(STAGE_LABEL, "the key the warehouse terminal prints on its status row");

pub const SEPARATOR: &str = " ";
because!(SEPARATOR, "a single space is what the fixed width terminal row uses");
