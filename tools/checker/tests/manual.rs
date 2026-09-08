use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
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

fn codes(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes: Vec<char> = text.chars().collect();
    let mut at = 0usize;
    while at + 2 < bytes.len() {
        if bytes[at] == 'E' && bytes[at + 1] == '-' {
            let mut end = at + 2;
            while end < bytes.len() && (bytes[end].is_ascii_uppercase() || bytes[end] == '-') {
                end += 1;
            }
            let word: String = bytes[at..end].iter().collect();
            if word.len() > 2 && !word.ends_with('-') {
                out.insert(word);
            }
            at = end;
            continue;
        }
        at += 1;
    }
    out
}

fn read(parts: &[&str]) -> String {
    let mut p = root();
    for part in parts {
        p = p.join(part);
    }
    std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("missing {}", p.display()))
}

#[test]
fn the_manual_documents_every_check_the_tools_emit() {
    let implemented: BTreeSet<String> = codes(&read(&["tools", "checker", "src", "lib.rs"]));
    let manual = manual();
    let table: String = manual
        .lines()
        .filter(|l| l.starts_with("| `E-"))
        .collect::<Vec<&str>>()
        .join("
");
    let documented = codes(&table);

    let undocumented: Vec<&String> = implemented.difference(&documented).collect();
    assert!(
        undocumented.is_empty(),
        "checks emitted but not in the manual: {:?}",
        undocumented
    );

    let invented: Vec<&String> = documented.difference(&implemented).collect();
    assert!(
        invented.is_empty(),
        "checks in the manual that no tool emits: {:?}",
        invented
    );
}

const COUNTS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    "twenty", "thirty", "forty",
];

#[test]
fn the_manual_never_states_how_many_checks_there_are() {
    let text = manual().to_lowercase();
    let mut stated = Vec::new();
    for word in COUNTS {
        stated.push(format!("{} checks", word));
    }
    for digits in 0..100usize {
        stated.push(format!("{} checks", digits));
    }
    let found: Vec<&String> = stated.iter().filter(|p| text.contains(p.as_str())).collect();
    assert!(
        found.is_empty(),
        "the table is the only statement of the check set, and prose counting it goes stale: {:?}",
        found
    );
}

#[test]
fn every_check_can_explain_itself() {
    let implemented: BTreeSet<String> = codes(&read(&["tools", "checker", "src", "lib.rs"]));
    for code in &implemented {
        let said = checker::explain(code);
        assert!(
            !said.starts_with("no check named"),
            "checker --explain {} answers nothing",
            code
        );
        assert!(
            said.len() > 30,
            "checker --explain {} answers too little to act on: {:?}",
            code,
            said
        );
    }
}

#[test]
fn the_heading_counts_the_laws_the_section_states() {
    let manual = manual();
    let stated = manual
        .lines()
        .filter(|l| l.starts_with("## ") && l.contains("laws"))
        .count();
    assert_eq!(stated, 1, "expected exactly one laws heading");
    let heading = manual
        .lines()
        .find(|l| l.starts_with("## ") && l.contains("laws"))
        .unwrap_or_default()
        .to_string();
    let word = heading
        .trim_start_matches("## The ")
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_lowercase();
    let laws = section(&manual, &heading)
        .lines()
        .filter(|l| l.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) && l.contains(". **"))
        .count();
    let spelled = SPELLED.iter().find(|(w, _)| *w == word).map(|(_, n)| *n);
    assert_eq!(
        spelled,
        Some(laws),
        "the heading and the section disagree about how many laws there are: heading says {:?}, the section states {}",
        word,
        laws
    );
}

const RETIRED: &[&str] = &["generator", "`gen`", "TypeScript", "crossing"];

#[test]
fn the_manual_names_no_component_that_was_removed() {
    let kept: String = manual()
        .lines()
        .filter(|l| !l.contains("was removed") && !l.contains("existed and was"))
        .collect::<Vec<&str>>()
        .join("\n");
    let back: Vec<&&str> = RETIRED.iter().filter(|w| kept.contains(**w)).collect();
    assert!(
        back.is_empty(),
        "the manual names a component the tree no longer builds: {:?}",
        back
    );
}

fn spec_source() -> String {
    let mut out = String::new();
    let mut stack = vec![root().join("spec").join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                out.push_str(&std::fs::read_to_string(&p).unwrap_or_default());
                out.push('\n');
            }
        }
    }
    out
}

