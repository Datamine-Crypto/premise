use std::path::{Path, PathBuf};

const SPEC_SMUGGLES: &[(&str, &str)] = &[
    ("pub const S_PLAIN: u32 = 9;\n", "E-NO-BECAUSE"),
    ("pub const S_BLOCK: u32 = { let v = 9; v };\n", "E-NO-BECAUSE"),
    ("pub const S_FIELD: u32 = (9, 0).0;\n", "E-NO-BECAUSE"),
    ("pub const S_UNWRAP: u32 = Some(9).unwrap();\n", "E-NO-BECAUSE"),
    ("pub const S_POW: u32 = 9u32.pow(1);\n", "E-NO-BECAUSE"),
    ("pub enum SmugTier { Bronze = 100, Gold = 250 }\n", "E-NO-BECAUSE"),
    (
        "pub const S_ASSEMBLED: u32 = 2 * 2 * 2 + 1;\nbecause!(S_ASSEMBLED, \"spelled as arithmetic on literals\");\n",
        "E-ASSEMBLED-NUMBER",
    ),
    ("pub const S_FILE: &str = include_str!(\"tariff.txt\");\n", "E-SPLICE"),
    ("pub const S_ENV: &str = env!(\"CARGO_PKG_NAME\");\n", "E-SPLICE"),
    ("pub fn smuggled_fn() -> u32 { S_NAMED }\npub const S_NAMED: u32 = 1;\n", "E-SPEC-FN"),
    (
        "pub trait SmugPricing { fn price(&self) -> u32 { 1 } }\n",
        "E-SPEC-FN",
    ),
    ("#[cfg(test)]\npub fn smug_helper() -> u32 { 1 }\n", "E-SPEC-FN"),
    ("// a smuggled comment\n", "E-COMMENT"),
    ("#[doc = \"a smuggled doc\"]\npub struct SmugDoc;\n", "E-COMMENT"),
    (
        "pub struct SmugSource;\nsource!(SmugSource, \"a source nothing cites\");\n",
        "E-DEAD-SOURCE",
    ),
    (
        "pub const S_DATED: u32 = 1;\nbecause!(S_DATED, \"the figure the 2024 review set\");\n",
        "E-FACT-IN-REASON",
    ),
    (
        "pub struct SmugDock;\nbecause!(SmugDock, \"the dock the tariff review created\");\n",
        "E-UNDECLARED-SOURCE",
    ),
    (
        "pub const S_WEAK: u32 = 1;\nbecause!(S_WEAK, \"weak\");\n",
        "E-WEAK-REASON",
    ),
    ("#[cfg(feature = \"smug\")]\npub const S_GATED: u32 = 1;\n", "E-CONDITIONAL"),
    ("macro_rules! smug { () => { 9 } }\n", "E-MACRO-DEF"),
    ("pub const S_WIDEST: u32 = u32::MAX;\n", "E-NO-BECAUSE"),
    ("pub const S_LEAF: u32 = LOAN_DAYS * 15;\n", "E-NO-BECAUSE"),
    ("const _: () = assert!(LOAN_DAYS == 21);\n", "E-INLINE-LITERAL"),
    (
        "pub trait SmugCap { const MAX: u32; }\nimpl SmugCap for Fines { const MAX: u32 = 500; }\n",
        "E-NO-BECAUSE",
    ),
    ("impl Fines { pub const SMUG_MAX: u32 = 500; }\n", "E-NO-BECAUSE"),
    ("pub const S_THRICE: u32 = LOAN_DAYS + LOAN_DAYS + LOAN_DAYS;\nbecause!(S_THRICE, \"three loan terms laid end to end\");\n", "E-ASSEMBLED-NUMBER"),
    ("const S_PINNED: () = assert!(LOAN_DAYS == 21);\nbecause!(S_PINNED, \"the assertion that pins the loan term where nothing else states it\");\n", "E-INLINE-LITERAL"),
    ("#[deprecated(note = \"the rate doubled after the spring review and nobody moved it\")]\npub const S_OLD: u32 = 1;\nbecause!(S_OLD, \"what an overdue day cost before the desk repriced it\");\n", "E-COMMENT"),
    ("#[allow(dead_code, reason = \"kept because the branch will reinstate the old rate next spring\")]\npub const S_LINT: u32 = 1;\nbecause!(S_LINT, \"what an overdue day cost before the desk repriced it\");\n", "E-COMMENT"),
    ("const S_PIN: bool = LOAN_DAYS == 21;\nbecause!(S_PIN, \"the check that pins the loan term where nothing else states it\");\n", "E-INLINE-LITERAL"),
    ("pub struct SmugTwo;\nbecause!(SmugTwo, SmugBar, SmugCap2, \"the probe whose bar and cap the two tables decide\");\npub trait SmugBar { const BAR: u32; }\npub trait SmugCap2 { const CAP: u32; }\nimpl SmugBar for SmugTwo { const BAR: u32 = 250; }\nimpl SmugCap2 for SmugTwo { const CAP: u32 = 500; }\n", "E-NO-BECAUSE"),
    ("const S_GAP: u32 = LOAN_DAYS - 21;\nbecause!(S_GAP, \"the gap between the term and the figure the desk quoted\");\nconst _: () = assert!(S_GAP == 0);\n", "E-INLINE-LITERAL"),
    ("pub const SMUG_ONE: u32 = 1;\nbecause!(SMUG_ONE, \"the unit every other tariff figure is stated in\");\npub const S_SHIFT: u32 = (SMUG_ONE << SMUG_ONE) << SMUG_ONE;\n", "E-ASSEMBLED-NUMBER"),
    ("pub mod smuggled { pub struct State { pub n: u32 } }\n", "E-CONTEXT-LEAK"),
    ("use std::f64::consts::*;\npub struct SmugPi;\nbecause!(SmugPi, \"the probe whose rank is a circle constant from outside\");\nimpl Ranked for SmugPi { type Rank = f64; fn rank(&self) -> f64 { PI } }\n", "E-INLINE-LITERAL"),
    ("const S_GAP2: u32 = LOAN_DAYS - 21;\nbecause!(S_GAP2, \"the gap between the term and the figure the desk quoted\");\nconst S_HOLDS: bool = !(S_GAP2 != 0);\nbecause!(S_HOLDS, \"the check that the term and the quoted figure agree\");\nconst _: () = assert!(S_HOLDS);\n", "E-INLINE-LITERAL"),
    ("pub const S_TWO: u32 = LOAN_DAYS / LOAN_DAYS + SMUG_UNIT;\npub const SMUG_UNIT: u32 = 1;\nbecause!(SMUG_UNIT, \"the unit every other tariff figure is stated in\");\n", "E-ASSEMBLED-NUMBER"),
    ("pub const LOAN_D\u{0410}YS: u32 = 5;\nbecause!(LOAN_D\u{0410}YS, \"a second term that reads as the first to a person and not to a checker\");\n", "E-IDENT-SCRIPT"),
    ("pub const S_BLOCKED: u32 = { LOAN_DAYS - 21 };\nbecause!(S_BLOCKED, \"the gap between the term and the figure the desk quoted\");\nconst _: () = assert!(S_BLOCKED == 0);\n", "E-INLINE-LITERAL"),
    ("pub const S_DAYS_AGAIN: u32 = LOAN_DAYS;\npub const S_ALIAS_TWO: u32 = LOAN_DAYS / S_DAYS_AGAIN + LOAN_DAYS / S_DAYS_AGAIN;\n", "E-ASSEMBLED-NUMBER"),
    ("pub type SmugWord = u32;\n", "E-NO-BECAUSE"),
    ("pub use super::*;\n", "E-CONTEXT-LEAK"),
    ("pub const S_TEXTED: &str = \"15 pence a day, capped at 500\";\nbecause!(S_TEXTED, \"the tariff line the desk prints on the receipt\");\n", "E-FACT-IN-REASON"),
    ("#[allow(clippy::the_rate_doubled_after_the_spring_review_and_nobody_moved_it)]\npub const S_TOOLED: u32 = 1;\nbecause!(S_TOOLED, \"what an overdue day cost before the desk repriced it\");\n", "E-COMMENT"),
    ("#[allow(unknown_lints, the_rate, doubled_after, the_spring, review_and, nobody_moved_it)]\npub const S_LIFTED: u32 = 1;\nbecause!(S_LIFTED, \"what an overdue day cost before the desk repriced it\");\n", "E-COMMENT"),
    ("const _: &str = \"the rate doubled after the spring review and nobody moved it, so this line explains why\";\n", "E-COMMENT"),
    ("pub const S_TABLE: [u32; 2] = [15, 20];\nbecause!(S_TABLE, \"the two daily rates the desk has used, oldest first\");\n", "E-NO-BECAUSE"),
    ("pub const S_LINE: &str = \"the line the desk prints\";\nbecause!(S_LINE, \"the line the desk prints on a receipt, kept as one string\");\n", "E-INLINE-LITERAL"),
    ("pub const S_UNIT: u32 = 1;\nbecause!(S_UNIT, \"the unit every other tariff figure is stated in\");\npub const S_BIG: u32 = S_UNIT << LOAN_DAYS;\n", "E-ASSEMBLED-NUMBER"),
    ("pub struct TheRateDoubledAfterTheSpringReviewAndNobodyMovedIt;\nbecause!(TheRateDoubledAfterTheSpringReviewAndNobodyMovedIt, \"a marker whose name is the sentence its author wanted to write\");\n", "E-COMMENT"),
    ("pub enum SmugRate {\n    Old = 15,\n    New = 20,\n}\nbecause!(SmugRate, \"the two daily rates the desk has used, oldest first, as a vocabulary\");\n", "E-NO-BECAUSE"),
    ("pub type SmugAlias = crate::contexts::lockers::vocabulary::Lockers;\n", "E-CONTEXT-LEAK"),
    ("pub const S_ONE_MORE: u32 = 1;\nbecause!(S_ONE_MORE, \"the unit every other tariff figure is stated in\");\npub const S_UNIT_AGAIN: u32 = S_ONE_MORE;\npub const S_BIGGER: u32 = S_UNIT_AGAIN << LOAN_DAYS;\n", "E-ASSEMBLED-NUMBER"),
    ("pub const THE_RATE_DOUBLED_AFTER_THE: u32 = 1;\nbecause!(THE_RATE_DOUBLED_AFTER_THE, \"the unit every other tariff figure is stated in\");\n", "E-COMMENT"),
    ("pub trait SmugCaps {\n    const LO: u32 = 15;\n    const HI: u32 = 500;\n}\nbecause!(SmugCaps, \"the floor and the ceiling of a charge, both settled by the desk at once\");\n", "E-NO-BECAUSE"),
    ("pub struct SmugPeek(pub crate::contexts::lockers::vocabulary::Lockers);\nbecause!(SmugPeek, \"a holder of another context's marker, to see whether naming it is refused\");\n", "E-CONTEXT-LEAK"),
];

