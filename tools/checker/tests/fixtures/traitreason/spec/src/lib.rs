use patterns::because;

pub trait Cap {
    const CAP: u32;
}
because!(Cap, "the ceiling a plan puts on what one member may hold at once");
pub struct Alpha;
pub struct Beta;
impl Cap for Alpha {
    const CAP: u32 = 500;
}
impl Cap for Beta {
    const CAP: u32 = 7;
}
