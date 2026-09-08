use crate::contexts::gate::vocabulary::{Command, OPEN_LIMIT};
use patterns::is_at_least;

pub struct State {
    pub opened: u32,
    pub locked: bool,
}

pub trait Guards {
    fn worn(&self, command: &Command) -> bool;
    fn shut(&self, command: &Command) -> bool;
}

impl Guards for State {
    fn worn(&self, command: &Command) -> bool {
        match command {
            Command::Open => is_at_least(self.opened, OPEN_LIMIT),
            Command::Lock => false,
        }
    }
    fn shut(&self, command: &Command) -> bool {
        match command {
            Command::Open => false,
            Command::Lock => self.locked,
        }
    }
}
