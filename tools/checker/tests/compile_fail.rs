use std::path::PathBuf;
use std::process::Output;

const CASES: &[(&str, &str)] = &[
    ("dup_because", "E0428"),
    ("conflict_impl", "E0119"),
    ("raw_because", "E0428"),
    ("ghost_rejected", "E0432"),
    ("lint_prose", "unknown lint"),
];

const MALFORMED: &str = "malformed";

const COMPLAINTS: &[&str] = &[
    "because! takes item names, not paths",
    "because! found two slots with no comma between them",
    "because! takes string literals only",
    "source! has more slots than its shape takes",
    "rejected! is missing a slot",
    "because! slot 3 wants a string, got an item name",
    "because! slot 2 wants an item name, got a string",
    "because! found an empty slot before a comma",
    "provisional! takes item names, commas and string literals only",
    "supersedes! is missing a slot",
    "decided! slot 2 wants an item name, got a string",
    "usage: because!(ITEM, [CITE, ...], \"reason\")",
    "usage: source!(ITEM, \"what it is\")",
    "usage: provisional!(ITEM, [CITE, ...], \"what would settle it\")",
    "usage: rejected!(ITEM, \"alternative\", \"cost\")",
    "usage: supersedes!(NEW, OLD, WHEN, \"why\")",
    "usage: decided!(TYPE, TRAIT, \"reason\")",
];

const BUILDS: &[&str] = &["raw_supersedes"];

fn nested_target(tag: &str) -> PathBuf {
    let at = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("nested")
        .join(tag);
    std::fs::create_dir_all(&at).unwrap();
    at
}

fn build(dir: &str) -> Output {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("compile_fail")
        .join(dir)
        .join("Cargo.toml");
    std::process::Command::new(env!("CARGO"))
        .arg("build")
        .arg("--manifest-path")
        .arg(&manifest)
        .env("CARGO_TARGET_DIR", nested_target("compile_fail"))
        .output()
        .expect("cargo runs")
}

#[test]
fn the_compiler_rejects_every_restatement() {
    for (dir, code) in CASES {
        let out = build(dir);
        assert!(!out.status.success(), "{} was expected to fail", dir);
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(err.contains(code), "{} expected {}, got:\n{}", dir, code, err);
    }
}

#[test]
fn every_malformed_call_is_named_by_its_shape() {
    let out = build(MALFORMED);
    assert!(!out.status.success(), "{} was expected to fail", MALFORMED);
    let err = String::from_utf8_lossy(&out.stderr);
    for want in COMPLAINTS {
        assert!(err.contains(want), "{} expected {:?}, got:\n{}", MALFORMED, want, err);
    }
}

#[test]
fn every_well_formed_call_builds() {
    for dir in BUILDS {
        let out = build(dir);
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(out.status.success(), "{} was expected to build, got:\n{}", dir, err);
    }
}