const APP_SMUGGLES: &[(&str, &str)] = &[
    ("pub fn s_lit() -> u32 { 42 }\n", "E-INLINE-LITERAL"),
    ("pub fn s_hex() -> Vec<u32> { vec![0x2a] }\n", "E-INLINE-LITERAL"),
    ("pub fn s_float() -> Vec<f64> { vec![0.5] }\n", "E-INLINE-LITERAL"),
    ("pub fn s_prim() -> u32 { u32::MAX }\n", "E-INLINE-LITERAL"),
    ("const S_HIDDEN: u32 = 7;\npub fn s_named() -> u32 { S_HIDDEN }\n", "E-INLINE-LITERAL"),
    ("pub fn s_matches(a: u32) -> bool { matches!(a, 1) }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_if(a: bool) -> bool { if a { false } else { true } }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_method(a: &[u32]) -> usize { a.len() }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_op(a: u32) -> u32 { a + 1 }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_closure(a: &[u32]) -> bool { a.iter().any(|x| *x == 0) }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_match(a: Option<u32>) -> bool { match a { Some(_) => true, None => false } }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_cast(a: u32) -> usize { a as usize }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_index(a: &[u32]) -> u32 { a[0] }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_loop(a: &[u32]) -> u32 { for x in a { return *x; } 0 }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_foreign() -> String { String::new() }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_try(a: Option<u32>) -> Option<u32> { Some(a?) }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_vec(state: &State) -> Vec<usize> { vec![state.loans.len()] }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_assign(state: &mut State) { state.today = 0; }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub fn s_core() -> u32 { core::u32::MAX }\n", "E-INLINE-LITERAL"),
    ("use std::cmp as patterns;\npub fn s_shadow(a: u32, b: u32) -> u32 { patterns::max(a, b) }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("mod patterns { pub use std::cmp::max; }\npub fn s_shadow_mod(a: u32, b: u32) -> u32 { patterns::max(a, b) }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub async fn s_later(n: u32) -> u32 { n }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("use std::cmp::*;\npub fn s_glob(a: u32, b: u32) -> u32 { max(a, b) }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("pub type SmugWord = u32;\n", "E-INLINE-LITERAL"),
    ("pub fn s_alias() -> std::time::Duration { std::time::Duration::MAX }\n", "E-INLINE-LITERAL"),
    ("use spec::contexts::fines::vocabulary::{ADULT_FINE_CAP_PENCE, BORROW_BAR_PENCE};\nuse patterns::is_below;\npub fn s_typed(owed: u32) -> bool { let bar: u32 = BORROW_BAR_PENCE; is_below(owed, bar) }\npub fn s_typed_cap(owed: u32) -> bool { is_below(owed, ADULT_FINE_CAP_PENCE) }\n", "E-CASE-LEAK"),
    ("use spec::contexts::fines::vocabulary::{ADULT_FINE_CAP_PENCE, BORROW_BAR_PENCE};\nuse patterns::is_below;\npub fn s_block(owed: u32) -> bool { let bar = { BORROW_BAR_PENCE }; is_below(owed, bar) }\npub fn s_block_cap(owed: u32) -> bool { is_below(owed, ADULT_FINE_CAP_PENCE) }\n", "E-CASE-LEAK"),
    ("use std::f64::consts::PI;\npub fn s_pi() -> f64 { PI }\n", "E-INLINE-LITERAL"),
    ("use spec::*;\nuse std::time::Duration;\npub fn s_glob_max() -> Duration { Duration::MAX }\n", "E-INLINE-LITERAL"),
    ("use spec::*;\nuse std::num::*;\npub fn s_bits() -> u32 { NonZeroU32::BITS }\n", "E-INLINE-LITERAL"),
    ("use std::f64::consts::*;\npub fn s_glob_pi() -> f64 { PI }\n", "E-INLINE-LITERAL"),
    ("use std::time::*;\npub fn s_qself() -> Duration { <Duration>::MAX }\n", "E-INLINE-LITERAL"),
    ("pub fn s_qualified(n: u32) -> u32 { <spec::contexts::fines::vocabulary::Grade as patterns::Named>::text(&spec::contexts::fines::vocabulary::Grade::Adult).len() as u32 }\n", "E-LOGIC-OUTSIDE-PATTERN"),
    ("use spec::contexts::fines::vocabulary::{ADULT_FINE_CAP_PENCE, BORROW_BAR_PENCE};\nuse patterns::is_below;\npub fn s_chain(owed: u32) -> bool { let bar = BORROW_BAR_PENCE; let cap = bar; is_below(owed, cap) }\npub fn s_chain_cap(owed: u32) -> bool { is_below(owed, ADULT_FINE_CAP_PENCE) }\n", "E-CASE-LEAK"),
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let p = entry.path();
        let name = entry.file_name();
        if p.is_dir() {
            if name != "target" {
                copy_tree(&p, &to.join(name));
            }
        } else {
            std::fs::copy(&p, to.join(name)).unwrap();
        }
    }
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("premise_smuggle_{}_{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for crate_name in ["spec", "app", "patterns"] {
        copy_tree(&root().join(crate_name).join("src"), &dir.join(crate_name).join("src"));
    }
    std::fs::write(dir.join("premise.zones"), "spec = spec\napp = app\npatterns = patterns\n").unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\nmembers = [\"spec\", \"app\", \"patterns\"]\n",
    )
    .unwrap();
    for crate_name in ["spec", "app", "patterns"] {
        std::fs::write(
            dir.join(crate_name).join("Cargo.toml"),
            format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
                crate_name
            ),
        )
        .unwrap();
    }
    dir
}

