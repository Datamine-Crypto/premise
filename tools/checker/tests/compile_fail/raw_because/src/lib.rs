use patterns::because;

pub const FOO: u32 = 7;
because!(FOO, "a measured threshold from the 2019 capacity report");
because!(r#FOO, "a second reason wearing a raw spelling");
