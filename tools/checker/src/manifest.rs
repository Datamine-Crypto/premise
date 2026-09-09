use toml::Value;

fn as_table(v: Option<&Value>) -> Option<&toml::map::Map<String, Value>> {
    v.and_then(|x| x.as_table())
}

fn harvest(table: Option<&toml::map::Map<String, Value>>, out: &mut Vec<String>) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    for (key, value) in table {
        let real = value
            .get("package")
            .and_then(|p| p.as_str())
            .unwrap_or(key.as_str());
        out.push(real.to_string());
    }
}

const KINDS: &[&str] = &["dependencies", "dev-dependencies", "build-dependencies"];
patterns_macros::because!(
    KINDS,
    "the three tables cargo reads dependencies from, listed once so a crate reached through a dev or build dependency is seen the same as one reached through the main table"
);

pub fn runtime_deps(text: &str) -> Vec<String> {
    let doc: Value = match text.parse() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for kind in KINDS {
        if *kind == "dev-dependencies" {
            continue;
        }
        harvest(as_table(doc.get(*kind)), &mut out);
    }
    if let Some(targets) = as_table(doc.get("target")) {
        for (_, per_target) in targets {
            for kind in KINDS {
                if *kind == "dev-dependencies" {
                    continue;
                }
                harvest(as_table(per_target.get(*kind)), &mut out);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

pub fn deps(text: &str) -> Vec<String> {
    let doc: Value = match text.parse() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for kind in KINDS {
        harvest(as_table(doc.get(*kind)), &mut out);
    }
    if let Some(targets) = as_table(doc.get("target")) {
        for (_, per_target) in targets {
            for kind in KINDS {
                harvest(as_table(per_target.get(*kind)), &mut out);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}





pub fn path_deps(text: &str) -> Vec<String> {
    let doc: Value = match text.parse() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for kind in KINDS {
        if let Some(table) = as_table(doc.get(*kind)) {
            for (_, value) in table {
                if let Some(p) = value.get("path").and_then(|p| p.as_str()) {
                    out.push(p.to_string());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

pub const CARGO_OVERRIDES: &[&str] = &["runner", "rustc", "rustc-wrapper", "rustc-workspace-wrapper", "rustflags", "rustdocflags"];
patterns_macros::because!(
    CARGO_OVERRIDES,
    "the cargo config keys that change what runs or what compiles when the gate invokes cargo, so a file setting one of them can make a red tree print green without touching a governed line"
);

pub const CARGO_TABLES: &[&str] = &["env", "alias", "patch", "paths", "source", "target", "build", "net"];
patterns_macros::because!(
    CARGO_TABLES,
    "the cargo config tables through which a runner, a wrapper, a flag or a substituted crate can be introduced, refused wholesale because a governed workspace has no lawful use for any of them"
);

fn keys_in(table: &toml::map::Map<String, Value>, found: &mut Vec<String>) {
    for (key, value) in table {
        if CARGO_OVERRIDES.contains(&key.as_str()) || CARGO_TABLES.contains(&key.as_str()) {
            found.push(key.clone());
        }
        if let Some(inner) = value.as_table() {
            keys_in(inner, found);
        }
    }
}

pub fn cargo_config_overrides(text: &str) -> Vec<String> {
    let doc: Value = match text.parse() {
        Ok(v) => v,
        Err(_) => return vec![String::from("unparseable")],
    };
    let mut found = Vec::new();
    if let Some(table) = doc.as_table() {
        keys_in(table, &mut found);
    }
    found.sort();
    found.dedup();
    found
}

pub const TOOLCHAIN_OVERRIDES: &[&str] = &["path"];
patterns_macros::because!(
    TOOLCHAIN_OVERRIDES,
    "the rust-toolchain key that selects a compiler shipped inside the repository instead of one rustup installed, the same class of substitution as a cargo runner; a channel name is the toolchain file's lawful content"
);

pub fn toolchain_overrides(text: &str) -> Vec<String> {
    let doc: Value = match text.parse() {
        Ok(v) => v,
        Err(_) => return vec![String::from("unparseable")],
    };
    let mut found = Vec::new();
    if let Some(table) = doc.get("toolchain").and_then(|t| t.as_table()) {
        for key in table.keys() {
            if TOOLCHAIN_OVERRIDES.contains(&key.as_str()) {
                found.push(key.clone());
            }
        }
    }
    found
}

pub fn legacy_toolchain_overrides(text: &str) -> Vec<String> {
    let bare = text.trim();
    let channel_only = !bare.is_empty() && !bare.contains(char::is_whitespace) && !bare.contains('=');
    match channel_only {
        true => Vec::new(),
        false => toolchain_overrides(text),
    }
}

pub fn commented(line: &str) -> bool {
    let mut in_double = false;
    let mut in_single = false;
    for c in line.chars() {
        match c {
            '"' if !in_single => in_double = !in_double,
            '\'' if !in_double => in_single = !in_single,
            '#' if !in_double && !in_single => return true,
            _ => {}
        }
    }
    false
}

pub const PROSE_FIELDS: &[&str] = &["description", "readme", "keywords", "categories", "metadata"];
patterns_macros::because!(
    PROSE_FIELDS,
    "the manifest entries whose value is free text about the crate rather than a name or a version, so a sentence can hide in one where the comment ban does not read"
);

pub const DESCRIPTION: &str = "description";
patterns_macros::because!(
    DESCRIPTION,
    "the one prose entry a registry demands before it accepts a crate, so it is read as a reason for the crate rather than refused: it must pass the same checks a because! passes"
);

pub const README: &str = "readme";
patterns_macros::because!(
    README,
    "the one entry in the list whose value a registry reads as a path rather than as text, so it is read as a pointer at prose that already exists rather than as prose hidden in a manifest"
);

fn reads_as_reason(package: &str, value: &str) -> bool {
    crate::because::weak(package, value).is_none()
        && crate::because::states_a_fact(value).is_none()
        && crate::because::leans_on(value).is_none()
}

fn points_at_a_file(at: &std::path::Path, value: &str) -> bool {
    at.parent().map(|dir| dir.join(value).is_file()).unwrap_or(false)
}

pub fn prose_fields(text: &str, at: &std::path::Path) -> Vec<String> {
    let doc: Value = match text.parse() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut found = Vec::new();
    let inherited = doc.get("workspace").and_then(|w| w.get("package"));
    let package = doc
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or_default();
    for t in [doc.get("package"), doc.get("workspace"), inherited] {
        if let Some(t) = t.and_then(|t| t.as_table()) {
            for (key, value) in t {
                let described = key == DESCRIPTION && value.as_str().map(|v| reads_as_reason(package, v)).unwrap_or(false);
                let pointed = key == README && value.as_str().map(|v| points_at_a_file(at, v)).unwrap_or(false);
                if PROSE_FIELDS.contains(&key.as_str()) && !described && !pointed {
                    found.push(key.clone());
                }
            }
        }
    }
    found
}