fn first_file(dir: &Path, leaf: &str) -> PathBuf {
    let mut stack = vec![dir.to_path_buf()];
    let mut found: Vec<PathBuf> = Vec::new();
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().map(|n| n == leaf).unwrap_or(false) {
                found.push(p);
            }
        }
    }
    found.sort();
    found.into_iter().next().expect("the live tree has such a file")
}

fn attack_in_lanes(
    tag: &str,
    target_of: impl Fn(&Path) -> PathBuf + Sync,
    smuggles: &[(&str, &str)],
) -> Vec<String> {
    let lanes = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(smuggles.len())
        .max(1);
    let each = smuggles.len().div_ceil(lanes);
    let mut missed: Vec<String> = Vec::new();
    let held = &target_of;
    std::thread::scope(|lane| {
        let mut running = Vec::new();
        for (at, part) in smuggles.chunks(each).enumerate() {
            running.push(lane.spawn(move || {
                let dir = scratch(&format!("{}_{}", tag, at));
                let target = held(&dir);
                let out = attack(&dir, &target, part);
                let _ = std::fs::remove_dir_all(&dir);
                out
            }));
        }
        for one in running {
            missed.extend(one.join().expect("a lane finishes"));
        }
    });
    missed.sort();
    missed
}

fn attack(dir: &Path, target: &Path, smuggles: &[(&str, &str)]) -> Vec<String> {
    let original = std::fs::read_to_string(target).unwrap();
    let rel = target
        .strip_prefix(dir)
        .unwrap()
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/");
    let mut missed = Vec::new();
    for (snippet, code) in smuggles {
        std::fs::write(target, format!("{}\n{}", original, snippet)).unwrap();
        let caught = checker::run(dir)
            .iter()
            .any(|d| d.code == *code && d.file == rel);
        if !caught {
            missed.push(format!("{} was not caught as {} in {}", snippet.trim(), code, rel));
        }
    }
    std::fs::write(target, original).unwrap();
    missed
}

#[test]
fn every_known_smuggle_into_the_live_spec_is_caught() {
    let missed = attack_in_lanes(
        "spec",
        |dir: &Path| first_file(&dir.join("spec"), "vocabulary.rs"),
        SPEC_SMUGGLES,
    );
    assert!(missed.is_empty(), "{}", missed.join("\n"));
}

#[test]
fn every_known_smuggle_into_the_live_app_is_caught() {
    let missed = attack_in_lanes(
        "app",
        |dir: &Path| first_file(&dir.join("app").join("src"), "fines.rs"),
        APP_SMUGGLES,
    );
    assert!(missed.is_empty(), "{}", missed.join("\n"));
}

#[test]
fn the_live_tree_copy_is_clean_before_any_smuggle() {
    let dir = scratch("clean");
    let got: Vec<String> = checker::run(&dir).iter().map(|d| d.to_string()).collect();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(got.is_empty(), "the copied tree is not clean, so the smuggle suite cannot tell its own hits from the tree's: {:?}", got);
}
