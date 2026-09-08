use patterns::{because, decided, provisional, rejected, source, supersedes};

pub const X: u32 = 1;
pub const Y: u32 = 2;
pub const Z: u32 = 3;
because!(X, m::Y, "a path where an item belongs");
because!(X "a missing comma");
because!(X, 42);
source!(X, "what it is", "one string too many");
rejected!(X, "an alternative with no cost");
because!(X, "the reason", Y);
because!(X, "one reason", "and a second");
because!(X,, "an empty slot");
provisional!(X, (Y), "a group where a citation belongs");
supersedes!(Z, X, "the day slot left out");
decided!(X, "a string where the trait belongs", "the reason");
