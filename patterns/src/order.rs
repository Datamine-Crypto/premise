use crate::arith::{away, Floor};
use crate::select::{either, is_below};
use crate::seq::Ranked;
use core::cmp::Ordering;
use patterns_macros::because;

pub fn highest<T: Ranked + Clone>(items: &[T]) -> Option<T> {
    let mut best: Option<&T> = None;
    for item in items {
        let better = match best {
            Some(held) => item.rank() > held.rank(),
            None => true,
        };
        if better {
            best = Some(item);
        }
    }
    best.cloned()
}
because!(highest, "the element a maximum picks out, kept as an Option so an empty shelf answers with a value, and a tie goes to the earliest so a queue order survives the choice");

pub fn lowest<T: Ranked + Clone>(items: &[T]) -> Option<T> {
    let mut best: Option<&T> = None;
    for item in items {
        let better = match best {
            Some(held) => item.rank() < held.rank(),
            None => true,
        };
        if better {
            best = Some(item);
        }
    }
    best.cloned()
}
because!(lowest, "the mirror of highest, separate because the smallest rank is the next thing due and a caller asking for it should not negate a rank to get there");

pub fn placed<T: Ranked + Clone>(items: &[T], item: T) -> Vec<T> {
    let mut out = Vec::new();
    let mut put = false;
    for held in items {
        if !put && held.rank() > item.rank() {
            out.push(item.clone());
            put = true;
        }
        out.push(held.clone());
    }
    if !put {
        out.push(item);
    }
    out
}
because!(placed, "insertion that keeps a list sorted by rank, with a tie landing after its equals so arrival order is never lost, which with would break by appending");

pub fn ordered<T: Ranked + Clone>(items: &[T]) -> Vec<T> {
    let mut out = items.to_vec();
    out.sort_by(|a, b| a.rank().partial_cmp(&b.rank()).unwrap_or(Ordering::Equal));
    out
}
because!(ordered, "a list sorted by its rank in one stable sort, separate from placed because building a list by repeated insertion copies it once per element, while placed stays the single insert into a list already in order; both land equal ranks in arrival order");

pub fn keep_between<T: Ranked + Clone>(items: &[T], lo: T::Rank, hi: T::Rank) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        let rank = item.rank();
        if rank >= lo && rank < hi {
            out.push(item.clone());
        }
    }
    out
}
because!(keep_between, "the elements inside a window, half open at the top so adjoining windows share no element and a day belongs to one period only");

pub fn split_above<T: Ranked + Clone>(items: &[T], bound: T::Rank) -> (Vec<T>, Vec<T>) {
    let mut over = Vec::new();
    let mut under = Vec::new();
    for item in items {
        if item.rank() > bound {
            over.push(item.clone());
        } else {
            under.push(item.clone());
        }
    }
    (over, under)
}
because!(split_above, "both halves of a partition in one pass, so the two lists are complementary by construction instead of by two filters that may drift apart");

pub fn nearest<T: Floor + Copy + PartialOrd>(want: T, from: &[T]) -> Option<T> {
    from.iter()
        .copied()
        .reduce(|best, one| either(is_below(away(one, want), away(best, want)), one, best))
}
because!(nearest, "the candidate closest to what was wanted, which is asked once by anything choosing a note, a step, a bay or a size from a set that does not hold the exact answer; a tie keeps the one found first, so the order of the candidates decides it and the caller can say what that order is");
