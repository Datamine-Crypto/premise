use patterns::{either, is_same, Advance};

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Ajar,
    Shut,
}
#[derive(Clone, Copy)]
pub enum Step {
    Flip,
    Slam,
}
impl Advance for Phase {
    type By = Step;
    fn advanced(self, by: Step) -> Phase {
        match by {
            Step::Flip => either(is_same(self, Phase::Ajar), Phase::Shut, Phase::Ajar),
            Step::Slam => Phase::Shut,
        }
    }
}
