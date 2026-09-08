use patterns_macros::because;

pub trait Floor: Sized {
    const ZERO: Self;
    const ONE: Self;
    fn sub_to_floor(self, taken: Self) -> Self;
    fn add_to_ceiling(self, given: Self) -> Self;
    fn mul_to_ceiling(self, times: Self) -> Self;
    fn checked_div(self, by: Self) -> Option<Self>;
    fn checked_rem(self, by: Self) -> Option<Self>;
}

macro_rules! floor_for {
    ($($t:ty),*) => {
        $(
            impl Floor for $t {
                const ZERO: $t = 0;
                const ONE: $t = 1;
                fn sub_to_floor(self, taken: $t) -> $t {
                    self.saturating_sub(taken)
                }
                fn add_to_ceiling(self, given: $t) -> $t {
                    self.saturating_add(given)
                }
                fn mul_to_ceiling(self, times: $t) -> $t {
                    self.saturating_mul(times)
                }
                fn checked_div(self, by: $t) -> Option<$t> {
                    <$t>::checked_div(self, by)
                }
                fn checked_rem(self, by: $t) -> Option<$t> {
                    <$t>::checked_rem(self, by)
                }
            }
        )*
    };
}

floor_for!(u8, u16, u32, u64, u128, usize);

pub fn reduce_by<T: Floor>(value: T, amount: T) -> T {
    value.sub_to_floor(amount)
}

pub fn raise_by<T: Floor>(value: T, amount: T) -> T {
    value.add_to_ceiling(amount)
}

pub fn mul<T: Floor>(value: T, times: T) -> T {
    value.mul_to_ceiling(times)
}

pub fn div<T: Floor>(value: T, by: T) -> Option<T> {
    value.checked_div(by)
}

pub fn rem<T: Floor>(value: T, by: T) -> Option<T> {
    value.checked_rem(by)
}
because!(Floor, "the arithmetic an unsigned integer does at its own edges, gathered on one trait so every pattern that adds, takes, scales or divides answers the overflow and zero cases in one place; signed types are left out because stopping at zero and folding into a range are only true of a type with nothing under zero");
because!(reduce_by, "subtraction that stops at the type floor, because a quantity that cannot go below zero should say so once here rather than at every call");
because!(raise_by, "the mirror of reduce_by, stopping at the type ceiling so an overflow is a saturation rather than a panic or a wrap");
because!(mul, "scaling one quantity by another, stopping at the type ceiling, because a rate times a term is money and a product that wraps to something smaller or ends the process is worse than one pinned at the top");
because!(div, "integer division that answers None on a zero divisor, so the caller decides what an undefined quotient means instead of the process ending; the quotient truncates, and a fraction that matters is a value the spec states");
because!(rem, "the remainder division discards, answering None on a zero divisor as div does, needed on its own wherever a cycle position or an evenness check is the fact");

pub fn doubled<T: Floor + Copy>(value: T) -> T {
    raise_by(value, value)
}
because!(doubled, "a value added to itself, which a binding cannot write, since two is not a literal it may use and a name added to itself is refused as a spelling of a multiple; the add saturates, so a doubling at the ceiling stays there rather than wrapping to nothing");

pub fn half<T: Floor>(value: T) -> T {
    div(value, raise_by(T::ONE, T::ONE)).unwrap_or(T::ZERO)
}
because!(half, "a value split in two, the other side of doubled, written here because two is not a literal a binding may write; the divisor is one added to one and so is never zero, which is why this answers with the value rather than with a maybe");

pub fn away<T: Floor + Copy>(a: T, b: T) -> T {
    raise_by(reduce_by(a, b), reduce_by(b, a))
}
because!(away, "how far apart two values are without first asking which is larger, since a subtraction that stops at the floor gives nothing in the wrong direction and the distance in the right one, so the two of them add to the distance whichever way round they came");
