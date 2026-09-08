use patterns::because;

pub const REFUND_WINDOW_DAYS: u32 = 30;
because!(REFUND_WINDOW_DAYS, "the window the payment processor allows for a chargeback");

pub const BROKEN: u32 = "this is not a u32";
because!(BROKEN, "a value the 2024 review recorded as a string by mistake");
