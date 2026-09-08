pub const LOW: u32 = 5;

pub const HIGH: u32 = 40;

pub const SPAN: u32 = HIGH - LOW;

pub const TIER: u32 = [LOW, HIGH, SPAN][((HIGH > LOW) as usize) + 1];

pub const PICK: u32 = [LOW, HIGH][(SPAN > LOW) as usize];
