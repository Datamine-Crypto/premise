use std::path::PathBuf;

const CASES: &[(&str, &str)] = &[
    ("comment", "E-COMMENT"),
    ("docattr", "E-COMMENT"),
    ("literal", "E-INLINE-LITERAL"),
    ("logic", "E-LOGIC-OUTSIDE-PATTERN"),
    ("methodcall", "E-LOGIC-OUTSIDE-PATTERN"),
    ("nobecause", "E-NO-BECAUSE"),
    ("duppattern", "E-DUP-PATTERN"),
    ("impure", "E-PATTERN-IMPURE"),
    ("ufcs", "E-LOGIC-OUTSIDE-PATTERN"),
    ("specfn", "E-SPEC-FN"),
    ("weakreason", "E-WEAK-REASON"),
    ("customzones", "E-INLINE-LITERAL"),
    ("speclogic", "E-LOGIC-OUTSIDE-PATTERN"),
    ("negate", "E-LOGIC-OUTSIDE-PATTERN"),
    ("fakecontext", "E-LOGIC-OUTSIDE-PATTERN"),
    ("nearpattern", "E-NEAR-PATTERN"),
    ("appconst", "E-INLINE-LITERAL"),
    ("macrodef", "E-MACRO-DEF"),
    ("boundswap", "E-DUP-PATTERN"),
    ("attack", "E-LOGIC-OUTSIDE-PATTERN"),
    ("appstring", "E-INLINE-LITERAL"),
    ("evade", "E-NEAR-PATTERN"),
    ("ctxbypass", "E-LOGIC-OUTSIDE-PATTERN"),
    ("upper", "E-LOGIC-OUTSIDE-PATTERN"),
    ("implclone", "E-DUP-PATTERN"),
    ("boolmatch", "E-LOGIC-OUTSIDE-PATTERN"),
    ("implbecause", "E-ORPHAN-REASON"),
    ("zonedep", "E-ZONE-DEPENDS"),
    ("attack2", "E-LOGIC-OUTSIDE-PATTERN"),
    ("wildmatch", "E-SPEC-FN"),
    ("srctests", "E-LOGIC-OUTSIDE-PATTERN"),
    ("leak", "E-CONTEXT-LEAK"),
    ("globws", "E-ZONE-DEPENDS"),
    ("dupvocab", "E-DUP-VOCABULARY"),
    ("sharedscope", "E-SHARED-SCOPE"),
    ("shouty", "E-LOGIC-OUTSIDE-PATTERN"),
    ("splice", "E-SPLICE"),
    ("shadow", "E-DUP-VOCABULARY"),
    ("shouty2", "E-LOGIC-OUTSIDE-PATTERN"),
    ("sharedmod", "E-SHARED-SCOPE"),
    ("dupname", "E-DUP-NAME"),
    ("macrosplice", "E-SPLICE"),
    ("drift2", "E-DUP-VOCABULARY"),
    ("drift3", "E-DUP-VOCABULARY"),
    ("metasplice", "E-SPLICE"),
    ("rawident", "E-SPLICE"),
    ("rawmacro", "E-MACRO-DEF"),
    ("rawdup", "E-DUP-PATTERN"),
    ("aliassplice", "E-SPLICE"),
    ("aliasmeta", "E-SPLICE"),
    ("chainsplice", "E-SPLICE"),
    ("reversechain", "E-SPLICE"),
    ("assembled", "E-ASSEMBLED-NUMBER"),
    ("staticfact", "E-NO-BECAUSE"),
    ("dupreason", "E-DUP-REASON"),
    ("scoping", "E-SPEC-FN"),
    ("orphan", "E-ORPHAN-FILE"),
    ("conditional", "E-CONDITIONAL"),
    ("outside", "E-OUTSIDE-SRC"),
    ("modcase", "E-MODULE-CASE"),
    ("nested", "E-ORPHAN-FILE"),
    ("libpath", "E-ORPHAN-FILE"),
    ("toolscfg", "E-CONDITIONAL"),
    ("cfgbang", "E-CONDITIONAL"),
    ("pathinline", "E-ORPHAN-FILE"),
    ("pathblock", "E-ORPHAN-FILE"),
    ("w2", "E-OUTSIDE-SRC"),
    ("w3", "E-ORPHAN-FILE"),
    ("w7", "E-CRATE-NESTED"),
    ("autobins", "E-ORPHAN-FILE"),
    ("c1", "E-CONDITIONAL"),
    ("d1", "E-DEFAULT-MEMBERS"),
    ("p1", "E-MACRO-DEF"),
    ("n3", "E-OUTSIDE-SRC"),
    ("n4", "E-OUTSIDE-WORKSPACE"),
    ("x2", "E-MACRO-DEF"),
    ("y6", "E-OUTSIDE-LAW"),
    ("allfacts", "E-NO-BECAUSE"),
    ("z5", "E-NO-BECAUSE"),
    ("implname", "E-DUP-NAME"),
    ("caseleak", "E-CASE-LEAK"),
    ("reqfeat", "E-ORPHAN-FILE"),
    ("benchsrc", "E-ORPHAN-FILE"),
    ("zonekey", "E-ZONE-KEY"),
    ("launder", "E-NO-BECAUSE"),
    ("discriminant", "E-NO-BECAUSE"),
    ("blanketimpl", "E-NO-BECAUSE"),
    ("bodylit", "E-INLINE-LITERAL"),
    ("includestr", "E-SPLICE"),
    ("macrologic", "E-LOGIC-OUTSIDE-PATTERN"),
    ("hexlit", "E-INLINE-LITERAL"),
    ("primconst", "E-INLINE-LITERAL"),
    ("aliasleak", "E-CONTEXT-LEAK"),
    ("vocabreexport", "E-CONTEXT-LEAK"),
    ("importreason", "E-ORPHAN-REASON"),
    ("privatedup", "E-DUP-NAME"),
    ("privatevocab", "E-DUP-VOCABULARY"),
    ("planmax", "E-DUP-NAME"),
    ("sharedfile", "E-SHARED-SCOPE"),
    ("sharedstate", "E-SHARED-SCOPE"),
    ("traitdefault", "E-SPEC-FN"),
    ("cfgtestfn", "E-SPEC-FN"),
    ("specassembled", "E-ASSEMBLED-NUMBER"),
    ("testfixturecomment", "E-COMMENT"),
    ("nestedconst", "E-NO-BECAUSE"),
    ("citetype", "E-UNTRACEABLE"),
    ("dupnamed", "E-DUP-PATTERN"),
    ("letleak", "E-CASE-LEAK"),
    ("rejectedfact", "E-FACT-IN-REASON"),
    ("deadsource", "E-DEAD-SOURCE"),
    ("dupsource", "E-DUP-NAME"),
    ("prosesource", "E-UNDECLARED-SOURCE"),
    ("y6", "E-ZONE-DEPENDS"),
    ("nearunexcused", "E-NEAR-PATTERN"),
    ("shadowpatterns", "E-LOGIC-OUTSIDE-PATTERN"),
    ("veclogic", "E-LOGIC-OUTSIDE-PATTERN"),
    ("primspell", "E-INLINE-LITERAL"),
    ("assign", "E-LOGIC-OUTSIDE-PATTERN"),
    ("implbytrait", "E-NO-BECAUSE"),
    ("primspec", "E-NO-BECAUSE"),
    ("leafliteral", "E-NO-BECAUSE"),
    ("constassert", "E-INLINE-LITERAL"),
    ("sharedalias", "E-SHARED-SCOPE"),
    ("derefleak", "E-CASE-LEAK"),
    ("shadowmod", "E-LOGIC-OUTSIDE-PATTERN"),
    ("asyncfn", "E-LOGIC-OUTSIDE-PATTERN"),
    ("globsmuggle", "E-LOGIC-OUTSIDE-PATTERN"),
    ("aliasconst", "E-INLINE-LITERAL"),
    ("inherentimpl", "E-NO-BECAUSE"),
    ("traitreason", "E-NO-BECAUSE"),
    ("repeatunit", "E-ASSEMBLED-NUMBER"),
    ("namedassert", "E-INLINE-LITERAL"),
    ("vocabalias", "E-CONTEXT-LEAK"),
    ("sharedtrait", "E-SHARED-SCOPE"),
    ("typedleak", "E-CASE-LEAK"),
    ("attrprose", "E-COMMENT"),
    ("onereason", "E-NO-BECAUSE"),
    ("boolcheck", "E-INLINE-LITERAL"),
    ("unitsum", "E-ASSEMBLED-NUMBER"),
    ("crateleak", "E-CONTEXT-LEAK"),
    ("lintreason", "E-COMMENT"),
    ("sharedadvance", "E-SHARED-SCOPE"),
    ("importedconst", "E-INLINE-LITERAL"),
    ("blockleak", "E-CASE-LEAK"),
    ("globstd", "E-INLINE-LITERAL"),
    ("vocabmod", "E-CONTEXT-LEAK"),
    ("qselfcall", "E-LOGIC-OUTSIDE-PATTERN"),
    ("stdglobs", "E-INLINE-LITERAL"),
    ("checkconst", "E-INLINE-LITERAL"),
    ("shiftunit", "E-ASSEMBLED-NUMBER"),
    ("chainleak", "E-CASE-LEAK"),
    ("namecollide", "E-INLINE-LITERAL"),
    ("cargoconfig", "E-CARGO-CONFIG"),
    ("thirdfile", "E-CONTEXT-LEAK"),
    ("dupzone", "E-ZONE-KEY"),
    ("specglobpi", "E-INLINE-LITERAL"),
    ("negcheck", "E-INLINE-LITERAL"),
    ("selfdiv", "E-ASSEMBLED-NUMBER"),
    ("homoglyph", "E-IDENT-SCRIPT"),
    ("vocabglob", "E-CONTEXT-LEAK"),
    ("aliaslaunder", "E-INLINE-LITERAL"),
    ("aliasreason", "E-NO-BECAUSE"),
    ("blockcheck", "E-INLINE-LITERAL"),
    ("aliascancel", "E-ASSEMBLED-NUMBER"),
    ("decidedelsewhere", "E-DUP-REASON"),
    ("toolchainpath", "E-CARGO-CONFIG"),
    ("nestedfile", "E-CONTEXT-LEAK"),
    ("selfbridge", "E-CONTEXT-LEAK"),
    ("digitstring", "E-FACT-IN-REASON"),
    ("letcopy", "E-DUP-PATTERN"),
    ("toollint", "E-COMMENT"),
    ("lintopen", "E-COMMENT"),
    ("lintlifted", "E-COMMENT"),
    ("markerleak", "E-CONTEXT-LEAK"),
    ("stringcomment", "E-COMMENT"),
    ("straystring", "E-INLINE-LITERAL"),
    ("bytefact", "E-INLINE-LITERAL"),
    ("littable", "E-NO-BECAUSE"),
    ("manifestcomment", "E-COMMENT"),
    ("legacytoolchain", "E-CARGO-CONFIG"),
    ("uniformbridge", "E-CONTEXT-LEAK"),
    ("retcopy", "E-DUP-PATTERN"),
    ("unitshift", "E-ASSEMBLED-NUMBER"),
    ("proseident", "E-COMMENT"),
    ("patternsdigit", "E-FACT-IN-REASON"),
    ("markeralias", "E-CONTEXT-LEAK"),
    ("markerglob", "E-CONTEXT-LEAK"),
    ("contextbound", "E-CONTEXT-LEAK"),
    ("enumtable", "E-NO-BECAUSE"),
    ("underscorenote", "E-COMMENT"),
    ("arabicdigit", "E-FACT-IN-REASON"),
    ("zonesprose", "E-ZONE-KEY"),
    ("manifestprose", "E-COMMENT"),
    ("fragmentident", "E-COMMENT"),
    ("unitalias", "E-ASSEMBLED-NUMBER"),
    ("unitacross", "E-ASSEMBLED-NUMBER"),
    ("markerchain", "E-CONTEXT-LEAK"),
    ("markerassoc", "E-CONTEXT-LEAK"),
    ("markersuper", "E-CONTEXT-LEAK"),
    ("markershared", "E-CONTEXT-LEAK"),
    ("foreigndecided", "E-CONTEXT-LEAK"),
    ("impltable", "E-NO-BECAUSE"),
    ("traittable", "E-NO-BECAUSE"),
    ("readonce", "E-COMMENT"),
    ("wsprose", "E-COMMENT"),
    ("markercollide", "E-CONTEXT-LEAK"),
    ("markerowned", "E-CONTEXT-LEAK"),
    ("stateless", "E-CONTEXT-LEAK"),
    ("readonce2", "E-COMMENT"),
    ("gluedigit", "E-FACT-IN-REASON"),
    ("renamehop", "E-CONTEXT-LEAK"),
    ("stringstmt", "E-INLINE-LITERAL"),
    ("primfake", "E-FACT-IN-REASON"),
    ("markerdup", "E-DUP-NAME"),
    ("rootmod", "E-LOGIC-OUTSIDE-PATTERN"),
    ("primtext", "E-FACT-IN-REASON"),
    ("deadreason", "E-DEAD-SOURCE"),
    ("deadenum", "E-DEAD-SOURCE"),
    ("deadnested", "E-DEAD-SOURCE"),
    ("eithercopy", "E-DUP-PATTERN"),
    ("libdup", "E-DUP-PATTERN"),
    ("descprose", "E-COMMENT"),
];

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn codes(name: &str) -> Vec<String> {
    checker::run(&fixture(name))
        .into_iter()
        .map(|d| d.code.to_string())
        .collect()
}

