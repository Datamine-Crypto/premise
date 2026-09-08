use serde_json::Value;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Target {
    pub kinds: Vec<String>,
    pub src: PathBuf,
    pub gated: bool,
}

#[derive(Clone)]
pub struct Package {
    pub name: String,
    pub dir: PathBuf,
    pub targets: Vec<Target>,
}

#[derive(Clone)]
pub struct Meta {
    pub packages: Vec<Package>,
    pub skipped: Vec<String>,
}

impl Target {
    pub fn is(&self, kind: &str) -> bool {
        self.kinds.iter().any(|k| k == kind)
    }

    pub fn linked(&self) -> bool {
        !self.gated && (self.is("lib") || self.is("rlib") || self.is("proc-macro"))
    }

    pub fn compiled(&self) -> bool {
        !self.gated && !self.is("bench")
    }
}

fn strings(v: Option<&Value>) -> Vec<String> {
    v.and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|s| s.as_str().map(|t| t.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

type Remembered = std::collections::HashMap<(PathBuf, u64, u64), Option<Meta>>;

static META: std::sync::OnceLock<std::sync::Mutex<Remembered>> = std::sync::OnceLock::new();
patterns_macros::because!(
    META,
    "one process-wide answer from cargo metadata per workspace root and manifest stamp, cleared at the start of every report, because spawning cargo is the slowest thing the checker does and every zone and target question asks it"
);

fn stamp(root: &Path) -> (u64, u64) {
    let meta = std::fs::metadata(root.join("Cargo.toml")).ok();
    let changed = meta
        .as_ref()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    (changed, meta.map(|m| m.len()).unwrap_or(0))
}

pub fn forget() {
    if let Some(held) = META.get() {
        if let Ok(mut cache) = held.lock() {
            cache.clear();
        }
    }
}

pub fn read(root: &Path) -> Option<Meta> {
    let (changed, len) = stamp(root);
    let key = (root.to_path_buf(), changed, len);
    let held = META.get_or_init(|| std::sync::Mutex::new(Remembered::new()));
    if let Ok(cache) = held.lock() {
        if let Some(hit) = cache.get(&key) {
            return hit.clone();
        }
    }
    let found = read_fresh(root);
    if let Ok(mut cache) = held.lock() {
        cache.insert(key, found.clone());
    }
    found
}

fn read_fresh(root: &Path) -> Option<Meta> {
    if !root.join("Cargo.toml").is_file() {
        return None;
    }
    let out = std::process::Command::new(
        std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo")),
    )
    .arg("metadata")
    .arg("--no-deps")
    .arg("--format-version")
    .arg("1")
    .arg("--manifest-path")
    .arg(root.join("Cargo.toml"))
    .output()
    .ok()?;
    if !out.status.success() {
        return None;
    }
    let doc: Value = serde_json::from_slice(&out.stdout).ok()?;
    let members = strings(doc.get("workspace_members"));
    let listed = strings(doc.get("workspace_default_members"));
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap_or_default();
    let keyed = manifest
        .parse::<toml::Value>()
        .ok()
        .and_then(|v| v.get("workspace").and_then(|w| w.get("default-members")).cloned())
        .is_some();
    let mut packages = Vec::new();
    let mut skipped = Vec::new();
    for p in doc.get("packages")?.as_array()? {
        let name = p.get("name")?.as_str()?.to_string();
        let id = p.get("id").and_then(|i| i.as_str()).unwrap_or_default().to_string();
        if keyed && members.contains(&id) && !listed.contains(&id) {
            skipped.push(name.clone());
        }
        let manifest = PathBuf::from(p.get("manifest_path")?.as_str()?);
        let dir = manifest.parent()?.to_path_buf();
        let mut targets = Vec::new();
        for t in p.get("targets").and_then(|t| t.as_array())? {
            let src = match t.get("src_path").and_then(|s| s.as_str()) {
                Some(s) => PathBuf::from(s),
                None => continue,
            };
            targets.push(Target {
                kinds: strings(t.get("kind")),
                src,
                gated: !strings(t.get("required-features")).is_empty(),
            });
        }
        packages.push(Package { name, dir, targets });
    }
    Some(Meta { packages, skipped })
}

impl Meta {
    pub fn of(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }
}

pub fn test_programs(root: &Path, package: &str) -> Vec<PathBuf> {
    let out = std::process::Command::new(
        std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo")),
    )
    .arg("test")
    .arg("--no-run")
    .arg("--message-format=json")
    .arg("-p")
    .arg(package)
    .current_dir(root)
    .output();
    let out = match out {
        Ok(v) if v.status.success() => v,
        _ => return Vec::new(),
    };
    let mut found = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let doc: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let tested = doc
            .get("profile")
            .and_then(|p| p.get("test"))
            .and_then(|t| t.as_bool())
            .unwrap_or(false);
        let named = doc
            .get("package_id")
            .and_then(|p| p.as_str())
            .map(|p| p.contains(package))
            .unwrap_or(false);
        if !tested || !named {
            continue;
        }
        if let Some(exe) = doc.get("executable").and_then(|e| e.as_str()) {
            found.push(PathBuf::from(exe));
        }
    }
    found.sort();
    found
}
