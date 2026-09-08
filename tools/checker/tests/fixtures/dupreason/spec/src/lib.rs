use patterns::because;

pub const CAP: u32 = 40;
because!(CAP, "the widest stack the inventory grid can render");

pub mod restated {
    use patterns::because;
    use super::CAP;

    because!(CAP, "actually forty because the 2019 engine used a byte");
}
