pub struct Probe3;

pub trait Holder {
    type S;
    const START: Self::S;
}
