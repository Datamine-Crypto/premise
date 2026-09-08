use patterns_macros::because;
use std::collections::{BTreeMap, BTreeSet};

#[derive(patterns_macros::Record)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Award {
    pub count: u64,
    pub first: u64,
    pub last: u64,
}
because!(Award, "how many have earned one badge and when the first and the last did, the row a badge's leaderboard reads");

#[derive(patterns_macros::Record)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Earned<K, A> {
    pub who: K,
    pub badge: A,
    pub at: u64,
    pub rank: u64,
}
because!(Earned, "one account earning one badge at a moment, ranked by how many had earned it before, the row an account page lists");

pub type Given<K, A> = BTreeSet<(K, A)>;
because!(Given, "who has already been given which award, kept as a set of pairs so the question a grant asks, whether this one has had this already, is the set own membership test");

pub type Awards<A> = BTreeMap<A, Award>;
because!(Awards, "how many of each award have been given and when the first and last went out, which is what a rank is counted from");

pub type Granted<K, A> = (Given<K, A>, Awards<A>, Vec<Earned<K, A>>);
because!(Granted, "what a round of granting leaves behind: the pairs now given, the counts now raised, and the rows the round produced, returned together because a caller that took one and not the others would be holding a record of a grant that never happened");

pub fn awarded<K: Ord + Clone, A: Ord + Copy>(given: Given<K, A>, awards: Awards<A>, candidates: &[(K, A)], at: u64) -> Granted<K, A> {
    let mut given = given;
    let mut awards = awards;
    let mut rows = Vec::new();
    for (who, badge) in candidates {
        if !given.insert((who.clone(), *badge)) {
            continue;
        }
        let award = awards.entry(*badge).or_default();
        award.count += 1;
        if award.first == 0 {
            award.first = at;
        }
        award.last = award.last.max(at);
        rows.push(Earned {
            who: who.clone(),
            badge: *badge,
            at,
            rank: award.count,
        });
    }
    (given, awards, rows)
}
because!(awarded, "candidates granted once each and never again, each taking the next rank of its badge, in the order offered, so a badge earned twice in one batch counts once and the rank an account keeps is the one it earned at the time");
