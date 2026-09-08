use patterns::because;

pub const CAP: u32 = 40;
because!(CAP, "the widest stack the inventory grid can render");

pub mod legacy {
    use patterns::because;

    pub const CAP: u32 = 250;
    because!(CAP, "the old ceiling kept for the 2019 replay archive");
}
