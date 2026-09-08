use patterns_macros::because;

pub fn either<T>(when: bool, yes: T, no: T) -> T {
    if when {
        yes
    } else {
        no
    }
}

pub fn is_below<T: PartialOrd>(a: T, b: T) -> bool {
    a < b
}

pub fn is_above<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}

pub fn is_at_most<T: PartialOrd>(a: T, b: T) -> bool {
    a <= b
}

pub fn is_at_least<T: PartialOrd>(a: T, b: T) -> bool {
    a >= b
}

pub fn is_same<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}
because!(either, "the replacement for an if expression in a binding, so a choice between two values is a call and the branch cannot grow a body");
because!(is_below, "a strict ordering test named so a comparison operator never appears in spec or app");
because!(is_above, "the mirror of is_below, named rather than derived because negating a strict comparison flips to a non-strict one and that is a common error");
because!(is_at_most, "the non-strict form of is_below, separate because the boundary case is exactly what a limit turns on");
because!(is_at_least, "the non-strict form of is_above, separate because the boundary case is exactly what a limit turns on");
because!(is_same, "equality as a call, so an equality operator never appears in spec or app");

pub fn either_lazily<T>(when: bool, yes: fn() -> T, no: fn() -> T) -> T {
    if when {
        yes()
    } else {
        no()
    }
}
because!(either_lazily, "the form that builds only the branch it returns, for a choice whose arms are expensive or whose unused arm would panic; either is eager and evaluates both arguments before the call");

pub fn refuse_when<E, F: Clone>(checks: &[(bool, F)], ok: E) -> Result<E, F> {
    for (refused, fault) in checks {
        if *refused {
            return Err(fault.clone());
        }
    }
    Ok(ok)
}
because!(refuse_when, "a table of guards read top to bottom, the first true one refusing with its fault, so a decide arm with several rules is a list in the order the rules apply rather than a nest of either calls that reads inside out; every guard is built before the call, and refuse_when_lazily is the form for a guard that must not run until an earlier one has passed");

pub type Guard<S, C, F> = (fn(&S, &C) -> bool, F);

pub fn refuse_when_lazily<S, C, E, F: Clone>(
    state: &S,
    command: &C,
    checks: &[Guard<S, C, F>],
    ok: E,
) -> Result<E, F> {
    for (refused, fault) in checks {
        if refused(state, command) {
            return Err(fault.clone());
        }
    }
    Ok(ok)
}
because!(refuse_when_lazily, "the guard table of refuse_when with each guard a function of the state and the command, run only until the first refusal, kept beside refuse_when because an eager table builds every guard before the call and a guard that is only safe to evaluate after an earlier one has refused cannot be written that way");

pub fn flipped(value: bool) -> bool {
    !value
}
because!(flipped, "a flag turned over, which a binding cannot write because the negation operator is control flow to the checker, and which the first setting that toggles needs before anything else in a project works");
