use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct Tree {
    pub live: HashSet<PathBuf>,
    pub reachable: HashSet<PathBuf>,
    pub miscased: Vec<(PathBuf, String)>,
}

fn stated_path(attrs: &[syn::Attribute]) -> Option<String> {
    for a in attrs {
        let head = a
            .path()
            .segments
            .last()
            .map(|s| crate::names::plain(&s.ident))
            .unwrap_or_default();
        if head != "path" {
            continue;
        }
        if let syn::Meta::NameValue(nv) = &a.meta {
            if let syn::Expr::Lit(l) = &nv.value {
                if let syn::Lit::Str(s) = &l.lit {
                    return Some(s.value());
                }
            }
        }
    }
    None
}

fn declared(
    items: &[syn::Item],
    prefix: &Path,
    out: &mut Vec<(PathBuf, String, Option<String>, bool)>,
) {
    for it in items {
        if let syn::Item::Mod(m) = it {
            let name = crate::names::plain(&m.ident);
            match &m.content {
                Some((_, inner)) => {
                    let step = stated_path(&m.attrs).unwrap_or(name);
                    declared(inner, &prefix.join(step), out)
                }
                None => out.push((
                    prefix.to_path_buf(),
                    name,
                    stated_path(&m.attrs),
                    matches!(m.vis, syn::Visibility::Public(_)),
                )),
            }
        }
    }
}

fn on_disk(here: &Path, wanted: &str) -> Option<(PathBuf, bool)> {
    let entries = std::fs::read_dir(here).ok()?;
    let mut fallback = None;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name == wanted {
            return Some((entry.path(), true));
        }
        if name.to_lowercase() == wanted.to_lowercase() {
            fallback = Some((entry.path(), false));
        }
    }
    fallback
}

fn reach(
    here: &Path,
    wanted: &str,
    tail: Option<&str>,
    name: &str,
    open: bool,
    tree: &mut Tree,
) {
    let (path, exact) = match on_disk(here, wanted) {
        Some(f) => f,
        None => return,
    };
    let target = match tail {
        Some(t) => path.join(t),
        None => path,
    };
    if !target.is_file() {
        return;
    }
    if !exact {
        tree.miscased.push((target.clone(), name.to_string()));
    }
    walk_from(&target, &here.join(name), open, tree);
}

fn walk_from(file: &Path, children: &Path, open: bool, tree: &mut Tree) {
    if open {
        tree.reachable.insert(file.to_path_buf());
    }
    if !tree.live.insert(file.to_path_buf()) {
        return;
    }
    let text = match std::fs::read_to_string(file) {
        Ok(v) => v,
        Err(_) => return,
    };
    let parsed = match crate::parsed::of(&text) {
        Ok(p) => p,
        Err(_) => return,
    };
    let mut names = Vec::new();
    declared(&parsed.items, Path::new(""), &mut names);
    for (prefix, name, stated, vis) in names {
        let open = open && vis;
        if let Some(p) = stated {
            let own = match file.parent() {
                Some(d) => d.join(&prefix).join(p),
                None => continue,
            };
            if own.is_file() {
                let next = own.parent().map(|d| d.to_path_buf()).unwrap_or_default();
                walk_from(&own, &next, open, tree);
            }
            continue;
        }
        let here = children.join(&prefix);
        reach(&here, &format!("{}.rs", name), None, &name, open, tree);
        reach(&here, &name, Some("mod.rs"), &name, open, tree);
    }
}

fn root(file: &Path, tree: &mut Tree) {
    let here = match file.parent() {
        Some(p) => p.to_path_buf(),
        None => return,
    };
    walk_from(file, &here, true, tree);
}


pub fn from_roots(roots: &[PathBuf]) -> Tree {
    let mut tree = Tree {
        live: HashSet::new(),
        reachable: HashSet::new(),
        miscased: Vec::new(),
    };
    for start in roots {
        if start.is_file() {
            root(start, &mut tree);
        }
    }
    tree
}

thread_local! {
    static REAL: std::cell::RefCell<std::collections::HashMap<PathBuf, PathBuf>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

pub fn real(p: &Path) -> PathBuf {
    REAL.with(|cache| {
        if let Some(hit) = cache.borrow().get(p) {
            return hit.clone();
        }
        let found = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        cache.borrow_mut().insert(p.to_path_buf(), found.clone());
        found
    })
}