#[test]
fn every_spec_path_the_readme_names_exists() {
    let readme = read(&["README.md"]);
    let spec = spec_source();
    let mut missing = Vec::new();
    for piece in readme.split('`').skip(1).step_by(2) {
        if !piece.starts_with("spec::") {
            continue;
        }
        let item = piece.rsplit("::").next().unwrap_or_default();
        let declared = ["const ", "struct ", "enum ", "trait ", "mod "]
            .iter()
            .any(|kind| spec.contains(&format!("{}{}", kind, item)));
        if !declared {
            missing.push(piece.to_string());
        }
    }
    assert!(
        missing.is_empty(),
        "the README names spec items that spec does not declare: {:?}",
        missing
    );
}

#[test]
fn every_crate_the_prose_runs_is_a_workspace_member() {
    let members: Vec<String> = checker::cargo::read(&root())
        .map(|m| m.packages.iter().map(|p| p.name.clone()).collect())
        .unwrap_or_default();
    let text = manual();
    let mut unknown = Vec::new();
    for line in text.lines() {
        let mut words = line.split_whitespace().peekable();
        while let Some(w) = words.next() {
            if w != "-p" {
                continue;
            }
            let name = words
                .peek()
                .map(|n| n.trim_matches(|c: char| !c.is_alphanumeric() && c != '_'))
                .unwrap_or_default();
            if !name.is_empty() && !members.iter().any(|m| m == name) {
                unknown.push(name.to_string());
            }
        }
    }
    assert!(
        unknown.is_empty(),
        "the manual runs crates the workspace does not build: {:?}",
        unknown
    );
}

fn unix(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn worked_example() -> Vec<(String, String)> {
    let manual = unix(&manual());
    let start = manual
        .find("## One context, end to end")
        .expect("the manual has the worked example section");
    let body = &manual[start..];
    let end = body[1..].find("\n## ").map(|i| i + 1).unwrap_or(body.len());
    let section = &body[..end];
    let mut out = Vec::new();
    for piece in section.split("```rust\n").skip(1) {
        let block = piece.split("```").next().unwrap_or_default();
        let mut lines = block.lines();
        let first = lines.next().unwrap_or_default();
        if !first.starts_with("// ") {
            continue;
        }
        let path = first.trim_start_matches("// ").trim().to_string();
        let rest: Vec<&str> = lines.collect();
        let mut text = rest.join("\n");
        text.push('\n');
        out.push((path, text));
    }
    out
}

fn door() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("door")
}

#[test]
fn the_worked_example_is_the_door_fixture() {
    let blocks = worked_example();
    assert_eq!(blocks.len(), 8, "the worked example should show eight files");
    for (path, text) in &blocks {
        let mut file = door();
        for part in path.split('/') {
            file = file.join(part);
        }
        let held = std::fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("the fixture has no file for {}", path));
        assert_eq!(
            unix(&held),
            *text,
            "the manual and the door fixture disagree about {}",
            path
        );
    }
}

fn copy_tree(from: &std::path::Path, to: &std::path::Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            std::fs::copy(&src, &dst).unwrap();
        }
    }
}

