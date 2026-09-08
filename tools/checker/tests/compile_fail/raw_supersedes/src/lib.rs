#![allow(non_upper_case_globals)]
use patterns::{because, rejected, source, supersedes};

pub struct Review;
source!(Review, "the review that reset the value",);

pub const OLD: u32 = 1;
because!(OLD, "the first value",);

pub const WHEN: u32 = 2;
because!(WHEN, Review, "the day the value changed");

pub const r#type: u32 = 3;
because!(r#type, Review, "the value that replaced the first");
supersedes!(r#type, OLD, WHEN, "the review that reset it");
rejected!(r#type, "keeping the first value", "it no longer covered the cost");
