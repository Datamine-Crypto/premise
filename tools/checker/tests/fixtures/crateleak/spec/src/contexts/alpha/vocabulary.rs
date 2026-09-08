pub struct Alpha;
pub trait Peekable {
    type Inner;
}
impl Peekable for Alpha {
    type Inner = crate::contexts::alpha::state::State;
}
pub(crate) type Seen = crate::contexts::alpha::state::State;