#[test]
fn every_rule_fires_on_its_fixture() {
    for (name, code) in CASES {
        let got = codes(name);
        assert!(
            got.iter().any(|c| c == code),
            "fixture {} expected {} got {:?}",
            name,
            code,
            got
        );
    }
}

const SILENT: &[&str] = &[
    "clean", "display", "units", "sharedvocab", "n2b", "z6", "parallelcap", "provcite", "modcall",
    "rootpkg", "door", "nearexcused", "implcited", "globcall", "devdep", "specglob", "squared",
    "decidedtwice", "lazyguard", "decidedstate", "markerconsts", "testattrs", "variantimport", "stateparam", "ownqself", "naturalnames", "localdriver", "sharedglob", "usedbysibling",
    "described",
];

#[test]
fn a_local_module_root_is_not_a_foreign_crate() {
    let got = codes("uniformbridge");
    assert!(
        !got.iter().any(|c| c == "E-OUTSIDE-LAW"),
        "a uniform path into a module declared in the same file was read as a crate: {:?}",
        got
    );
}

#[test]
fn silent_fixtures_report_nothing() {
    for name in SILENT {
        let got = codes(name);
        assert!(got.is_empty(), "fixture {} reported {:?}", name, got);
    }
}

const NAMED: &[(&str, &str, &[&str])] = &[
    ("nested", "E-ORPHAN-FILE", &["spec/src/inner.rs"]),
    ("libpath", "E-ORPHAN-FILE", &["spec/src/ghost.rs"]),
    ("pathattr", "E-ORPHAN-FILE", &["spec/src/ghost.rs"]),
    ("orphan", "E-ORPHAN-FILE", &["spec/src/contexts/door/state.rs"]),
    ("pathinline", "E-ORPHAN-FILE", &["spec/src/target.rs"]),
    ("pathblock", "E-ORPHAN-FILE", &["spec/src/outer/inner.rs", "spec/src/outer_inner_unused.rs"]),
    ("pathchild", "E-ORPHAN-FILE", &["spec/src/sub/entry/leaf.rs"]),
    ("deep2", "E-ORPHAN-FILE", &["spec/src/leaf.rs"]),
    ("rootsub", "E-ORPHAN-FILE", &["spec/src/used.rs"]),
    ("w3", "E-ORPHAN-FILE", &["spec/src/lib.rs"]),
    ("autobins", "E-ORPHAN-FILE", &["spec/src/bin/ghost.rs"]),
];

