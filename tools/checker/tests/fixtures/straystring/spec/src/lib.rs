use patterns::because;

pub const TARIFF_TEXT: &str = "the line the desk prints";
because!(TARIFF_TEXT, "the line the desk prints on a receipt, kept as one string");
