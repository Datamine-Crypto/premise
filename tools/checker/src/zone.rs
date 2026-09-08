#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Tests,
    Patterns,
    Spec,
    App,
    Tools,
}

#[derive(Debug, Clone, Copy)]
pub struct Rules {
    pub lexical: bool,
    pub no_splice: bool,
    pub no_cfg: bool,
    pub citation: bool,
    pub no_macro: bool,
    pub const_literal: bool,
    pub no_fn: bool,
    pub comment: bool,
    pub literal: bool,
    pub logic: bool,
    pub impure: bool,
    pub because: bool,
    pub dup: bool,
}

impl Zone {
    pub fn of(rel: &str) -> Option<Zone> {
        let p = rel.replace(std::path::MAIN_SEPARATOR, "/");
        if p.starts_with("patterns/") {
            return Some(Zone::Patterns);
        }
        if p.starts_with("spec/") {
            return Some(Zone::Spec);
        }
        if p.starts_with("app/") {
            return Some(Zone::App);
        }
        if p.starts_with("tools/") {
            return Some(Zone::Tools);
        }
        None
    }

    pub fn rules(self) -> Rules {
        match self {
            Zone::Patterns => Rules {
                lexical: false,
                no_splice: true,
                no_cfg: true,
                citation: true,
                no_macro: true,
                const_literal: true,
                no_fn: false,
                comment: true,
                literal: true,
                logic: false,
                impure: true,
                because: true,
                dup: true,
            },
            Zone::Spec => Rules {
                lexical: false,
                no_splice: true,
                no_cfg: true,
                citation: false,
                no_macro: true,
                const_literal: true,
                no_fn: true,
                comment: true,
                literal: true,
                logic: true,
                impure: false,
                because: true,
                dup: false,
            },
            Zone::App => Rules {
                lexical: false,
                no_splice: true,
                no_cfg: true,
                citation: false,
                no_macro: true,
                const_literal: false,
                no_fn: false,
                comment: true,
                literal: true,
                logic: true,
                impure: false,
                because: true,
                dup: false,
            },
            Zone::Tools => Rules {
                lexical: true,
                no_splice: true,
                no_cfg: true,
                citation: false,
                no_macro: true,
                const_literal: true,
                no_fn: false,
                comment: true,
                literal: true,
                logic: false,
                impure: false,
                because: true,
                dup: true,
            },
            Zone::Tests => Rules {
                lexical: false,
                no_splice: false,
                no_cfg: false,
                citation: false,
                no_macro: false,
                const_literal: false,
                no_fn: false,
                comment: true,
                literal: false,
                logic: false,
                impure: false,
                because: false,
                dup: false,
            },
        }
    }
}

pub const NAMES: &[&str] = &["patterns", "spec", "app", "tools"];
patterns_macros::because!(
    NAMES,
    "the zones premise.zones may name; tests is derived from a crate's own tests directory and never keyed. A key that is none of these is reported rather than ignored, since an ignored key is a file under no law"
);