#[test]
fn the_orphan_report_names_the_file_rustc_skips() {
    for (name, code, want) in NAMED {
        let mut got: Vec<String> = checker::run(&fixture(name))
            .into_iter()
            .filter(|d| d.code == *code)
            .map(|d| d.file.replace(92u8 as char, "/"))
            .collect();
        got.sort();
        let mut expect: Vec<String> = want.iter().map(|s| s.to_string()).collect();
        expect.sort();
        assert_eq!(
            got,
            expect,
            "fixture {} named the wrong file for {}",
            name,
            code
        );
    }
}

#[test]
fn the_checker_reads_only_what_rustc_compiles() {
    let dir = fixture("orphan");
    let live: Vec<String> = checker::compiled_sources(&dir, checker::zone::Zone::Spec)
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();
    assert!(
        live.contains(&String::from("vocabulary.rs")),
        "a declared file was hidden from the checker: {:?}",
        live
    );
    assert!(
        !live.contains(&String::from("state.rs")),
        "the checker still reads a file rustc never compiles: {:?}",
        live
    );
}

#[test]
fn a_directory_cycle_reports_rather_than_crashes() {
    let dir = std::env::temp_dir().join(format!("premise_cycle_probe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("spec").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"spec\"]\n").unwrap();
    std::fs::write(dir.join("premise.zones"), "spec = spec\n").unwrap();
    std::fs::write(
        dir.join("spec").join("Cargo.toml"),
        "[package]\nname = \"spec\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    std::fs::write(src.join("lib.rs"), "pub const A: u32 = 1;\n").unwrap();
    #[cfg(windows)]
    let made = std::os::windows::fs::symlink_dir(dir.join("spec"), src.join("ring")).is_ok();
    #[cfg(not(windows))]
    let made = std::os::unix::fs::symlink(dir.join("spec"), src.join("ring")).is_ok();
    #[cfg(not(windows))]
    assert!(made, "a symlink could not be made, so the cycle was never tested");
    if !made {
        return;
    }
    let got = checker::run(&dir);
    assert!(
        got.iter().any(|d| d.code == "E-NO-BECAUSE"),
        "the walker did not survive a directory cycle: {:?}",
        got.iter().map(|d| d.code).collect::<Vec<&str>>()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn every_public_value_shape_is_governed_by_a_reason() {
    let dir = fixture("allfacts");
    let src = std::fs::read_to_string(dir.join("spec").join("src").join("lib.rs")).unwrap();
    let parsed = syn::parse_file(&src).unwrap();
    let facts = checker::facts::public_in(&parsed, "");
    assert!(
        facts.len() >= 4,
        "the fixture lost a value shape: {:?}",
        facts
    );
    let cried: Vec<String> = checker::run(&dir)
        .into_iter()
        .filter(|d| d.code == "E-NO-BECAUSE")
        .map(|d| d.note.clone())
        .collect();
    for fact in &facts {
        assert!(
            cried.iter().any(|c| {
                let subject = c.split(char::is_whitespace).next().unwrap_or_default();
                fact == subject || fact.starts_with(&format!("{}_", subject))
            }),
            "{} is a public value shape but no reason is demanded for it; the two lists have drifted: {:?}",
            fact,
            cried
        );
    }
}

#[test]
fn a_private_pattern_constant_still_needs_its_source() {
    let dir = std::env::temp_dir().join(format!("premise_private_const_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("patterns").join("src");
    std::fs::create_dir_all(&src).unwrap();
    let _ = std::fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"patterns\"]\n");
    let _ = std::fs::write(dir.join("premise.zones"), "patterns = patterns\n");
    let _ = std::fs::write(
        dir.join("patterns").join("Cargo.toml"),
        "[package]\nname = \"patterns\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    let _ = std::fs::write(
        src.join("lib.rs"),
        "use patterns_macros::because;\n\nconst GAMMA: u64 = 7;\nbecause!(GAMMA, \"a private algorithm constant with no citation at all\");\n",
    );
    let got: Vec<String> = checker::run(&dir)
        .into_iter()
        .map(|d| d.code.to_string())
        .collect();
    assert!(
        got.iter().any(|c| c == "E-UNTRACEABLE"),
        "a private pattern constant escaped the citation rule; the library's own PRNG constants are all private: {:?}",
        got
    );
    let _ = std::fs::remove_dir_all(&dir);
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{}_{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("app").join("src")).unwrap();
    dir
}

#[test]
fn a_deeply_nested_expression_is_answered_rather_than_overflowing() {
    let dir = scratch("premise_deep_probe");
    let depth = 2000;
    let body = format!(
        "pub fn deep() -> u32 {{ {}1{} }}\n",
        "(".repeat(depth),
        ")".repeat(depth)
    );
    std::fs::write(dir.join("app").join("src").join("lib.rs"), body).unwrap();
    let got = checker::run(&dir);
    assert!(
        got.iter().all(|d| d.code != "E-PARSE"),
        "a nesting rustc accepts was reported as unparseable: {:?}",
        got
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_file_that_is_not_utf8_is_reported_not_skipped() {
    let dir = scratch("premise_bytes_probe");
    let mut bytes = b"pub fn bad() -> u32 { 1 }\n".to_vec();
    bytes.extend_from_slice(&[0xff, 0xfe, b'\n']);
    std::fs::write(dir.join("app").join("src").join("lib.rs"), bytes).unwrap();
    let got = checker::run(&dir);
    assert!(
        got.iter().any(|d| d.code == "E-PARSE"),
        "an unreadable file was counted as checked: {:?}",
        got
    );
    let _ = std::fs::remove_dir_all(&dir);
}

fn notes(name: &str, code: &str) -> Vec<String> {
    checker::run(&fixture(name))
        .into_iter()
        .filter(|d| d.code == code)
        .map(|d| d.note)
        .collect()
}

#[test]
fn a_diagnostic_names_the_macro_that_was_written() {
    let dir = scratch("premise_macro_named");
    std::fs::create_dir_all(dir.join("spec").join("src")).unwrap();
    std::fs::write(
        dir.join("spec").join("src").join("lib.rs"),
        "use patterns::provisional;\n\npub const DOCK_BAYS: usize = 12;\nprovisional!(DOCK_BAYS, \"12 bays\");\n",
    )
    .unwrap();
    let got = checker::run(&dir);
    for code in ["E-WEAK-REASON", "E-FACT-IN-REASON"] {
        let said: Vec<&String> = got.iter().filter(|d| d.code == code).map(|d| &d.note).collect();
        assert!(
            said.iter().all(|n| n.starts_with("provisional!(")),
            "{} blamed the wrong macro: {:?}",
            code,
            said
        );
        assert!(!said.is_empty(), "{} did not fire", code);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_match_on_a_call_says_to_bind_it() {
    let dir = scratch("premise_match_call");
    std::fs::create_dir_all(dir.join("spec").join("src")).unwrap();
    std::fs::write(
        dir.join("spec").join("src").join("lib.rs"),
        "use patterns::{found, Context};\npub struct S;\nimpl Context for S {\n    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {\n        match found(&state.rides, 1) {\n            Some(one) => Ok(Event::Seen),\n            None => Err(Fault::Gone),\n        }\n    }\n}\n",
    )
    .unwrap();
    let got: Vec<String> = checker::run(&dir)
        .into_iter()
        .filter(|d| d.code == "E-LOGIC-OUTSIDE-PATTERN")
        .map(|d| d.note)
        .collect();
    assert!(
        got.iter().any(|n| n.contains("bind the call with let")),
        "the match diagnostic did not say what to do: {:?}",
        got
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_near_pattern_message_counts_its_edits_in_english() {
    let said = notes("nearpattern", "E-NEAR-PATTERN");
    assert!(!said.is_empty(), "the nearpattern fixture went silent");
    for n in &said {
        assert!(
            !n.contains("1 edits"),
            "one edit is not plural: {}",
            n
        );
    }
}

#[test]
fn every_signature_visitor_in_the_scanner_carries_the_same_checks() {
    let source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src").join("scan.rs"),
    )
    .unwrap();
    let visitors = ["fn visit_item_fn(", "fn visit_impl_item_fn(", "fn visit_trait_item_fn("];
    for head in visitors {
        let body = source
            .split(head)
            .nth(1)
            .and_then(|rest| rest.split("\n    }\n").next())
            .unwrap_or_default();
        assert!(!body.is_empty(), "the scanner no longer overrides {}", head);
        assert!(
            body.contains("asyncness"),
            "{} does not carry the signature checks its siblings carry",
            head
        );
    }
}

const MUST_BUILD: &[&str] = &["lazyguard", "decidedtwice", "implcited", "provcite", "decidedstate", "markerconsts"];

fn library_path() -> String {
    let real = std::fs::canonicalize(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("patterns"),
    )
    .unwrap();
    let shown = real.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/");
    shown.trim_start_matches("//?/").to_string()
}

fn copy_all(from: &std::path::Path, to: &std::path::Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let p = entry.path();
        if p.is_dir() {
            copy_all(&p, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&p, to.join(entry.file_name())).unwrap();
        }
    }
}

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

#[test]
fn every_silent_fixture_that_writes_a_reason_macro_builds() {
    let patterns = library_path();
    let dir = std::env::temp_dir().join(format!("premise_build_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut members: Vec<String> = Vec::new();
    for name in MUST_BUILD {
        assert!(
            !fixture(name).join("app").is_dir(),
            "{} carries an app, so it needs a crate named spec and cannot share one workspace; build it on its own",
            name
        );
        copy_all(&fixture(name).join("spec"), &dir.join(name));
        std::fs::write(
            dir.join(name).join("Cargo.toml"),
            format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\npremise = {{ path = \"{}\" }}\n",
                name, patterns
            ),
        )
        .unwrap();
        members.push(format!("\"{}\"", name));
    }
    std::fs::write(
        dir.join("Cargo.toml"),
        format!("[workspace]\nresolver = \"2\"\nmembers = [{}]\n", members.join(", ")),
    )
    .unwrap();
    let out = std::process::Command::new(env!("CARGO"))
        .arg("build")
        .arg("--manifest-path")
        .arg(dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", nested_target("rules"))
        .output()
        .expect("cargo runs");
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(out.status.success(), "a silent fixture does not build:\n{}", err);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Source {
    Spec,
    Patterns,
    Std,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Form {
    Explicit,
    Alias,
    Glob,
}

type Import = Option<(Source, Form)>;

const CHOICES: &[Import] = &[
    None,
    Some((Source::Spec, Form::Explicit)),
    Some((Source::Spec, Form::Alias)),
    Some((Source::Spec, Form::Glob)),
    Some((Source::Patterns, Form::Explicit)),
    Some((Source::Patterns, Form::Alias)),
    Some((Source::Patterns, Form::Glob)),
    Some((Source::Std, Form::Explicit)),
    Some((Source::Std, Form::Alias)),
    Some((Source::Std, Form::Glob)),
];

fn module_of(source: Source) -> &'static str {
    match source {
        Source::Spec => "spec",
        Source::Patterns => "patterns",
        Source::Std => "std::time",
    }
}

fn type_of(source: Source) -> &'static str {
    match source {
        Source::Spec => "Kind",
        Source::Patterns => "Seed",
        Source::Std => "Duration",
    }
}

fn import_text(import: Import, alias: &str) -> String {
    match import {
        None => String::new(),
        Some((source, Form::Explicit)) => format!("use {}::{};\n", module_of(source), type_of(source)),
        Some((source, Form::Alias)) => format!("use {} as {};\n", module_of(source), alias),
        Some((source, Form::Glob)) => format!("use {}::*;\n", module_of(source)),
    }
}

fn access_text(target: Source, through: Import, alias: &str) -> String {
    match through {
        Some((_, Form::Alias)) => format!("{}::{}::MAX", alias, type_of(target)),
        _ => format!("{}::MAX", type_of(target)),
    }
}

fn oracle(imports: &[Import], target: Source) -> bool {
    if target == Source::Std {
        return true;
    }
    let named = imports
        .iter()
        .flatten()
        .any(|(s, f)| *s == target && matches!(f, Form::Explicit | Form::Alias));
    if named {
        return false;
    }
    let std_glob = imports
        .iter()
        .flatten()
        .any(|(s, f)| *s == Source::Std && *f == Form::Glob);
    if std_glob {
        return true;
    }
    let own_glob = imports.iter().flatten().any(|(s, f)| *s == target && *f == Form::Glob);
    match own_glob {
        true => false,
        false => target == Source::Patterns,
    }
}

const STAND_IN_LIBRARY: &str = "pub struct Seed(pub u64);
";

#[test]
fn every_pair_of_import_forms_is_judged_as_the_glob_oracle_judges_it() {
    let mut wrong = Vec::new();
    let mut counted = 0usize;
    for (ia, a) in CHOICES.iter().enumerate() {
        for (ib, b) in CHOICES.iter().enumerate() {
            let (target, through) = match (a, b) {
                (Some((s, _)), _) => (*s, *a),
                (None, Some((s, _))) => (*s, *b),
                (None, None) => continue,
            };
            let first = import_text(*a, "ma");
            let second = import_text(*b, "mb");
            let alias = match through == *a {
                true => "ma",
                false => "mb",
            };
            let body = format!(
                "{}{}\npub fn widest() -> u32 {{\n    {}\n}}\n",
                first,
                second,
                access_text(target, through, alias)
            );
            let dir = std::env::temp_dir().join(format!("premise_pairs_{}_{}_{}", ia, ib, std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(dir.join("app").join("src")).unwrap();
            std::fs::create_dir_all(dir.join("spec").join("src")).unwrap();
            std::fs::write(dir.join("spec").join("src").join("lib.rs"), "pub struct Kind;\n").unwrap();
            std::fs::create_dir_all(dir.join("patterns").join("src")).unwrap();
            std::fs::write(dir.join("patterns").join("src").join("lib.rs"), STAND_IN_LIBRARY).unwrap();
            std::fs::write(dir.join("app").join("src").join("lib.rs"), &body).unwrap();
            let found = checker::run(&dir);
            let judged = found
                .iter()
                .any(|d| d.code == "E-INLINE-LITERAL" && d.file.ends_with("app/src/lib.rs"));
            let _ = std::fs::remove_dir_all(&dir);
            counted += 1;
            if judged != oracle(&[*a, *b], target) {
                let said: Vec<String> = found.iter().map(|d| d.note.clone()).collect();
                wrong.push(format!(
                    "{:?} then {:?} reaching {:?}: checker says {}, oracle says {}; {:?}",
                    a,
                    b,
                    target,
                    judged,
                    oracle(&[*a, *b], target),
                    said
                ));
            }
        }
    }
    assert!(counted > 0, "the generator produced no cases");
    assert!(
        wrong.is_empty(),
        "import pairs judged differently from the glob oracle (the oracle draws its std cases from one module, since the checker treats every glob from outside spec and patterns alike; widen the oracle before widening the std set):\n{}",
        wrong.join("\n")
    );
}

#[test]
fn every_test_program_of_this_crate_is_one_the_gate_would_run() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let programs = checker::cargo::test_programs(&root, "premise_checker");
    let names: Vec<String> = programs
        .iter()
        .filter_map(|p| p.file_stem())
        .map(|s| s.to_string_lossy().to_string())
        .collect();
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"))
        .expect("the tests directory is readable")
        .flatten()
    {
        let path = entry.path();
        if path.extension().map(|e| e != "rs").unwrap_or(true) {
            continue;
        }
        let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        if !names.iter().any(|n| n.starts_with(&format!("{}-", stem)) || *n == stem) {
            missing.push(stem);
        }
    }
    missing.sort();
    assert!(
        missing.is_empty(),
        "the gate runs the programs cargo lists, and cargo did not list {:?}; a step that runs a short list passes a tree it never checked. Listed: {:?}",
        missing,
        names
    );
}

#[test]
fn a_readme_that_names_a_file_is_a_pointer_and_one_that_names_nothing_is_not() {
    let dir = std::env::temp_dir().join(format!("premise_readme_probe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("spec").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"spec\"]\n").unwrap();
    std::fs::write(dir.join("premise.zones"), "spec = spec\n").unwrap();
    std::fs::write(src.join("lib.rs"), "pub const A: u32 = 1;\n").unwrap();
    std::fs::write(dir.join("spec").join("MANUAL.md"), "the prose, stated once\n").unwrap();
    let manifest = |readme: &str| {
        format!(
            "[package]\nname = \"spec\"\nversion = \"0.1.0\"\nedition = \"2021\"\nreadme = \"{}\"\n",
            readme
        )
    };
    let flagged = |at: &std::path::Path| {
        checker::run(at)
            .iter()
            .any(|d| d.code == "E-COMMENT" && d.file.contains("readme"))
    };

    std::fs::write(dir.join("spec").join("Cargo.toml"), manifest("MANUAL.md")).unwrap();
    assert!(
        !flagged(&dir),
        "a readme naming a file that exists points at prose the tree already holds, and is not prose hidden in a manifest"
    );

    std::fs::write(dir.join("spec").join("Cargo.toml"), manifest("GONE.md")).unwrap();
    assert!(
        flagged(&dir),
        "a readme naming a file that does not exist is a fact that has drifted, and nothing else in the tree would catch it"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_library_may_be_named_by_its_crate_rather_than_by_a_path() {
    let dir = std::env::temp_dir().join(format!("premise_library_probe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let away = dir.join("away").join("shed");
    std::fs::create_dir_all(away.join("src")).unwrap();
    std::fs::write(
        away.join("Cargo.toml"),
        "[package]\nname = \"shed\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    std::fs::write(away.join("src").join("lib.rs"), "pub fn stowed<T>(items: &[T]) -> usize { items.len() }\n").unwrap();

    let here = dir.join("here");
    std::fs::create_dir_all(here.join("app").join("src")).unwrap();
    std::fs::write(
        here.join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\nmembers = [\"app\"]\n",
    )
    .unwrap();
    std::fs::write(
        here.join("app").join("Cargo.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nshed = { path = \"../../away/shed\" }\n",
    )
    .unwrap();
    std::fs::write(here.join("app").join("src").join("lib.rs"), "pub fn counted(items: &[u32]) -> usize { shed::stowed(items) }\n").unwrap();

    let named = |line: &str| {
        std::fs::write(here.join("premise.zones"), format!("{}\napp = app\n", line)).unwrap();
        crate::forget_metadata();
        checker::run(&here)
            .iter()
            .any(|d| d.code == "E-LOGIC-OUTSIDE-PATTERN" || d.code == "E-OUTSIDE-WORKSPACE")
    };

    assert!(
        !named("library = ../away/shed"),
        "a library named by a path is still read for the names it exports"
    );
    assert!(
        !named("library = shed"),
        "a library named by its crate is found through cargo, which is what lets one come from the registry"
    );
    assert!(
        named("library = nothing_of_that_name"),
        "a library that is neither a path nor a crate cargo knows is not silently taken as absent"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

fn forget_metadata() {
    checker::cargo::forget();
}

#[test]
fn a_reason_on_a_thread_local_names_something_that_is_declared() {
    let dir = std::env::temp_dir().join(format!("premise_thread_local_probe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("spec").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"spec\"]\n").unwrap();
    std::fs::write(dir.join("premise.zones"), "spec = spec\n").unwrap();
    std::fs::write(
        dir.join("spec").join("Cargo.toml"),
        "[package]\nname = \"spec\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    let held = concat!(
        "use std::cell::RefCell;\n",
        "thread_local! {\n",
        "    static SEEN: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };\n",
        "}\n",
        "because!(SEEN, \"the names this thread has already reported, kept per thread because two reports read different trees\");\n",
    );
    std::fs::write(src.join("lib.rs"), held).unwrap();
    let orphaned = checker::run(&dir)
        .iter()
        .any(|d| d.code == "E-ORPHAN-REASON");
    assert!(
        !orphaned,
        "thread_local! declares the statics inside it, so a reason on one is not a reason for nothing"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_reason_may_not_spell_the_value_it_explains() {
    let dir = std::env::temp_dir().join(format!("premise_spelled_probe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("spec").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"spec\"]\n").unwrap();
    std::fs::write(dir.join("premise.zones"), "spec = spec\n").unwrap();
    std::fs::write(
        dir.join("spec").join("Cargo.toml"),
        "[package]\nname = \"spec\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    let stated = |body: &str| {
        std::fs::write(src.join("lib.rs"), body).unwrap();
        checker::run(&dir)
            .iter()
            .any(|d| d.code == "E-FACT-IN-REASON")
    };

    assert!(
        stated("pub const OPEN_LIMIT: u32 = 3;\nbecause!(OPEN_LIMIT, \"three openings is where the test showed hinge fatigue\");\n"),
        "a reason that spells its own value states the constant twice, and the second copy is where nothing checks it"
    );
    assert!(
        !stated("pub const OPEN_LIMIT: u32 = 3;\nbecause!(OPEN_LIMIT, \"the opening count at which the test showed hinge fatigue\");\n"),
        "a reason that says what the value counts, without saying the value, is the whole point"
    );
    assert!(
        !stated("pub const OPEN_LIMIT: u32 = 3;\nbecause!(OPEN_LIMIT, \"the count the test settled on, of the two the report offered\");\n"),
        "a number that is not this item's value is ordinary English and must not be refused"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_body_whose_only_calls_are_methods_is_still_compared() {
    let dir = std::env::temp_dir().join(format!("premise_method_probe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("patterns").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[workspace]\nmembers = [\"patterns\"]\n").unwrap();
    std::fs::write(dir.join("premise.zones"), "patterns = patterns\n").unwrap();
    std::fs::write(
        dir.join("patterns").join("Cargo.toml"),
        "[package]\nname = \"patterns\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    let held = concat!(
        "pub fn opened(text: &str) -> String {\n",
        "    let mut out = String::new();\n",
        "    for part in text.split(' ') {\n",
        "        out.push_str(part.trim_start());\n",
        "    }\n",
        "    out\n",
        "}\n",
        "because!(opened, \"every word of a line with its leading space taken off, so a column reads flush\");\n",
        "pub fn closed(text: &str) -> String {\n",
        "    let mut out = String::new();\n",
        "    for part in text.split(' ') {\n",
        "        out.push_str(part.trim_end());\n",
        "    }\n",
        "    out\n",
        "}\n",
        "because!(closed, \"every word of a line with its trailing space taken off, so a column reads flush\");\n",
    );
    std::fs::write(src.join("lib.rs"), held).unwrap();
    let flagged = checker::run(&dir)
        .iter()
        .any(|d| d.code == "E-NEAR-PATTERN");
    assert!(
        flagged,
        "a pair differing only by a method name is a copy and a rename, and was invisible while only a free call marked a body as composed"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