fn patterns_path() -> String {
    let raw = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("patterns")
        .canonicalize()
        .expect("the patterns crate exists");
    raw.to_string_lossy()
        .trim_start_matches(r"\\?\")
        .replace('\\', "/")
}

#[test]
fn the_worked_example_compiles() {
    let dir = std::env::temp_dir().join(format!("premise_door_compiles_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    copy_tree(&door().join("spec"), &dir.join("spec"));
    copy_tree(&door().join("app"), &dir.join("app"));
    let patterns = patterns_path();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\nmembers = [\"spec\", \"app\"]\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("spec").join("Cargo.toml"),
        format!(
            "[package]\nname = \"spec\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\npremise = {{ path = \"{}\" }}\n",
            patterns
        ),
    )
    .unwrap();
    std::fs::write(
        dir.join("app").join("Cargo.toml"),
        format!(
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\npremise = {{ path = \"{}\" }}\nspec = {{ path = \"../spec\" }}\n",
            patterns
        ),
    )
    .unwrap();
    let out = std::process::Command::new(env!("CARGO"))
        .arg("build")
        .arg("--manifest-path")
        .arg(dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", nested_target("manual"))
        .output()
        .expect("cargo runs");
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        out.status.success(),
        "the manual's worked example does not compile:\n{}",
        err
    );
}

fn manual() -> String {
    read(&["README.md"])
        .lines()
        .map(plain_heading)
        .collect::<Vec<String>>()
        .join("
")
}

fn plain_heading(line: &str) -> String {
    let hashes: String = line.chars().take_while(|c| *c == '#').collect();
    let rest = &line[hashes.len()..];
    match hashes.is_empty() || !rest.starts_with(' ') {
        true => line.to_string(),
        false => {
            let title = rest.trim_start_matches(|c: char| !c.is_ascii_alphanumeric() && c != '`');
            format!("{} {}", hashes, title)
        }
    }
}

fn section(text: &str, heading: &str) -> String {
    text.split(heading)
        .nth(1)
        .unwrap_or_default()
        .split("
## ")
        .next()
        .unwrap_or_default()
        .to_string()
}

fn table_rows(text: &str, first_cell: &str) -> Vec<String> {
    text.lines()
        .filter(|l| l.starts_with("| `") && l.contains(first_cell))
        .map(|l| l.trim_start_matches("| `").split('`').next().unwrap_or_default().to_string())
        .collect()
}

#[test]
fn the_zone_table_lists_exactly_the_zones_the_checker_has() {
    let listed: BTreeSet<String> = table_rows(&section(&manual(), "## Zones"), "/` |")
        .into_iter()
        .filter(|r| !r.contains("::") && !r.contains("<"))
        .map(|r| r.trim_end_matches('/').to_string())
        .collect();
    let source = read(&["tools", "checker", "src", "zone.rs"]);
    let body = source
        .split("pub enum Zone {")
        .nth(1)
        .and_then(|rest| rest.split('}').next())
        .unwrap_or_default();
    let coded: BTreeSet<String> = body
        .split(',')
        .map(|v| v.trim().to_lowercase())
        .filter(|v| !v.is_empty())
        .collect();
    assert_eq!(
        listed, coded,
        "the manual's zone table and the checker's Zone enum disagree"
    );
}

#[test]
fn every_marker_the_manual_names_is_declared_in_spec() {
    let spec = spec_source();
    let mut missing = Vec::new();
    for piece in manual().split('`').skip(1).step_by(2) {
        for word in piece.split(|c: char| !c.is_alphanumeric()) {
            let marker = word.starts_with("By")
                && word.len() > 2
                && word.chars().nth(2).map(|c| c.is_uppercase()).unwrap_or(false);
            if marker && !spec.contains(&format!("struct {};", word)) {
                missing.push(word.to_string());
            }
        }
    }
    missing.sort();
    missing.dedup();
    assert!(
        missing.is_empty(),
        "the manual names markers spec does not declare: {:?}",
        missing
    );
}

const SPELLED: &[(&str, usize)] = &[("two", 2), ("four", 4), ("twelve", 12), ("five", 5)];

fn quoted_threshold(text: &str, name: &str) -> Option<usize> {
    let after = text.split(&format!("`{}`, ", name)).nth(1)?;
    let word = after.split_whitespace().next()?;
    SPELLED.iter().find(|(w, _)| *w == word).map(|(_, n)| *n)
}

fn coded_constant(source: &str, name: &str) -> Option<usize> {
    let after = source.split(&format!("const {}: usize = ", name)).nth(1)?;
    after.split(';').next()?.trim().parse().ok()
}

#[test]
fn the_thresholds_the_manual_quotes_are_the_checkers() {
    let text = manual();
    let source = read(&["tools", "checker", "src", "lib.rs"]);
    for name in ["MAX_DRIFT", "MIN_SHAPE_TOKENS"] {
        let said = quoted_threshold(&text, name);
        let coded = coded_constant(&source, name);
        assert!(said.is_some(), "the manual no longer quotes {} in words", name);
        assert_eq!(said, coded, "the manual quotes {} wrongly", name);
    }
}

#[test]
fn the_manual_counts_the_gate_steps_the_gate_runs() {
    let text = manual();
    let gate = read(&["tools", "gate", "src", "main.rs"]);
    let steps = gate
        .split("const STEPS")
        .nth(1)
        .map(|rest| rest.split("];").next().unwrap_or_default().matches("\n    (").count())
        .unwrap_or(0);
    let said = text
        .lines()
        .find(|l| l.contains(" steps: the compiler"))
        .and_then(|l| l.split_whitespace().next())
        .map(|w| w.to_lowercase())
        .unwrap_or_default();
    let spelled = SPELLED.iter().find(|(w, _)| *w == said).map(|(_, n)| *n);
    assert_eq!(spelled, Some(steps), "the manual counts the gate steps wrongly: {:?}", said);
}

#[test]
fn the_manual_names_every_census_field() {
    let text = manual();
    let main = read(&["tools", "checker", "src", "lib.rs"]);
    let line = main
        .lines()
        .find(|l| l.contains("pattern functions, {} governed lines"))
        .unwrap_or_default();
    let fields: Vec<&str> = line
        .trim()
        .trim_matches('"')
        .trim_end_matches("\",")
        .split(", ")
        .map(|f| f.trim_start_matches("{} ").trim_end_matches('"'))
        .collect();
    assert!(fields.len() >= 5, "the census line lost a field: {:?}", fields);
    let census = text
        .split("## The census")
        .nth(1)
        .unwrap_or_default();
    for f in &fields {
        let word = f.split_whitespace().next().unwrap_or_default();
        assert!(
            census.contains(word),
            "the census section does not name the {} field",
            f
        );
    }
}

#[test]
fn the_near_pattern_sample_matches_the_real_message() {
    let text = manual();
    let sample = text
        .lines()
        .find(|l| l.starts_with("E-NEAR-PATTERN "))
        .unwrap_or_default();
    let source = read(&["tools", "checker", "src", "lib.rs"]);
    assert!(
        source.contains("\"{} is {} from {} at {}:{} ({}). Parameterise the difference\""),
        "the E-NEAR-PATTERN format string moved; update this test and the manual together"
    );
    assert!(
        sample.contains(" is 1 edit from ") && sample.ends_with("). Parameterise the difference"),
        "the manual's E-NEAR-PATTERN sample does not match the checker's format: {}",
        sample
    );
}

#[test]
fn every_context_the_manual_names_in_its_table_exists() {
    let table = section(&manual(), "## When the door is too small");
    let named: Vec<String> = table_rows(&table, "/` |")
        .into_iter()
        .map(|r| r.trim_end_matches('/').to_string())
        .collect();
    let live: Vec<String> = std::fs::read_dir(root().join("spec").join("src").join("contexts"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    let zones = ["patterns", "spec", "app", "tools", "tests"];
    let ghosts: Vec<&String> = named
        .iter()
        .filter(|n| !live.contains(n) && !zones.contains(&n.as_str()))
        .collect();
    assert!(ghosts.is_empty(), "the manual's context table names contexts not in the tree: {:?}", ghosts);
}

#[test]
fn setup_is_stated_once_in_the_manual() {
    let phrase = "Copy `patterns/`";
    assert_eq!(
        manual().matches(phrase).count(),
        1,
        "{:?} must appear exactly once in the manual",
        phrase
    );
}

#[test]
fn the_tools_name_no_component_that_was_removed() {
    let mut hits = Vec::new();
    let mut stack = vec![root().join("tools")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if p.is_dir() {
                if name != "target" && name != "fixtures" && name != "corpus" {
                    stack.push(p);
                }
                continue;
            }
            let in_src = p.components().any(|c| c.as_os_str() == "src");
            if in_src && p.extension().map(|x| x == "rs").unwrap_or(false) {
                let text = std::fs::read_to_string(&p).unwrap_or_default();
                if text.contains("generator") {
                    hits.push(p.display().to_string());
                }
            }
        }
    }
    assert!(hits.is_empty(), "the tools still name the generator: {:?}", hits);
}

#[test]
fn every_reason_the_manual_shows_as_right_obeys_the_reason_checks() {
    let text = manual();
    let wrong_way = section(&text, "## A reason is not a place to put facts");
    let mut broken = Vec::new();
    for block in text.split("```rust").skip(1) {
        let code = block.split("```").next().unwrap_or_default();
        for line in code.lines() {
            let trimmed = line.trim();
            let shown = trimmed.starts_with("because!(") || trimmed.starts_with("provisional!(") || trimmed.starts_with("decided!(");
            if !shown || wrong_way.contains(trimmed) {
                continue;
            }
            let said = trimmed.split('"').nth(1).unwrap_or_default();
            let item = trimmed
                .trim_start_matches("because!(")
                .trim_start_matches("provisional!(")
                .trim_start_matches("decided!(")
                .split(',')
                .next()
                .unwrap_or_default()
                .trim();
            let cites = trimmed.matches(',').count() > 1;
            if let Some(fact) = checker::because::states_a_fact(said) {
                broken.push(format!("{} writes {} into prose", trimmed, fact));
            }
            if let Some(why) = checker::because::weak(item, said) {
                broken.push(format!("{} is weak: {}", trimmed, why));
            }
            if !cites {
                if let Some(word) = checker::because::leans_on(said) {
                    broken.push(format!("{} names a {} and cites nothing", trimmed, word));
                }
            }
        }
    }
    assert!(broken.is_empty(), "the manual shows reasons its own checks refuse:\n{}", broken.join("\n"));
}

fn pattern_sources() -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![root().join("patterns").join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                out.push(std::fs::read_to_string(&p).unwrap_or_default());
            }
        }
    }
    out
}

fn public_arities() -> std::collections::BTreeMap<String, usize> {
    let mut out = std::collections::BTreeMap::new();
    for text in pattern_sources() {
        let parsed = syn::parse_file(&text).expect("patterns parse");
        for it in parsed.items {
            if let syn::Item::Fn(f) = it {
                if matches!(f.vis, syn::Visibility::Public(_)) {
                    out.insert(f.sig.ident.to_string(), f.sig.inputs.len());
                }
            }
        }
    }
    out
}

fn public_traits() -> Vec<String> {
    let mut out = Vec::new();
    for text in pattern_sources() {
        let parsed = syn::parse_file(&text).expect("patterns parse");
        for it in parsed.items {
            if let syn::Item::Trait(t) = it {
                if matches!(t.vis, syn::Visibility::Public(_)) {
                    out.push(t.ident.to_string());
                }
            }
        }
    }
    out
}

fn call_arity(text: &str, at: usize) -> Option<usize> {
    let bytes: Vec<char> = text[at..].chars().collect();
    let mut depth = 0usize;
    let mut args = 0usize;
    let mut seen_any = false;
    for c in bytes {
        match c {
            '(' | '[' | '{' | '<' => {
                depth += 1;
            }
            ')' | ']' | '}' | '>' => {
                if depth == 1 {
                    return Some(args + usize::from(seen_any));
                }
                depth = depth.saturating_sub(1);
            }
            ',' if depth == 1 => {
                args += 1;
                seen_any = false;
            }
            c if !c.is_whitespace() && depth >= 1 => seen_any = true,
            _ => {}
        }
    }
    None
}

#[test]
fn every_library_call_the_manual_shows_has_the_librarys_arity() {
    let arities = public_arities();
    let text = manual();
    let mut wrong = Vec::new();
    let mut samples: Vec<String> = text
        .split("```rust")
        .skip(1)
        .map(|block| block.split("```").next().unwrap_or_default().to_string())
        .collect();
    let prose: String = text
        .split("```")
        .step_by(2)
        .collect::<Vec<&str>>()
        .join("
");
    samples.extend(prose.split('`').skip(1).step_by(2).map(|s| s.to_string()));
    for code in &samples {
        let mut from = 0usize;
        while let Some(open) = code[from..].find('(') {
            let at = from + open;
            from = at + 1;
            let head = called_name(&code[..at]);
            let defined_here = code.contains(&format!("fn {}", head));
            if head.is_empty() || defined_here {
                continue;
            }
            let elided = code[at..].starts_with("(..)");
            let (Some(want), Some(got)) = (arities.get(&head), call_arity(code, at)) else {
                continue;
            };
            if *want != got && !elided {
                wrong.push(format!("{}( takes {} arguments in patterns, the manual passes {} in {:?}", head, want, got, code.trim()));
            }
        }
    }
    assert!(wrong.is_empty(), "the manual's samples disagree with the library:\n{}", wrong.join("\n"));
}

#[test]
fn every_public_trait_in_the_library_is_named_in_the_manual() {
    let text = manual();
    let named: Vec<String> = text
        .split('`')
        .skip(1)
        .step_by(2)
        .flat_map(|span| {
            span.split(|c: char| !c.is_alphanumeric() && c != '_')
                .map(|w| w.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let missing: Vec<String> = public_traits()
        .into_iter()
        .filter(|t| !named.contains(t))
        .collect();
    assert!(missing.is_empty(), "the library holds traits the manual never names: {:?}", missing);
}

fn called_name(before: &str) -> String {
    let mut text = before;
    if text.ends_with('>') {
        let mut depth = 0usize;
        let mut cut = None;
        for (i, c) in text.char_indices().rev() {
            match c {
                '>' => depth += 1,
                '<' => {
                    depth -= 1;
                    if depth == 0 {
                        cut = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(open) = cut else { return String::new() };
        text = text[..open].trim_end_matches("::");
    }
    text.chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect()
}

const PROSE_WIDTH: usize = 100;

#[test]
fn every_prose_line_in_the_manual_fits_the_width() {
    let text = manual();
    let mut code = false;
    let mut front = true;
    let mut wide = Vec::new();
    for (at, line) in text.lines().enumerate() {
        if at > 0 && line == "---" {
            front = false;
            continue;
        }
        if line.starts_with("```") {
            code = !code;
            continue;
        }
        let exempt = front || code || line.starts_with('|') || line.starts_with('#');
        if !exempt && line.chars().count() > PROSE_WIDTH {
            wide.push(format!("{}: {} characters", at + 1, line.chars().count()));
        }
    }
    assert!(wide.is_empty(), "prose lines over the width: {:?}", wide);
}

fn workspace_sources() -> String {
    let mut out = String::new();
    let mut stack = vec![root().join("spec"), root().join("patterns"), root().join("tools"), root().join("app")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if p.is_dir() {
                if name != "target" && name != "fixtures" && name != "corpus" {
                    stack.push(p);
                }
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                out.push_str(&std::fs::read_to_string(&p).unwrap_or_default());
                out.push('\n');
            }
        }
    }
    out
}

#[test]
fn every_constant_an_inline_sample_names_is_declared_somewhere() {
    let text = manual();
    let fenced: String = text
        .split("```")
        .skip(1)
        .step_by(2)
        .collect::<Vec<&str>>()
        .join("\n");
    let sources = format!("{}\n{}", workspace_sources(), fenced);
    let prose: String = text.split("```").step_by(2).collect::<Vec<&str>>().join("\n");
    let mut loose = Vec::new();
    for span in prose.split('`').skip(1).step_by(2) {
        for word in span.split(|c: char| !c.is_alphanumeric() && c != '_') {
            let shouty = word.len() > 2
                && word.chars().all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
                && word.chars().any(|c| c.is_ascii_uppercase())
                && word.contains('_');
            if !shouty {
                continue;
            }
            let declared = sources.contains(&format!("const {}", word))
                || sources.contains(&format!("static {}", word))
                || span.contains(&format!("const {}", word));
            if !declared {
                loose.push(format!("{} in `{}`", word, span));
            }
        }
    }
    loose.sort();
    loose.dedup();
    assert!(loose.is_empty(), "inline samples name constants nothing declares: {:?}", loose);
}

#[test]
fn every_item_a_manual_snippet_cites_is_declared_in_that_snippet() {
    let text = manual();
    let sources = workspace_sources();
    let mut loose = Vec::new();
    for block in text.split("```rust").skip(1) {
        let code = block.split("```").next().unwrap_or_default();
        let declared: Vec<String> = code
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                let after = t
                    .strip_prefix("pub const ")
                    .or_else(|| t.strip_prefix("pub struct "))
                    .or_else(|| t.strip_prefix("pub enum "))
                    .or_else(|| t.strip_prefix("pub trait "))
                    .or_else(|| t.strip_prefix("const "))?;
                Some(after.split(|c: char| !c.is_alphanumeric() && c != '_').next().unwrap_or_default().to_string())
            })
            .collect();
        for line in code.lines() {
            let t = line.trim();
            let macros = ["because!(", "provisional!(", "source!(", "supersedes!(", "rejected!(", "decided!("];
            if !macros.iter().any(|m| t.starts_with(m)) {
                continue;
            }
            let inside = t.split('(').nth(1).unwrap_or_default();
            for name in inside.split(',').map(|s| s.trim()).take_while(|s| !s.starts_with('"')) {
                let name = name.trim_end_matches(')').trim_end_matches(';');
                let in_workspace = sources.contains(&format!("const {}", name))
                    || sources.contains(&format!("struct {}", name));
                if !name.is_empty() && !declared.contains(&name.to_string()) && !in_workspace {
                    loose.push(format!("{} cites {} which the snippet does not declare", t, name));
                }
            }
        }
    }
    assert!(loose.is_empty(), "manual snippets cite items they never declare, so they cannot compile:\n{}", loose.join("\n"));
}

#[test]
fn the_gate_maintains_the_census_the_manual_describes() {
    let gate = read(&["tools", "gate", "src", "main.rs"]);
    let binary = read(&["tools", "checker", "src", "main.rs"]);
    for (name, source) in [("gate", &gate), ("checker", &binary)] {
        assert!(
            source.contains("refresh_census"),
            "{} no longer refreshes the census, so the manual's census section and the CI diff step rest on nothing",
            name
        );
    }
}
