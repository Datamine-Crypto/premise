use patterns_macros::{because, source};

pub struct SplitMixSearch;
source!(
    SplitMixSearch,
    "the avalanche multiplier search by Steele, Lea and Flood"
);

pub struct WeylSequence;
source!(
    WeylSequence,
    "the equidistribution result an odd addend relies on"
);

const GOLDEN_GAMMA: u64 = 0x9E3779B97F4A7C15;
because!(
    GOLDEN_GAMMA,
    WeylSequence,
    "the odd golden ratio addend at this word width, which gives the sequence full period"
);

const MIX_A: u64 = 0xBF58476D1CE4E5B9;
because!(
    MIX_A,
    SplitMixSearch,
    "the first multiplier it found"
);

const MIX_B: u64 = 0x94D049BB133111EB;
because!(
    MIX_B,
    SplitMixSearch,
    "the second multiplier it found"
);

const SHIFT_HIGH: u32 = 30;
because!(
    SHIFT_HIGH,
    SplitMixSearch,
    "the first xor shift distance it found, which moves high bits down before the multiply"
);

const SHIFT_MID: u32 = 27;
because!(
    SHIFT_MID,
    SplitMixSearch,
    "the second xor shift distance, tuned against avalanche bias"
);

const SHIFT_LOW: u32 = 31;
because!(
    SHIFT_LOW,
    SplitMixSearch,
    "the final xor shift distance it found, which spreads the top bit across the word"
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Seed(pub u64);

pub fn next(seed: Seed) -> (u64, Seed) {
    let moved = seed.0.wrapping_add(GOLDEN_GAMMA);
    let a = (moved ^ (moved >> SHIFT_HIGH)).wrapping_mul(MIX_A);
    let b = (a ^ (a >> SHIFT_MID)).wrapping_mul(MIX_B);
    (b ^ (b >> SHIFT_LOW), Seed(moved))
}

pub fn below(seed: Seed, bound: u64) -> (u64, Seed) {
    let (value, moved) = next(seed);
    let scaled = (value as u128 * bound as u128) >> u64::BITS;
    (scaled as u64, moved)
}

pub fn pick<T: Clone>(seed: Seed, items: &[T]) -> (Option<T>, Seed) {
    if items.is_empty() {
        return (None, seed);
    }
    let (at, moved) = below(seed, items.len() as u64);
    (Some(items[at as usize].clone()), moved)
}
because!(next, "the next value from the generator, exposed so a caller advances the sequence without reaching into its state");
because!(below, "a value under a bound taken from the high bits of the word by one multiply, which never divides, so a zero bound yields zero rather than a panic; a bound that does not divide the word leaves the same slight bias a remainder would, and that is not removed here");
because!(pick, "choosing an element by index from a slice, so a caller never computes an index itself");
