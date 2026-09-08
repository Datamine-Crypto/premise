use patterns::Context;

pub struct Gate;

pub struct Flag {
    pub ready: bool,
    pub hot: bool,
}

pub enum Signal {
    Open,
    Shut,
}

impl Context for Gate {
    type State = Flag;
    type Command = Signal;
    type Event = Signal;
    type Fault = Signal;

    fn initial() -> Flag {
        Flag {
            ready: false,
            hot: false,
        }
    }

    fn decide(_s: &Flag, c: &Signal) -> Result<Signal, Signal> {
        match c {
            Signal::Open => Ok(Signal::Open),
            Signal::Shut => Ok(Signal::Shut),
        }
    }

    fn apply(state: Flag, event: &Signal) -> Flag {
        match state.ready {
            true => match state.hot {
                true => Flag {
                    ready: true,
                    hot: true,
                },
                false => Flag {
                    ready: true,
                    hot: false,
                },
            },
            false => Flag {
                ready: false,
                hot: false,
            },
        }
    }
}
