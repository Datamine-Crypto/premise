use crate::zone::Zone;
use std::path::Path;

pub const OPEN: char = 91u8 as char;
pub const CLOSE: char = 93u8 as char;
pub const EQUALS: char = 61u8 as char;
pub const DOT: char = 46u8 as char;

pub struct Crate {
    pub name: String,
    pub lib: String,
    pub dir: String,
    pub deps: Vec<String>,
    pub runtime_deps: Vec<String>,
}

pub struct Library {
    pub package: String,
    pub lib: String,
    pub dir: std::path::PathBuf,
}

pub struct Zones {
    pub crates: Vec<Crate>,
    pub libraries: Vec<Library>,
    pub zone_of: Vec<(String, Zone)>,
    pub unknown_keys: Vec<String>,
    pub missing: Vec<String>,
    pub by_identity: bool,
}

pub const LIBRARY_KEY: &str = "library";
patterns_macros::because!(
    LIBRARY_KEY,
    "the one zones key that names a place rather than a member: the pattern library a project takes from outside its workspace, read for its names and zoned as patterns without being scanned as the project's own"
);

pub const HOME_LIBRARY: &str = "patterns";
patterns_macros::because!(
    HOME_LIBRARY,
    "the crate name every governed path may start from even when no zones file names a library, since the manual teaches the library under that name"
);

fn lib_name_of(manifest: &str, package: &str) -> String {
    let named = manifest
        .parse::<toml::Value>()
        .ok()
        .and_then(|v| v.get("lib").and_then(|l| l.get("name")).and_then(|n| n.as_str()).map(|n| n.to_string()));
    match named {
        Some(n) => n,
        None => package.replace(DASH, "_"),
    }
}

fn package_name_of(manifest: &str) -> Option<String> {
    manifest
        .parse::<toml::Value>()
        .ok()
        .and_then(|v| v.get("package").and_then(|p| p.get("name")).and_then(|n| n.as_str()).map(|n| n.to_string()))
}

fn library_at(root: &Path, value: &str) -> Option<Library> {
    let dir = crate::modules::real(&root.join(value));
    let manifest = std::fs::read_to_string(dir.join("Cargo.toml")).ok()?;
    let package = package_name_of(&manifest)?;
    Some(Library {
        lib: lib_name_of(&manifest, &package),
        package,
        dir,
    })
}

fn owns<'a>(crates: &'a [Crate], p: &str) -> Option<&'a Crate> {
    let mut best: Option<&Crate> = None;
    for c in crates {
        let head = format!("{}/", c.dir);
        if c.dir.is_empty() || p.starts_with(&head) {
            let longer = best.map(|b| c.dir.len() > b.dir.len()).unwrap_or(true);
            if longer {
                best = Some(c);
            }
        }
    }
    best
}

fn inside<'a>(owner: &Crate, p: &'a str) -> &'a str {
    match owner.dir.is_empty() {
        true => p,
        false => &p[owner.dir.len() + 1..],
    }
}

fn named(key: &str) -> Option<Zone> {
    match key {
        "patterns" => Some(Zone::Patterns),
        "spec" => Some(Zone::Spec),
        "app" => Some(Zone::App),
        "tools" => Some(Zone::Tools),
        _ => None,
    }
}

fn unquote(s: &str) -> String {
    s.trim().trim_matches(crate::names::QUOTE).trim().to_string()
}

fn value_after_equals(line: &str) -> String {
    let at = match line.find(EQUALS) {
        Some(a) => a,
        None => return String::new(),
    };
    let rest = &line[at + 1..];
    let mut parts = rest.split(crate::names::QUOTE);
    parts.next();
    match parts.next() {
        Some(v) => v.to_string(),
        None => unquote(rest),
    }
}

fn defaults() -> Vec<(String, Zone)> {
    vec![
        (String::from("patterns"), Zone::Patterns),
        (String::from("spec"), Zone::Spec),
        (String::from("app"), Zone::App),
        (String::from("tools"), Zone::Tools),
    ]
}

