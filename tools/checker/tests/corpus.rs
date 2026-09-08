use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
}

fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            files(&p, out);
        }
        if p.extension().map(|e| e == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
}

fn reasons(text: &str) -> Vec<(String, String)> {
    let parsed = match syn::parse_file(text) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for item in &parsed.items {
        let mac = match item {
            syn::Item::Macro(m) => m,
            _ => continue,
        };
        let tokens = mac.mac.tokens.to_string();
        let (item_name, rest) = match tokens.split_once(',') {
            Some(v) => v,
            None => continue,
        };
        let text = rest.trim().trim_matches('"').to_string();
        out.push((item_name.trim().to_string(), text));
    }
    out
}

#[test]
fn no_reason_written_before_the_rule_is_flagged() {
    let mut found = Vec::new();
    files(&root(), &mut found);
    assert!(found.len() > 1, "the corpus is missing");
    let mut seen = 0usize;
    let mut cried = Vec::new();
    for path in &found {
        let text = std::fs::read_to_string(path).unwrap();
        for (item, reason) in reasons(&text) {
            seen += 1;
            if let Some(why) = checker::because::weak(&item, &reason) {
                cried.push(format!("{} {:?}: {}", item, reason, why));
            }
        }
    }
    assert!(seen >= 40, "expected the full corpus, read {} reasons", seen);
    assert!(
        cried.is_empty(),
        "the reason checks fired on reasons written before the rules existed: {:#?}",
        cried
    );
}

#[test]
fn the_corpus_test_would_notice_a_bad_reason() {
    assert!(
        checker::because::weak("MAX_HP", "max hp").is_some(),
        "a reason that restates its own name passed"
    );
    assert!(
        checker::because::weak("MAX_HP", "").is_some(),
        "an empty reason passed"
    );
}