impl Zones {
    pub fn load(root: &Path) -> Zones {
        let mut crates: Vec<Crate> = Vec::new();
        if let Some(meta) = crate::cargo::read(root) {
            for pkg in &meta.packages {
                let text =
                    std::fs::read_to_string(pkg.dir.join("Cargo.toml")).unwrap_or_default();
                let dir = crate::modules::real(&pkg.dir);
                let here = crate::modules::real(root);
                let rel = dir
                    .strip_prefix(&here)
                    .unwrap_or(&dir)
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");
                crates.push(Crate {
                    name: pkg.name.clone(),
                    lib: lib_name_of(&text, &pkg.name),
                    dir: rel,
                    deps: crate::manifest::deps(&text),
                    runtime_deps: crate::manifest::runtime_deps(&text),
                });
            }
        }

        let text = std::fs::read_to_string(root.join("premise.zones")).unwrap_or_default();
        let configured = !text.trim().is_empty();
        let mut map: Vec<(String, Zone)> = Vec::new();
        let mut unknown_keys: Vec<String> = Vec::new();
        let mut libraries: Vec<Library> = Vec::new();
        let mut lost: Vec<String> = Vec::new();
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }
            if !line.contains(EQUALS) {
                unknown_keys.push(format!("{} (a line with no {}, which is prose in a file of zone lines)", line, EQUALS));
                continue;
            }
            let key = line
                .split(EQUALS)
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();
            let val = value_after_equals(line);
            if val.is_empty() {
                continue;
            }
            if key == LIBRARY_KEY {
                match library_at(root, &val) {
                    Some(lib) => {
                        map.push((lib.package.clone(), Zone::Patterns));
                        libraries.push(lib);
                    }
                    None => lost.push(val),
                }
                continue;
            }
            match named(&key) {
                Some(z) => {
                    if map.iter().any(|(seen, _)| seen == &val) {
                        unknown_keys.push(format!("{} = {} (already zoned above)", key, val));
                    }
                    map.push((val, z));
                }
                None => unknown_keys.push(key),
            }
        }
        let by_identity = !crates.is_empty();
        if map.is_empty() {
            map = defaults();
        }
        let mut missing: Vec<String> = if !configured {
            Vec::new()
        } else if by_identity {
            map.iter()
                .filter(|(n, _)| !crates.iter().any(|c| &c.name == n))
                .filter(|(n, _)| !libraries.iter().any(|l| &l.package == n))
                .map(|(n, _)| n.clone())
                .collect()
        } else {
            map.iter()
                .filter(|(p, _)| !libraries.iter().any(|l| &l.package == p))
                .filter(|(p, _)| !root.join(p).is_dir())
                .map(|(p, _)| p.clone())
                .collect()
        };
        missing.extend(lost);
        Zones {
            crates,
            libraries,
            zone_of: map,
            unknown_keys,
            missing,
            by_identity,
        }
    }

    pub fn zone_named(&self, name: &str) -> Option<Zone> {
        self.zone_of
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, z)| *z)
    }

    pub fn of(&self, rel: &str) -> Option<Zone> {
        let p = rel.replace(std::path::MAIN_SEPARATOR, "/");
        if self.by_identity {
            let owner = owns(&self.crates, &p)?;
            if inside(owner, &p).starts_with("tests/") {
                return Some(Zone::Tests);
            }
            return self.zone_named(&owner.name);
        }
        let before_src = match p.find("/src/") {
            Some(at) => &p[..at],
            None => p.as_str(),
        };
        if before_src.contains("/tests/") || before_src.ends_with("/tests") {
            return Some(Zone::Tests);
        }
        if p.contains("/tests/") && !p.contains("/src/") {
            return Some(Zone::Tests);
        }
        for (prefix, z) in &self.zone_of {
            let head = format!("{}/", prefix);
            if p.starts_with(&head) {
                return Some(*z);
            }
        }
        None
    }
}

impl Zones {
    pub fn outside_src(&self, rel: &str) -> bool {
        let p = rel.replace(std::path::MAIN_SEPARATOR, "/");
        let owner = match owns(&self.crates, &p) {
            Some(o) => o,
            None => return false,
        };
        let governed = matches!(
            self.zone_named(&owner.name),
            Some(Zone::Spec) | Some(Zone::App) | Some(Zone::Patterns)
        );
        if !governed {
            return false;
        }
        let within = inside(owner, &p);
        !within.starts_with("src/") && !within.starts_with("tests/")
    }
}

pub const SLASH: char = 47u8 as char;
patterns_macros::because!(
    SLASH,
    "the separator between a context name and an item name in a scoped key, chosen because a Rust identifier can never contain it"
);

pub const BSLASH_ALT: char = 92u8 as char;
patterns_macros::because!(
    BSLASH_ALT,
    "the path separator Windows reports, normalised away so a scope given with forward slashes matches a diagnostic printed with back ones"
);

impl Zones {
    pub fn crate_of(&self, rel: &str) -> String {
        let p = rel.replace(std::path::MAIN_SEPARATOR, "/");
        owns(&self.crates, &p)
            .map(|c| c.name.clone())
            .unwrap_or_default()
    }
}

impl Zones {
    pub fn library_roots(&self) -> Vec<String> {
        let mut out = vec![String::from(HOME_LIBRARY)];
        for lib in &self.libraries {
            out.push(lib.lib.clone());
        }
        for c in &self.crates {
            if self.zone_named(&c.name) == Some(Zone::Patterns) {
                out.push(c.lib.clone());
            }
        }
        out.sort();
        out.dedup();
        out
    }

    pub fn library_sources(&self) -> Vec<std::path::PathBuf> {
        let mut out = Vec::new();
        for lib in &self.libraries {
            let manifest = std::fs::read_to_string(lib.dir.join("Cargo.toml")).unwrap_or_default();
            if manifest.contains("proc-macro") {
                continue;
            }
            out.extend(crate::rust_files(&lib.dir.join("src")));
        }
        out
    }

    pub fn holds_library(&self, path: &Path) -> bool {
        self.libraries.iter().any(|l| path.starts_with(&l.dir))
    }
}

pub const DASH: char = 45u8 as char;
patterns_macros::because!(
    DASH,
    "the character cargo turns into an underscore when a package name becomes a crate name, so a zoned member is matched by the name a path in the code can spell"
);
