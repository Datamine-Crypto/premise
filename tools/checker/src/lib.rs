pub mod because;
pub mod cargo;
pub mod facts;
pub mod boundary;
pub mod config;
pub mod comment;
pub mod diag;
pub mod manifest;
pub mod modules;
pub mod names;
pub mod parsed;
pub mod roots;
pub mod scan;
pub mod shape;
pub mod zone;

use patterns_macros::because;

const MIN_SHAPE_TOKENS: usize = 12;
because!(
    MIN_SHAPE_TOKENS,
    "the length below which two functions collide by coincidence rather than by copying"
);

const MIN_VARIANTS: usize = 2;
because!(
    MIN_VARIANTS,
    "a single variant carries no vocabulary, so comparison begins where a choice does"
);

const MAX_DRIFT: usize = 2;
because!(
    MAX_DRIFT,
    "the widest gap in edits where a pair still reads as one function copied"
);

const STACK_BYTES: usize = 268435456;
because!(
    STACK_BYTES,
    "syn recurses once per nesting level and the default thread stack overflows near a hundred levels, while rustc accepts far deeper; this much stack outlasts anything rustc compiles, so an expression is answered with a diagnostic rather than an abort"
);

pub fn tally(n: usize, one: &str, many: &str) -> String {
    match n {
        1 => format!("{} {}", n, one),
        _ => format!("{} {}", n, many),
    }
}

pub const CONTEXT_FILES: &[&str] = &["mod.rs", "vocabulary.rs", "state.rs", "reducer.rs"];
because!(
    CONTEXT_FILES,
    "the four files a context is made of; a fifth would be a place to keep state or rules that the boundary check does not know to guard"
);

pub const OWN_TESTS: &str = "tools/checker/tests/";
because!(
    OWN_TESTS,
    "where the checker crate keeps its fixtures and corpus of deliberately unlawful code when no workspace manifest says otherwise; with a manifest the crate named checker is found through the zones, so a vendored checker under another path keeps its fixtures skipped and every other crate's fixtures directory answers to the test zone"
);

use diag::Diag;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use zone::Zone;
use syn::spanned::Spanned;
use syn::visit::Visit;

struct Inline {
    bound: HashMap<String, syn::Expr>,
}

impl syn::fold::Fold for Inline {
    fn fold_block(&mut self, b: syn::Block) -> syn::Block {
        let mut stmts = Vec::new();
        for stmt in b.stmts {
            if let syn::Stmt::Local(l) = &stmt {
                let pat = match &l.pat {
                    syn::Pat::Type(t) => &*t.pat,
                    other => other,
                };
                if let (syn::Pat::Ident(p), Some(init)) = (pat, &l.init) {
                    if let Some(bare) = bare_path(&init.expr) {
                        let folded = syn::fold::Fold::fold_expr(self, bare);
                        self.bound.insert(crate::names::plain(&p.ident), folded);
                        continue;
                    }
                }
            }
            stmts.push(syn::fold::fold_stmt(self, stmt));
        }
        syn::Block {
            brace_token: b.brace_token,
            stmts,
        }
    }

    fn fold_expr(&mut self, e: syn::Expr) -> syn::Expr {
        if let syn::Expr::Path(p) = &e {
            if let Some(one) = p.path.get_ident() {
                if let Some(init) = self.bound.get(&crate::names::plain(one)) {
                    return init.clone();
                }
            }
        }
        syn::fold::fold_expr(self, e)
    }
}

fn bare_path(e: &syn::Expr) -> Option<syn::Expr> {
    match e {
        syn::Expr::Path(_) => Some(e.clone()),
        syn::Expr::Paren(p) => bare_path(&p.expr),
        syn::Expr::Reference(r) => bare_path(&r.expr),
        syn::Expr::Unary(u) => bare_path(&u.expr),
        syn::Expr::Block(b) if b.block.stmts.len() == 1 => match b.block.stmts.first() {
            Some(syn::Stmt::Expr(inner, _)) => bare_path(inner),
            _ => None,
        },
        _ => None,
    }
}

fn inlined(f: &syn::ItemFn) -> syn::ItemFn {
    let mut folder = Inline {
        bound: HashMap::new(),
    };
    syn::fold::Fold::fold_item_fn(&mut folder, f.clone())
}

fn shouty_names(f: &syn::ItemFn) -> Vec<String> {
    let mut out = Vec::new();
    let text = quote::ToTokens::to_token_stream(&f.block).to_string();
    for word in text.split(|c: char| !c.is_alphanumeric() && c != '_') {
        let upper = !word.is_empty()
            && word.chars().all(|c| c.is_uppercase() || c.is_ascii_digit() || c == '_')
            && word.chars().any(|c| c.is_uppercase());
        if upper && !out.iter().any(|w| w == word) {
            out.push(word.to_string());
        }
    }
    out.sort();
    out
}

struct Binding {
    shouty: Vec<String>,
    profile: shape::Profile,
}

impl Binding {
    fn of(f: &syn::ItemFn) -> Binding {
        let flat = inlined(f);
        Binding {
            shouty: shouty_names(&flat),
            profile: shape::profile(&flat.sig, &flat.block),
        }
    }
}

fn same_but_for_constants(a: &Binding, b: &Binding) -> Option<(String, String)> {
    let (ka, kb) = (&a.shouty, &b.shouty);
    if ka.is_empty() || kb.is_empty() || ka == kb {
        return None;
    }
    let (pa, pb) = (&a.profile, &b.profile);
    if pa.tokens.len() != pb.tokens.len() {
        return None;
    }
    let bodies_match = pa.tokens.iter().zip(pb.tokens.iter()).all(|(x, y)| x == y);
    if !bodies_match {
        return None;
    }
    let only_a: Vec<&String> = ka.iter().filter(|k| !kb.contains(k)).collect();
    let only_b: Vec<&String> = kb.iter().filter(|k| !ka.contains(k)).collect();
    if only_a.is_empty() || only_b.is_empty() {
        return None;
    }
    Some((
        only_a
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" and "),
        only_b
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" and "),
    ))
}


struct Shaped {
    profile: shape::Profile,
    file: String,
    line: usize,
    name: String,
    reason: String,
    dictated: bool,
    binding: bool,
}

fn library_shapes(zones: &config::Zones) -> Vec<Shaped> {
    let mut out = Vec::new();
    for f in zones.library_sources() {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        let parsed = match parsed::of(&text) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let mut fns = Fns {
            items: Vec::new(),
            in_trait_impl: false,
        };
        fns.visit_file(&parsed);
        let shown = f.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/");
        let shown = shown.trim_start_matches("//?/").to_string();
        for (sig, block, line, dictated) in &fns.items {
            out.push(Shaped {
                profile: shape::profile(sig, block),
                file: shown.clone(),
                line: *line,
                name: names::plain(&sig.ident),
                reason: String::new(),
                dictated: *dictated,
                binding: false,
            });
        }
    }
    out
}

fn names_each_other(a: &Shaped, b: &Shaped) -> bool {
    let mentions = |text: &str, name: &str| {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == name)
    };
    mentions(&a.reason, &b.name) && mentions(&b.reason, &a.name)
}

struct Fns {
    items: Vec<(syn::Signature, syn::Block, usize, bool)>,
    in_trait_impl: bool,
}

pub const ENTRY_POINTS: &[&str] = &["proc_macro", "proc_macro_derive", "proc_macro_attribute", "no_mangle"];
because!(
    ENTRY_POINTS,
    "the attributes that make a function a name the language requires rather than logic somebody chose to write, so two of them being alike is the compiler's shape and not a duplicate"
);

pub fn is_entry_point(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        let head = a
            .path()
            .segments
            .last()
            .map(|s| names::plain(&s.ident))
            .unwrap_or_default();
        ENTRY_POINTS.contains(&head.as_str())
    })
}

impl<'ast> Visit<'ast> for Fns {
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        if is_entry_point(&f.attrs) {
            return;
        }
        self.items.push((
            f.sig.clone(),
            (*f.block).clone(),
            f.sig.ident.span().start().line,
            false,
        ));
        syn::visit::visit_item_fn(self, f);
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let prev = self.in_trait_impl;
        self.in_trait_impl = i.trait_.is_some();
        syn::visit::visit_item_impl(self, i);
        self.in_trait_impl = prev;
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        self.items.push((
            f.sig.clone(),
            f.block.clone(),
            f.sig.ident.span().start().line,
            self.in_trait_impl,
        ));
        syn::visit::visit_impl_item_fn(self, f);
    }
}

fn context_aliases(items: &[syn::Item], out: &mut Vec<String>) {
    fn walk(t: &syn::UseTree, out: &mut Vec<String>) {
        match t {
            syn::UseTree::Path(p) => walk(&p.tree, out),
            syn::UseTree::Rename(r) if crate::names::is(&r.ident, "Context") => {
                out.push(crate::names::plain(&r.rename));
            }
            syn::UseTree::Group(g) => {
                for i in &g.items {
                    walk(i, out);
                }
            }
            _ => {}
        }
    }
    for it in items {
        if let syn::Item::Use(u) = it {
            walk(&u.tree, out);
        }
    }
}

fn shared_overreach(items: &[syn::Item]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut reducers = vec![String::from("Context")];
    context_aliases(items, &mut reducers);
    for it in items {
        match it {
            syn::Item::Impl(im) => {
                let trait_name = im
                    .trait_
                    .as_ref()
                    .and_then(|(_, p, _)| p.segments.last())
                    .map(|s| crate::names::plain(&s.ident))
                    .unwrap_or_default();
                if reducers.contains(&trait_name) {
                    out.push((im.span().start().line, String::from("a reducer")));
                }
                for sub in &im.items {
                    if let syn::ImplItem::Fn(m) = sub {
                        let table = match m.block.stmts.first() {
                            Some(syn::Stmt::Expr(syn::Expr::Match(mt), _)) if m.block.stmts.len() == 1 => mt
                                .arms
                                .iter()
                                .all(|arm| matches!(&*arm.body, syn::Expr::Path(_) | syn::Expr::Lit(_))),
                            _ => false,
                        };
                        if !table {
                            out.push((
                                m.span().start().line,
                                format!("fn {} has a body that is not a total match over own variants, which is logic", crate::names::plain(&m.sig.ident)),
                            ));
                        }
                    }
                }
            }
            syn::Item::Struct(s) => {
                if !matches!(s.fields, syn::Fields::Unit) {
                    out.push((
                        s.span().start().line,
                        format!("struct {} carries fields, which is state", crate::names::plain(&s.ident)),
                    ));
                }
            }
            syn::Item::Enum(e) => {
                if e.variants.iter().any(|v| !matches!(v.fields, syn::Fields::Unit)) {
                    out.push((
                        e.span().start().line,
                        format!("enum {} carries data in a variant, which is state", crate::names::plain(&e.ident)),
                    ));
                }
            }
            syn::Item::Trait(t) => {
                out.push((
                    t.span().start().line,
                    format!("trait {} is a local trait definition, which is a rule", crate::names::plain(&t.ident)),
                ));
            }
            syn::Item::Type(t) => {
                out.push((
                    t.span().start().line,
                    format!("type {} is an alias, which can name state", crate::names::plain(&t.ident)),
                ));
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    out.extend(shared_overreach(inner));
                }
            }
            _ => {}
        }
    }
    out
}

fn cargo_configs(root: &Path) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    let skip = own_tests_dir(root);
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if p.is_dir() {
                let inside = p
                    .strip_prefix(root)
                    .unwrap_or(&p)
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");
                let held = (name == "fixtures" || name == "corpus") && inside.starts_with(&skip);
                if name != "target" && name != ".git" && !held {
                    stack.push(p);
                }
                continue;
            }
            let under_cargo = dir.file_name().map(|d| d == ".cargo").unwrap_or(false);
            if name == "rust-toolchain.toml" || name == "rust-toolchain" {
                let text = std::fs::read_to_string(&p).unwrap_or_default();
                let keys = match name.ends_with(".toml") {
                    true => manifest::toolchain_overrides(&text),
                    false => manifest::legacy_toolchain_overrides(&text),
                };
                if !keys.is_empty() {
                    let rel = p
                        .strip_prefix(root)
                        .unwrap_or(&p)
                        .to_string_lossy()
                        .replace(std::path::MAIN_SEPARATOR, "/");
                    out.push((rel, keys));
                }
            }
            if under_cargo && (name == "config.toml" || name == "config") {
                let text = std::fs::read_to_string(&p).unwrap_or_default();
                let keys = manifest::cargo_config_overrides(&text);
                if !keys.is_empty() {
                    let rel = p
                        .strip_prefix(root)
                        .unwrap_or(&p)
                        .to_string_lossy()
                        .replace(std::path::MAIN_SEPARATOR, "/");
                    out.push((rel, keys));
                }
            }
        }
    }
    out.sort();
    out
}

fn manifest_comments(root: &Path) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let skip = own_tests_dir(root);
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let inside = p
                .strip_prefix(root)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            if p.is_dir() {
                let held = (name == "fixtures" || name == "corpus") && inside.starts_with(&skip);
                if name != "target" && name != ".git" && !held {
                    stack.push(p);
                }
                continue;
            }
            if name != "Cargo.toml" {
                continue;
            }
            let text = std::fs::read_to_string(&p).unwrap_or_default();
            for (at, line) in text.lines().enumerate() {
                if manifest::commented(line) {
                    out.push((inside.clone(), at + 1));
                }
            }
            for key in manifest::prose_fields(&text, &p) {
                let at = text.lines().position(|l| l.trim_start().starts_with(&key)).map(|n| n + 1).unwrap_or_default();
                out.push((format!("{} ({})", inside, key), at));
            }
        }
    }
    out.sort();
    out
}

fn walk(root: &Path, skip: &str, out: &mut Vec<PathBuf>) {
    let mut seen = std::collections::HashSet::new();
    walk_seen(root, root, skip, out, &mut seen);
}

fn walk_seen(
    top: &Path,
    root: &Path,
    skip: &str,
    out: &mut Vec<PathBuf>,
    seen: &mut std::collections::HashSet<PathBuf>,
) {
    if !seen.insert(modules::real(root)) {
        return;
    }
    let rd = match std::fs::read_dir(root) {
        Ok(r) => r,
        Err(_) => return,
    };
    for e in rd.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            let inside = p
                .strip_prefix(top)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            let held = name == "fixtures" || name == "corpus";
            let is_corpus = held && inside.starts_with(skip);
            if name == "target" || name == ".git" || is_corpus {
                continue;
            }
            walk_seen(top, &p, skip, out, seen);
        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
}

fn use_root(t: &syn::UseTree) -> Option<String> {
    match t {
        syn::UseTree::Path(p) => Some(crate::names::plain(&p.ident)),
        syn::UseTree::Name(n) => Some(crate::names::plain(&n.ident)),
        syn::UseTree::Rename(r) => Some(crate::names::plain(&r.ident)),
        _ => None,
    }
}

fn reexports(items: &[syn::Item], out: &mut Vec<(usize, String)>) {
    for it in items {
        match it {
            syn::Item::Use(u) => {
                if !matches!(u.vis, syn::Visibility::Public(_)) {
                    continue;
                }
                if let Some(r) = use_root(&u.tree) {
                    out.push((u.span().start().line, r));
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    reexports(inner, out);
                }
            }
            _ => {}
        }
    }
}


fn project_uses(items: &[syn::Item], out: &mut Vec<(usize, String)>) {
    for it in items {
        match it {
            syn::Item::Use(u) => {
                if let Some(r) = use_root(&u.tree) {
                    if r == "spec" || r == "app" {
                        out.push((u.span().start().line, r));
                    }
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    project_uses(inner, out);
                }
            }
            _ => {}
        }
    }
}

pub struct Report {
    pub diags: Vec<Diag>,
    pub exempt: usize,
    pub checked: usize,
}

pub fn run(root: &Path) -> Vec<Diag> {
    report(root).diags
}

fn own_tests_dir(root: &Path) -> String {
    let zones = config::Zones::load(root);
    zones
        .crates
        .iter()
        .find(|c| c.name == "checker")
        .map(|c| format!("{}/tests/", c.dir))
        .unwrap_or_else(|| String::from(OWN_TESTS))
}

pub fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let skip = own_tests_dir(root);
    walk(root, &skip, &mut out);
    out.sort();
    out
}

struct Shouty {
    names: std::collections::HashSet<String>,
}

impl<'ast> Visit<'ast> for Shouty {
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        let id = crate::names::plain(&f.sig.ident);
        if id.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            self.names.insert(id);
        }
        syn::visit::visit_item_fn(self, f);
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        let id = crate::names::plain(&f.sig.ident);
        if id.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            self.names.insert(id);
        }
        syn::visit::visit_impl_item_fn(self, f);
    }
}

fn public_fn_names(items: &[syn::Item], out: &mut std::collections::HashSet<String>) {
    for it in items {
        match it {
            syn::Item::Fn(f) if facts::is_public(&f.vis) => {
                out.insert(crate::names::plain(&f.sig.ident));
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    public_fn_names(inner, out);
                }
            }
            _ => {}
        }
    }
}

fn type_names(items: &[syn::Item], out: &mut std::collections::HashSet<String>) {
    for it in items {
        match it {
            syn::Item::Struct(s) => {
                out.insert(crate::names::plain(&s.ident));
            }
            syn::Item::Enum(e) => {
                out.insert(crate::names::plain(&e.ident));
            }
            syn::Item::Trait(t) => {
                out.insert(crate::names::plain(&t.ident));
            }
            syn::Item::Type(t) => {
                if !scan::launders(&t.ty) {
                    out.insert(crate::names::plain(&t.ident));
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    type_names(inner, out);
                }
            }
            _ => {}
        }
    }
}

fn value_items(items: &[syn::Item], plain: &mut HashMap<String, i128>, aliased: &mut HashMap<String, String>) {
    for it in items {
        match it {
            syn::Item::Const(c) => {
                let name = crate::names::plain(&c.ident).to_lowercase();
                match &*c.expr {
                    syn::Expr::Lit(l) => {
                        if let syn::Lit::Int(v) = &l.lit {
                            if let Ok(n) = v.base10_digits().parse::<i128>() {
                                plain.insert(name, n);
                            }
                        }
                    }
                    syn::Expr::Path(p) => {
                        if let Some(last) = p.path.segments.last() {
                            aliased.insert(name, crate::names::plain(&last.ident).to_lowercase());
                        }
                    }
                    _ => {}
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    value_items(inner, plain, aliased);
                }
            }
            _ => {}
        }
    }
}

const DECLARATION_ITSELF: usize = 1;
because!(
    DECLARATION_ITSELF,
    "the mention every declared name has, its own declaration, so a name counted once over the whole tree is used by nothing and a name counted twice is used once"
);

fn reasoned_in(items: &[syn::Item], f: &because::Found, rel: &str, out: &mut Vec<(String, String, usize)>) {
    for it in items {
        if let syn::Item::Mod(m) = it {
            if let Some((_, inner)) = &m.content {
                reasoned_in(inner, f, rel, out);
            }
            continue;
        }
        let name = match it {
            syn::Item::Struct(st) => crate::names::plain(&st.ident),
            syn::Item::Enum(e) => crate::names::plain(&e.ident),
            syn::Item::Trait(t) => crate::names::plain(&t.ident),
            syn::Item::Type(t) => crate::names::plain(&t.ident),
            _ => continue,
        };
        if f.sources.contains(&name) {
            continue;
        }
        if let Some((_, _, line)) = f.reasons.iter().find(|(item, _, _)| item == &name) {
            out.push((name, rel.to_string(), *line));
        }
    }
}

fn marker_items(items: &[syn::Item], ctx: &str, rel: &str, out: &mut HashMap<String, Vec<String>>, sites: &mut Vec<(String, String, String, usize)>) {
    for it in items {
        match it {
            syn::Item::Impl(im) => {
                let on_context = im
                    .trait_
                    .as_ref()
                    .and_then(|(_, p, _)| p.segments.last())
                    .map(|s| crate::names::plain(&s.ident) == "Context")
                    .unwrap_or(false);
                if let (true, syn::Type::Path(tp)) = (on_context, &*im.self_ty) {
                    if let Some(last) = tp.path.segments.last() {
                        out.entry(crate::names::plain(&last.ident)).or_default().push(ctx.to_string());
                        sites.push((crate::names::plain(&last.ident), ctx.to_string(), rel.to_string(), syn::spanned::Spanned::span(im).start().line));
                    }
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    marker_items(inner, ctx, rel, out, sites);
                }
            }
            _ => {}
        }
    }
}

fn context_markers(root: &Path) -> (HashMap<String, Vec<String>>, Vec<Diag>) {
    let mut out = HashMap::new();
    let mut sites = Vec::new();
    for f in compiled_sources(root, zone::Zone::Spec) {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(&f)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        let ctx = boundary::context_of(&rel).unwrap_or_default();
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        if let Ok(parsed) = parsed::of(&text) {
            marker_items(&parsed.items, &ctx, &rel, &mut out, &mut sites);
        }
    }
    let mut dups = Vec::new();
    for (name, ctx, rel, line) in &sites {
        let elsewhere: Vec<String> = out
            .get(name)
            .map(|owners| owners.iter().filter(|o| *o != ctx).cloned().collect())
            .unwrap_or_default();
        if !elsewhere.is_empty() {
            dups.push(Diag::new(
                "E-DUP-NAME",
                rel,
                *line,
                format!("marker {} also runs {}; a struct carrying impl Context is a fact, and one name is one fact, so two contexts name their markers apart", name, elsewhere.join(" and ")),
            ));
        }
    }
    (out, dups)
}

fn declared_values(root: &Path) -> HashMap<String, i128> {
    let mut plain = HashMap::new();
    let mut aliased = HashMap::new();
    for f in compiled_sources(root, zone::Zone::Spec) {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        if let Ok(parsed) = parsed::of(&text) {
            value_items(&parsed.items, &mut plain, &mut aliased);
        }
    }
    let mut settled = true;
    while settled {
        settled = false;
        for (name, target) in &aliased {
            if !plain.contains_key(name) {
                if let Some(n) = plain.get(target).copied() {
                    plain.insert(name.clone(), n);
                    settled = true;
                }
            }
        }
    }
    plain
}

fn laundering_names(items: &[syn::Item], out: &mut std::collections::HashSet<String>) {
    for it in items {
        match it {
            syn::Item::Type(t) if scan::launders(&t.ty) => {
                out.insert(crate::names::plain(&t.ident));
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    laundering_names(inner, out);
                }
            }
            _ => {}
        }
    }
}

fn names_in(
    root: &Path,
    want: zone::Zone,
    collect: fn(&[syn::Item], &mut std::collections::HashSet<String>),
) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    let mut sources = compiled_sources(root, want);
    if want == zone::Zone::Patterns {
        sources.extend(config::Zones::load(root).library_sources());
    }
    for f in sources {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        if let Ok(parsed) = parsed::of(&text) {
            collect(&parsed.items, &mut out);
        }
    }
    out
}


fn shouty_fns(files: &[PathBuf]) -> std::collections::HashSet<String> {
    let mut seen = Shouty {
        names: std::collections::HashSet::new(),
    };
    for f in files {
        let text = match std::fs::read_to_string(f) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if let Ok(parsed) = parsed::of(&text) {
            seen.visit_file(&parsed);
        }
    }
    seen.names
}

pub fn on_big_stack<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    let spawned = std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(work);
    match spawned {
        Ok(handle) => match handle.join() {
            Ok(done) => done,
            Err(payload) => std::panic::resume_unwind(payload),
        },
        Err(e) => panic!("a thread with a large stack could not be started: {}", e),
    }
}

pub fn report(root: &Path) -> Report {
    cargo::forget();
    let here = root.to_path_buf();
    on_big_stack(move || report_here(&here))
}

const HOLE: &str = "{shown}";
because!(
    HOLE,
    "the named gap where the offending words go in a message written once and filled in per site, so two reports of one check are one string rather than two that drift, and named rather than bare so a maintainer editing the sentence can see what the gap holds"
);

const DIGITS_WRITTEN: &str = "writes {shown} into prose, so a value lives outside the spec where nothing checks it and nothing keeps two copies of it equal. Name it: a spec const if it is a value, or a source! item if it is the study, contract or release the reason rests on, then cite that instead";
because!(
    DIGITS_WRITTEN,
    "what a reason carrying digits is told, which is that the value now has a second copy nothing compares"
);

const VALUE_SPELLED: &str = "spells {shown} in prose, which is the value itself, so the reason goes stale the day the value changes. Say why this value and not another";
because!(
    VALUE_SPELLED,
    "what a reason spelling out its own item's value is told, which is a different fault from digits because the words read as English until the value moves"
);

struct Vocab {
    names: Vec<String>,
    ctx: String,
    file: String,
    line: usize,
    item: String,
    excused: bool,
}

fn fact_in_reason(rel: &str, line: usize, wrote: &str, item: &str, said: &str, shown: &str) -> Diag {
    Diag::new(
        "E-FACT-IN-REASON",
        rel,
        line,
        format!("{}!({}) {}", wrote, item, said.replace(HOLE, shown)),
    )
}

fn overlapping_vocabularies(vocabs: &[Vocab]) -> Vec<Diag> {
    let mut diags: Vec<Diag> = Vec::new();
    for a in 0..vocabs.len() {
        for b in (a + 1)..vocabs.len() {
            let one = &vocabs[a];
            let two = &vocabs[b];
            if one.excused || two.excused || one.ctx == two.ctx {
                continue;
            }
            let apart = shape::set_apart(&one.names, &two.names);
            let smallest = one.names.len().min(two.names.len());
            if apart > smallest {
                continue;
            }
            let a_shared = one.ctx == boundary::SHARED;
            let b_shared = two.ctx == boundary::SHARED;
            let (file, line, mine, ctx, theirs) = match b_shared && !a_shared {
                true => (&one.file, one.line, &one.item, &one.ctx, &two.item),
                false => (&two.file, two.line, &two.item, &two.ctx, &one.item),
            };
            let shared = a_shared || b_shared;
            let note = if shared && apart == 0 {
                format!(
                    "{} in {} repeats contexts::shared::vocabulary::{}; name the shared type instead",
                    mine, ctx, theirs
                )
            } else if shared {
                format!(
                    "{} in {} differs in {} variants from contexts::shared::vocabulary::{}",
                    mine, ctx, apart, theirs
                )
            } else if apart == 0 {
                format!(
                    "{} in {} and {} in {} are the same vocabulary; move it to contexts/shared",
                    two.item, two.ctx, one.item, one.ctx
                )
            } else {
                format!(
                    "{} in {} differs in {} variants from {} in {} at {}:{}",
                    two.item, two.ctx, apart, one.item, one.ctx, one.file, one.line
                )
            };
            diags.push(Diag::new("E-DUP-VOCABULARY", file, line, note));
            if !shared {
                let back = match apart == 0 {
                    true => format!(
                        "{} in {} and {} in {} are the same vocabulary; move it to contexts/shared",
                        one.item, one.ctx, two.item, two.ctx
                    ),
                    false => format!(
                        "{} in {} differs in {} variants from {} in {} at {}:{}",
                        one.item, one.ctx, apart, two.item, two.ctx, two.file, two.line
                    ),
                };
                diags.push(Diag::new("E-DUP-VOCABULARY", &one.file, one.line, back));
            }
        }
    }
    diags
}

fn near_duplicate_shapes(shapes: &[Shaped]) -> Vec<Diag> {
    let mut diags: Vec<Diag> = Vec::new();
    let mut by_callee: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, s) in shapes.iter().enumerate() {
        for c in &s.profile.callees {
            by_callee.entry(c.clone()).or_default().push(i);
        }
    }
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for sharing in by_callee.values() {
        for x in 0..sharing.len() {
            for y in (x + 1)..sharing.len() {
                let (lo, hi) = (sharing[x].min(sharing[y]), sharing[x].max(sharing[y]));
                pairs.push((lo, hi));
            }
        }
    }
    pairs.sort();
    pairs.dedup();
    for (a, b) in pairs {
        {
            let (sa, sb) = (&shapes[a], &shapes[b]);
            let (pa, fa, la, na) = (&sa.profile, &sa.file, sa.line, &sa.name);
            let (pb, fb, lb, nb) = (&sb.profile, &sb.file, sb.line, &sb.name);
            if (sa.binding || sb.binding) && fa != fb {
                continue;
            }
            if pa.tokens.len().abs_diff(pb.tokens.len()) > MAX_DRIFT {
                continue;
            }
            if sa.dictated || sb.dictated {
                continue;
            }
            if names_each_other(sa, sb) {
                continue;
            }
            if !pa.composed || !pb.composed {
                continue;
            }
            let shortest = pa.tokens.len().min(pb.tokens.len());
            if shortest < MIN_SHAPE_TOKENS {
                continue;
            }
            let apart = shape::drift(pa, pb);
            if apart == 0 || apart > MAX_DRIFT {
                continue;
            }
            if !shape::only_names_differ(pa, pb) {
                continue;
            }
            diags.push(Diag::new(
                "E-NEAR-PATTERN",
                fb,
                lb,
                format!(
                    "{} is {} from {} at {}:{} ({}). Parameterise the difference",
                    nb,
                    tally(apart, "edit", "edits"),
                    na,
                    fa,
                    la,
                    shape::shown(&shape::differing(&pa.tokens, &pb.tokens))
                ),
            ));
        }
    }
    diags
}

fn report_here(root: &Path) -> Report {
    let files = rust_files(root);
    let shouty = shouty_fns(&files);
    let zones = config::Zones::load(root);
    roots::set(zones.library_roots());
    let library = names_in(root, zone::Zone::Patterns, public_fn_names);
    let spec_names = names_in(root, zone::Zone::Spec, type_names);
    let library_names = names_in(root, zone::Zone::Patterns, type_names);
    let values = declared_values(root);
    let (markers, marker_dups) = context_markers(root);
    let mut laundering = names_in(root, zone::Zone::Spec, laundering_names);
    laundering.extend(names_in(root, zone::Zone::Patterns, laundering_names));

    let mut exempt = 0usize;
    let mut checked = 0usize;
    let mut diags: Vec<Diag> = marker_dups;
    for gone in &zones.missing {
        let what = match zones.by_identity {
            true => format!("configured crate {} is not a workspace member", gone),
            false => format!("configured path {} does not exist", gone),
        };
        diags.push(Diag::new("E-ZONE-MISSING", "premise.zones", 0, what));
    }
    for (rel, keys) in cargo_configs(root) {
        diags.push(Diag::new(
            "E-CARGO-CONFIG",
            &rel,
            0,
            format!(
                "a cargo or toolchain file under the workspace sets {}, which changes what cargo runs or what compiles when the gate calls it, so a red tree could print green; delete the setting, since the gate is the only cargo configuration a governed workspace has",
                keys.join(", ")
            ),
        ));
    }
    for (rel, line) in manifest_comments(root) {
        diags.push(Diag::new(
            "E-COMMENT",
            &rel,
            line,
            "a comment or a prose field in a manifest; a manifest names crates and versions, and a line explaining one is a fact kept where no check reads it",
        ));
    }
    for key in &zones.unknown_keys {
        diags.push(Diag::new(
            "E-ZONE-KEY",
            "premise.zones",
            0,
            format!(
                "{} is not a zone, so the line does nothing and the crate it names is under no law; the zones are {}",
                key,
                zone::NAMES.join(", ")
            ),
        ));
    }
    let mut shapes: Vec<Shaped> = Vec::new();
    let mut bindings: Vec<(syn::ItemFn, String, usize)> = Vec::new();
    let mut vocabs: Vec<Vocab> = Vec::new();
    let mut exported: Vec<(String, String, usize)> = Vec::new();
    let mut cited_at: Vec<(String, String, usize)> = Vec::new();
    let mut sources_seen: Vec<(String, String, usize)> = Vec::new();
    let mut reasoned_items: Vec<(String, String, usize)> = Vec::new();
    let mut mentions: HashMap<String, usize> = HashMap::new();
    let mut cited_anywhere: std::collections::HashSet<String> = std::collections::HashSet::new();

    for path in &files {
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        if zones.outside_src(&rel) {
            diags.push(Diag::new(
                "E-OUTSIDE-SRC",
                &rel,
                0,
                "a spec, app or patterns crate holds src and tests only; cargo build never compiles an examples, benches or build script file, so it states the spec a second time where nothing checks it",
            ));
            continue;
        }
        let z = match zones.of(&rel) {

            Some(z) => z,
            None => {
                diags.push(Diag::new(
                    "E-ZONE-UNKNOWN",
                    &rel,
                    0,
                    "belongs to no zone; add its crate to the workspace members and to premise.zones",
                ));
                continue;
            }
        };
        checked += 1;
        if z == Zone::Tools {
            exempt += 1;
        }
        let r = z.rules();
        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                diags.push(Diag::new(
                    "E-PARSE",
                    &rel,
                    0,
                    format!("the file could not be read as UTF-8 text, so nothing in it was checked: {}", e),
                ));
                continue;
            }
        };

        if r.comment {
            for line in comment::find(&src) {
                diags.push(Diag::new("E-COMMENT", &rel, line, "comment token"));
            }
        }

        let parsed = match parsed::of(&src) {
            Ok(p) => p,
            Err(e) => {
                diags.push(Diag::new("E-PARSE", &rel, 0, e.to_string()));
                continue;
            }
        };

        let mut s = scan::Scan::new();
        s.check_calls = r.logic;
        s.allow_strings = r.const_literal;
        s.allow_lexical = r.lexical;
        s.allow_embed = z == Zone::Tools;
        s.tables_ok = r.no_fn;
        s.lints_named = z == Zone::Spec || z == Zone::App;
        s.digits_watched = z == Zone::Spec || z == Zone::Patterns;
        s.idents_watched = matches!(z, Zone::Spec | Zone::App | Zone::Patterns);
        s.shouty = shouty.clone();
        s.library_fns = library.clone();
        s.spec_types = spec_names.clone();
        s.library_types = library_names.clone();
        s.laundering = laundering.clone();
        scan::imports(&parsed, &mut s);
        s.visit_file(&parsed);

        if r.because {
            for (line, name) in &s.foreign_idents {
                diags.push(Diag::new(
                    "E-IDENT-SCRIPT",
                    &rel,
                    *line,
                    format!("{} carries a character outside ASCII; a name that looks like another name is the duplicate the name rules exist to catch, and no check can compare what it cannot read", name),
                ));
            }
        }
        if r.comment {
            for line in &s.docs {
                diags.push(Diag::new("E-COMMENT", &rel, *line, "a doc attribute or prose carried in an attribute"));
            }
            for (line, name) in &s.tool_lints {
                diags.push(Diag::new(
                    "E-COMMENT",
                    &rel,
                    *line,
                    format!("{} is a lint under a tool prefix, which the compiler accepts without reading the name, so the name may be prose; spec and app allow no tool lint", name),
                ));
            }
            for line in &s.underscored {
                diags.push(Diag::new(
                    "E-COMMENT",
                    &rel,
                    *line,
                    "a string bound by a let in spec, or to a name starting with _, or to a _ const, is read by nothing and explains something; that is a comment by another route, and a Named text returns its string from a match arm",
                ));
            }
            for line in &s.lifted_lints {
                diags.push(Diag::new(
                    "E-COMMENT",
                    &rel,
                    *line,
                    "an attribute names unknown_lints anywhere but the crate root's forbid, which would let a lint name carry prose past the compiler",
                ));
            }
            if s.lints_named && crate_root_with_manifest(root, &rel) && !forbids_unknown_lints(&parsed) {
                diags.push(Diag::new(
                    "E-COMMENT",
                    &rel,
                    0,
                    "the crate root does not forbid unknown_lints, so a lint attribute anywhere in the crate may carry prose the compiler accepts; write #![forbid(unknown_lints)] on the first line",
                ));
            }
        }
        for (line, said) in &s.fact_strings {
            let digits: String = said.chars().filter(|c| c.is_numeric()).collect();
            diags.push(Diag::new(
                "E-FACT-IN-REASON",
                &rel,
                *line,
                format!("{:?} carries {}, a value in prose that nothing checks and nothing keeps equal to the constant it restates; render it from the constant with digits, joined or label", said, digits),
            ));
        }
        for (line, name) in &s.fragment_idents {
            diags.push(Diag::new(
                "E-COMMENT",
                &rel,
                *line,
                format!("{} starts or ends with a word a name never does and a sentence fragment does; a sentence split across identifiers is still a comment, so say it in a because! and name the item by what it is", name),
            ));
        }
        for (line, name) in &s.prose_idents {
            diags.push(Diag::new(
                "E-COMMENT",
                &rel,
                *line,
                format!("{} is {} words long, and a name of more than {} is a sentence wearing an identifier; say the thing in a because! and name the item by what it is", name, scan::word_count(name), scan::MAX_IDENT_WORDS),
            ));
        }
        if r.no_fn {
            for line in s.stray_strings.iter().filter(|l| !s.underscored.contains(l)) {
                diags.push(Diag::new(
                    "E-INLINE-LITERAL",
                    &rel,
                    *line,
                    "a string in spec belongs in a trait impl that renders a named thing, as the whole value of a match arm or of the body; anywhere else it is a fact in prose or a comment",
                ));
            }
            for (line, name, leaves) in &s.literal_tables {
                diags.push(Diag::new(
                    "E-NO-BECAUSE",
                    &rel,
                    *line,
                    format!("{} carries {} literals under one reason; each fact names its own constant, so build the table from named consts", name, leaves),
                ));
            }
            if boundary::is_contexts_module(&rel) {
                for it in &parsed.items {
                    if !matches!(it, syn::Item::Mod(_)) {
                        let line = syn::spanned::Spanned::span(it).start().line;
                        diags.push(Diag::new(
                            "E-CONTEXT-LEAK",
                            &rel,
                            line,
                            "the contexts module declares its contexts and nothing else; any other item here is a bridge every context reaches with one glob",
                        ));
                    }
                }
            }
        }

        if r.literal || r.lexical {
            for line in &s.assembled {
                diags.push(Diag::new(
                    "E-ASSEMBLED-NUMBER",
                    &rel,
                    *line,
                    "a value built from literal arithmetic is still a magic number; name it",
                ));
            }
        }
        if r.because {
            for line in &s.repeated {
                diags.push(Diag::new(
                    "E-ASSEMBLED-NUMBER",
                    &rel,
                    *line,
                    "arithmetic that repeats one constant or cancels it against itself is a spelling of a small number; write the number as a named constant with a reason, or the multiple as a derivation with a named factor",
                ));
            }
        }

        if r.literal && !r.const_literal {
            for line in &s.type_aliases {
                diags.push(Diag::new(
                    "E-INLINE-LITERAL",
                    &rel,
                    *line,
                    "a type alias is a fact about a type, and facts belong in spec",
                ));
            }
        }
        if r.literal && !r.const_literal {
            for (line, name) in &s.primitive_consts {
                diags.push(Diag::new(
                    "E-INLINE-LITERAL",
                    &rel,
                    *line,
                    format!("{} is a value from nowhere, exactly as a literal would be; a bound belongs in spec", name),
                ));
            }
        }
        if r.logic {
            for (line, name) in &s.ambiguous_roots {
                diags.push(Diag::new(
                    "E-INLINE-LITERAL",
                    &rel,
                    *line,
                    format!("{} is reachable through more than one glob import, so the checker cannot tell which crate it belongs to; import it from spec by name and the doubt is gone", name),
                ));
            }
        }
        if r.literal && !r.const_literal {
            for (line, t) in &s.const_lits {
                diags.push(Diag::new(
                    "E-INLINE-LITERAL",
                    &rel,
                    *line,
                    format!("literal {} belongs in spec", t),
                ));
            }
        }

        if z == Zone::Spec {
            let mut checked: Vec<String> = s.check_refs.iter().cloned().collect();
            let mut at = 0usize;
            while at < checked.len() {
                let more: Vec<String> = s.const_refs.get(&checked[at]).cloned().unwrap_or_default();
                for name in more {
                    if !checked.contains(&name) {
                        checked.push(name);
                    }
                }
                at += 1;
            }
            for name in checked.iter().filter(|n| s.compound_consts.contains(*n)) {
                for (line, t) in s.named_lits.get(name).map(|v| v.as_slice()).unwrap_or(&[]) {
                    diags.push(Diag::new(
                        "E-INLINE-LITERAL",
                        &rel,
                        *line,
                        format!("literal {} inside {}, which a const assertion names, so it is part of a check that restates a value; a check compares named constants", t, name),
                    ));
                }
            }
        }
        if r.literal {
            for (line, t) in &s.lits {
                diags.push(Diag::new(
                    "E-INLINE-LITERAL",
                    &rel,
                    *line,
                    match z == Zone::Spec {
                        true => format!("literal {} outside a named const; a value in spec is a const carrying a reason, and a body or an assertion names the const", t),
                        false => format!("literal {} belongs in spec", t),
                    },
                ));
            }
        }

        if r.logic {
            for (line, k) in &s.logic {
                let note = match *k == "a match on a call" {
                    true => String::from("a match on a call belongs in a pattern; bind the call with let and match the binding"),
                    false => format!("{} belongs in a pattern", k),
                };
                diags.push(Diag::new("E-LOGIC-OUTSIDE-PATTERN", &rel, *line, note));
            }
            if let Some(line) = s.patterns_shadowed {
                diags.push(Diag::new(
                    "E-LOGIC-OUTSIDE-PATTERN",
                    &rel,
                    line,
                    "an item named after a crate root, patterns, patterns_macros, std, core or alloc, shadows that crate, so a path through the name reaches anything; only a leading :: reaches the real crate here",
                ));
            }
            if z == Zone::Spec {
                for (line, name) in &s.primitive_consts {
                    diags.push(Diag::new(
                        "E-INLINE-LITERAL",
                        &rel,
                        *line,
                        format!("{} in a function body is a value from nowhere; a bound is a const carrying a reason, and the body names the const", name),
                    ));
                }
            }
            for (line, name) in &s.foreign_macros {
                diags.push(Diag::new(
                    "E-LOGIC-OUTSIDE-PATTERN",
                    &rel,
                    *line,
                    format!(
                        "{}! is a macro, and a macro carries patterns, guards and expressions no check reads; only {} and the reason macros may be invoked here",
                        name,
                        scan::CALLABLE_MACROS.join("! ")
                    ),
                ));
            }
        }

        if r.no_fn && boundary::context_of(&rel).is_some() && !rel.contains("/contexts/shared/") {
            let inside = boundary::within_context(&rel);
            if !CONTEXT_FILES.contains(&inside.as_str()) {
                diags.push(Diag::new(
                    "E-CONTEXT-LEAK",
                    &rel,
                    0,
                    format!("a context holds {} and nothing else; {} is a file every other context could be handed through the vocabulary", CONTEXT_FILES.join(", "), inside),
                ));
            }
        }
        if r.no_fn {
            for (line, note) in boundary::leaks(&parsed, &rel, &markers) {
                diags.push(Diag::new("E-CONTEXT-LEAK", &rel, line, note));
            }
        }

        if r.no_fn {
            let ctx = boundary::context_of(&rel)
                .or_else(|| Some(zones.crate_of(&rel)).filter(|c| !c.is_empty()))
                .unwrap_or_else(|| String::from("the crate root"));
            let cited = because::collect(&parsed).cited;
            for (name, names, line) in because::enums(&parsed) {
                if names.len() < MIN_VARIANTS {
                    continue;
                }
                let excused = cited.contains(&name);
                vocabs.push(Vocab { names, ctx: ctx.clone(), file: rel.clone(), line, item: name, excused });
            }
        }

        if rel.ends_with("/contexts/shared.rs") {
            diags.push(Diag::new(
                "E-SHARED-SCOPE",
                &rel,
                0,
                "shared is a directory holding vocabulary.rs, not a file; a file here can hold anything and answers to no context",
            ));
        }
        if rel.contains("/contexts/shared/") {
            let leaf = rel.rsplit(config::SLASH).next().unwrap_or_default();
            let only_decls = parsed
                .items
                .iter()
                .all(|i| matches!(i, syn::Item::Mod(_) | syn::Item::Use(_)));
            let structural = leaf == "mod.rs" && only_decls;
            if leaf != "vocabulary.rs" && !structural {
                diags.push(Diag::new(
                    "E-SHARED-SCOPE",
                    &rel,
                    0,
                    "the shared tier holds vocabulary only, never state and never a reducer",
                ));
            }
            for (line, what) in shared_overreach(&parsed.items) {
                diags.push(Diag::new(
                    "E-SHARED-SCOPE",
                    &rel,
                    line,
                    format!("{} in the shared tier; state and reducers belong to a named context", what),
                ));
            }
        }

        if r.no_splice {
            for (line, what) in &s.splices {
                diags.push(Diag::new(
                    "E-SPLICE",
                    &rel,
                    *line,
                    what.clone(),
                ));
            }
        }

        if r.no_cfg {
            for (line, name) in &s.conditionals {
                diags.push(Diag::new(
                    "E-CONDITIONAL",
                    &rel,
                    *line,
                    name.clone(),
                ));
            }
        }

        for (line, name, shared) in &s.macros {
            let reachable = r.no_macro && (*shared || !r.impure);
            if reachable {
                diags.push(Diag::new(
                    "E-MACRO-DEF",
                    &rel,
                    *line,
                    format!(
                        "macro {} has a body no check can see; a caller inlines it",
                        name
                    ),
                ));
            }
        }

        if r.logic && !r.impure {
            let mut out_of_law = Vec::new();
            reexports(&parsed.items, &mut out_of_law);
            let local_mods: Vec<String> = parsed
                .items
                .iter()
                .filter_map(|it| match it {
                    syn::Item::Mod(m) => Some(crate::names::plain(&m.ident)),
                    _ => None,
                })
                .collect();
            for (line, name) in out_of_law {
                if name == "crate" || name == "self" || name == "super" || name == "std" || local_mods.contains(&name) {
                    continue;
                }
                if zones.zone_named(&name).is_some() {
                    continue;
                }
                diags.push(Diag::new(
                    "E-OUTSIDE-LAW",
                    &rel,
                    line,
                    format!(
                        "this re-exports {} into the spec, but {} is in no zone, so a fact enters the spec from outside every law and no check reads it",
                        name, name
                    ),
                ));
            }
        }

        if r.impure {
            let mut uses = Vec::new();
            project_uses(&parsed.items, &mut uses);
            for (line, name) in uses {
                diags.push(Diag::new(
                    "E-PATTERN-IMPURE",
                    &rel,
                    line,
                    format!("pattern names project crate {}", name),
                ));
            }
        }

        if r.because {
            let f = because::collect_with(&parsed, &values);
            if r.no_fn {
                for (name, line, _open) in &f.fns {
                    diags.push(Diag::new(
                        "E-SPEC-FN",
                        &rel,
                        *line,
                        format!("fn {} belongs in a pattern, spec holds no functions", name),
                    ));
                }
            }
            for (item, _reason, line) in &f.reasons {
                let wrote = f.written_as.get(item).cloned().unwrap_or_default();
                if !f.defined.contains(item) {
                    let why = match f.declared.contains(item) {
                        true => "is imported here, not declared; a reason lives beside its declaration",
                        false => "names nothing declared here",
                    };
                    diags.push(Diag::new(
                        "E-ORPHAN-REASON",
                        &rel,
                        *line,
                        format!("{}!({}) {}", wrote, item, why),
                    ));
                }
                let scope = boundary::context_of(&rel)
                    .unwrap_or_else(|| zones.crate_of(&rel));
                cited_at.push((format!("{}{}{}", scope, config::SLASH, item), rel.clone(), *line));
            }
            for (name, times) in &s.mentions {
                *mentions.entry(name.clone()).or_default() += times;
            }
            if r.no_fn {
                reasoned_in(&parsed.items, &f, &rel, &mut reasoned_items);
            }
            for source in &f.sources {
                if f.provisional.iter().any(|(p, _)| p == source) {
                    continue;
                }
                let at = f
                    .reasons
                    .iter()
                    .find(|(item, _, _)| item == source)
                    .map(|(_, _, line)| *line)
                    .unwrap_or(0);
                sources_seen.push((source.clone(), rel.clone(), at));
            }
            for (_, on, _) in &f.rests_on {
                cited_anywhere.extend(on.iter().cloned());
            }
            if r.citation {
                for (name, line, open) in &f.fns {
                    if !open || f.cited.iter().any(|c| c == name) {
                        continue;
                    }
                    diags.push(Diag::new(
                        "E-PATTERN-NO-REASON",
                        &rel,
                        *line,
                        format!(
                            "pattern {} states no reason for existing. A pattern is a decision, and E-NEAR-PATTERN excuses a pair only when each reason names the other",
                            name
                        ),
                    ));
                }
            }

            for (item, on, line) in &f.rests_on {
                let wrote = f.written_as.get(item).cloned().unwrap_or_default();
                for named in on {
                    if f.declared.contains(named) {
                        continue;
                    }
                    diags.push(Diag::new(
                        "E-UNDECLARED-SOURCE",
                        &rel,
                        *line,
                        format!(
                            "{}!({}) rests on {}, and nothing here declares it. What a reason leans on is a fact like any other: declare it with source! or name a type the vocabulary already has, so two reasons leaning on the same thing lean on one item rather than two sentences",
                            wrote, item, named
                        ),
                    ));
                }
                if !on.is_empty() || wrote == "provisional" {
                    continue;
                }
                let said = f
                    .reasons
                    .iter()
                    .find(|(who, _, at)| who == item && at == line)
                    .map(|(_, text, _)| text.clone())
                    .unwrap_or_default();
                if let Some(word) = because::leans_on(&said) {
                    diags.push(Diag::new(
                        "E-UNDECLARED-SOURCE",
                        &rel,
                        *line,
                        format!(
                            "{}!({}) names a {} in prose and cites nothing. The thing a reason leans on is an item: declare it with source! and cite it, so two reasons leaning on the same {} lean on one name",
                            wrote, item, word, word
                        ),
                    ));
                }
            }
            for (name, line) in &f.unit_sums {
                if s.repeated.contains(line) {
                    continue;
                }
                diags.push(Diag::new(
                    "E-ASSEMBLED-NUMBER",
                    &rel,
                    *line,
                    format!("{} is built from units, by a sum of them or a unit shifted by a named constant, which is a spelling of a count or a power; write the value as a named constant with a reason", name),
                ));
            }
            for (ty, tr, parts, text, line) in &f.decisions {
                let item = format!("{}_{}", ty, tr);
                if let Some(why) = because::weak(&item, text) {
                    diags.push(Diag::new("E-WEAK-REASON", &rel, *line, format!("decided!({}, {}) {}", ty, tr, why)));
                }
                if let Some(shown) = because::states_a_fact(text) {
                    diags.push(fact_in_reason(&rel, *line, "decided", &format!("{}, {}", ty, tr), DIGITS_WRITTEN, &shown));
                }
                if !f.declared.contains(ty) || !parts.iter().all(|p| f.declared.contains(p)) {
                    diags.push(Diag::new("E-ORPHAN-REASON", &rel, *line, format!("decided!({}, {}) names something not declared here", ty, tr)));
                } else if !f.impls.iter().any(|(t, on)| t == ty && on == tr) {
                    diags.push(Diag::new("E-ORPHAN-REASON", &rel, *line, format!("decided!({}, {}) is written away from the impl it covers; a reason lives beside its declaration", ty, tr)));
                }
                let scope = boundary::context_of(&rel).unwrap_or_else(|| zones.crate_of(&rel));
                cited_at.push((format!("{}{}{}_{}", scope, config::SLASH, ty, tr), rel.clone(), *line));
                if let Some(owners) = markers.get(ty) {
                    let here = boundary::context_of(&rel).unwrap_or_default();
                    if !owners.contains(&here) {
                        diags.push(Diag::new(
                            "E-CONTEXT-LEAK",
                            &rel,
                            *line,
                            format!("decided!({}, {}) decides a fact about {}, the marker {} runs on, from outside that context; a context carries its own decisions", ty, tr, ty, owners.join(" and ")),
                        ));
                    }
                }
            }
            for (name, line) in &s.laundering_aliases {
                if r.no_fn && !f.reasons.iter().any(|(item, _, _)| item == name) {
                    diags.push(Diag::new(
                        "E-NO-BECAUSE",
                        &rel,
                        *line,
                        format!("type {} renames a primitive or std type, which is a choice of range and unit that needs a reason; without one, {}::MAX is a value from nowhere wearing a spec name", name, name),
                    ));
                }
            }
            for said in &f.prose {
                if let Some(why) = because::weak(&said.item, &said.last) {
                    diags.push(Diag::new(
                        "E-WEAK-REASON",
                        &rel,
                        said.line,
                        format!("{}!({}) {}", said.wrote, said.item, why),
                    ));
                }
                if let Some(shown) = because::states_a_fact(&said.text) {
                    diags.push(fact_in_reason(&rel, said.line, &said.wrote, &said.item, DIGITS_WRITTEN, &shown));
                }
            }
            for (item, reason, line) in &f.reasons {
                let wrote = f.written_as.get(item).cloned().unwrap_or_default();
                if r.because {
                    if let Some(value) = f.values.get(&item.to_lowercase()) {
                        if let Some(word) = because::restates_value(reason, *value) {
                            diags.push(fact_in_reason(&rel, *line, &wrote, item, VALUE_SPELLED, &word));
                        }
                    }
                    if let Some(shown) = because::states_a_fact(reason) {
                        diags.push(fact_in_reason(&rel, *line, &wrote, item, DIGITS_WRITTEN, &shown));
                    }
                }
                let excusing = f.fns.iter().any(|(n, _, _)| n == item);
                let rests = f
                    .rests_on
                    .iter()
                    .any(|(who, on, _)| who == item && on.iter().any(|n| f.sources.contains(n)));
                let is_source = f.sources.contains(item);
                let is_value = f.value_items.contains(item);
                if r.citation && is_value && !excusing && !rests && !is_source {
                    diags.push(Diag::new(
                        "E-UNTRACEABLE",
                        &rel,
                        *line,
                        format!(
                            "because!({}) cites no source! item. A pattern constant answers to something outside this repo, so declare that thing with source! and cite it, rather than naming it in prose or citing a type that is not a source",
                            item
                        ),
                    ));
                }
                if let Some(why) = because::weak(item, reason) {
                    diags.push(Diag::new(
                        "E-WEAK-REASON",
                        &rel,
                        *line,
                        format!("{}!({}) {}", wrote, item, why),
                    ));
                }
            }
            for held in &f.named {
                if z != zone::Zone::Spec && !held.public {
                    continue;
                }
                let scope = match (z == zone::Zone::Spec, held.prefixed) {
                    (false, _) => zones.crate_of(&rel),
                    (true, true) => boundary::context_of(&rel)
                        .unwrap_or_else(|| zones.crate_of(&rel)),
                    (true, false) => String::new(),
                };
                exported.push((format!("{}{}{}", scope, config::SLASH, held.name), rel.clone(), held.line));
            }
            if z == zone::Zone::Spec {
                for source in &f.sources {
                    if f.provisional.iter().any(|(p, _)| p == source) {
                        continue;
                    }
                    let line = f
                        .reasons
                        .iter()
                        .find(|(item, _, _)| item == source)
                        .map(|(_, _, l)| *l)
                        .unwrap_or(0);
                    exported.push((format!("{}{}", config::SLASH, source), rel.clone(), line));
                }
            }
            for (name, line, shape) in &f.chosen {
                if r.lexical && shape == "char" {
                    continue;
                }
                if shape == "inherent" {
                    diags.push(Diag::new(
                        "E-NO-BECAUSE",
                        &rel,
                        *line,
                        format!(
                            "an inherent impl on {} carries literal consts, which no reason can cover one by one; make each a plain const with its own reason, or a trait impl that cites its trait",
                            name
                        ),
                    ));
                    continue;
                }
                if let Some(trait_of) = shape.strip_prefix("blanket:") {
                    if !f.cited.contains(&trait_of.to_string()) && !f.cited.contains(name) {
                        diags.push(Diag::new(
                            "E-NO-BECAUSE",
                            &rel,
                            *line,
                            format!("the blanket impl of {} is a choice with no because! on {}", trait_of, trait_of),
                        ));
                    }
                    continue;
                }
                let trait_of = shape.strip_prefix("impl:").unwrap_or_default();
                if !trait_of.is_empty() {
                    let decided = f
                        .decisions
                        .iter()
                        .any(|(ty, tr, _, _, _)| ty == name && tr == trait_of);
                    if !decided {
                        diags.push(Diag::new(
                            "E-NO-BECAUSE",
                            &rel,
                            *line,
                            format!(
                                "{} impl {} is a choice with no decided! of its own; a reason on {} alone or on {} alone covers nothing this impl decides, so write decided!({}, {}, ...)",
                                name, trait_of, name, trait_of, name, trait_of
                            ),
                        ));
                    }
                    continue;
                }
                if !f.cited.contains(name) {
                    diags.push(Diag::new(
                        "E-NO-BECAUSE",
                        &rel,
                        *line,
                        format!("{} is a choice with no because!", name),
                    ));
                }
            }
        }

        if z == zone::Zone::App {
            for it in &parsed.items {
                if let syn::Item::Fn(f) = it {
                    bindings.push((f.clone(), rel.clone(), f.sig.ident.span().start().line));
                }
            }
        }

        if r.dup {
            let said = because::collect(&parsed).reasons;
            let mut fns = Fns {
                items: Vec::new(),
                in_trait_impl: false,
            };
            fns.visit_file(&parsed);
            for (sig, block, line, dictated) in &fns.items {
                let name = crate::names::plain(&sig.ident);
                let reason = said
                    .iter()
                    .find(|(item, _, _)| item == &name)
                    .map(|(_, text, _)| text.clone())
                    .unwrap_or_default();
                shapes.push(Shaped {
                    profile: shape::profile(sig, block),
                    file: rel.clone(),
                    line: *line,
                    name,
                    reason,
                    dictated: *dictated,
                    binding: z == zone::Zone::App,
                });
            }
        }
    }

    let mut seen: HashMap<String, (String, usize)> = HashMap::new();
    for s in library_shapes(&zones) {
        seen.entry(s.profile.key()).or_insert((s.file.clone(), s.line));
    }
    for s in &shapes {
        let key = s.profile.key();
        match seen.get(&key) {
            Some((pf, pl)) => diags.push(Diag::new(
                "E-DUP-PATTERN",
                &s.file,
                s.line,
                format!("same shape as {}:{}", pf, pl),
            )),
            None => {
                seen.insert(key, (s.file.clone(), s.line));
            }
        }
    }

    let prepared: Vec<Binding> = bindings.iter().map(|(f, _, _)| Binding::of(f)).collect();
    for a in 0..bindings.len() {
        for b in (a + 1)..bindings.len() {
            let (fa, file_a, line_a) = &bindings[a];
            let (fb, file_b, line_b) = &bindings[b];
            if file_a != file_b {
                continue;
            }
            if let Some((was, now)) = same_but_for_constants(&prepared[a], &prepared[b]) {
                diags.push(Diag::new(
                    "E-CASE-LEAK",
                    file_b,
                    *line_b,
                    format!(
                        "{} and {} at {}:{} differ only at {} against {}, which are spec constants. A rule with several cases is one binding taking the case; enumerating them one binding apiece writes the values out again where app may not write values",
                        crate::names::plain(&fb.sig.ident),
                        crate::names::plain(&fa.sig.ident),
                        file_a,
                        line_a,
                        now,
                        was
                    ),
                ));
            }
        }
    }

    diags.extend(near_duplicate_shapes(&shapes));

    for (name, file, line) in &reasoned_items {
        let named = mentions.get(name).copied().unwrap_or_default();
        if named > DECLARATION_ITSELF {
            continue;
        }
        diags.push(Diag::new(
            "E-DEAD-SOURCE",
            file,
            *line,
            format!("{} carries a reason and is named nowhere but where it is declared; a fact nothing uses is a comment wearing a reason, so use it or delete it", name),
        ));
    }
    for (source, file, line) in &sources_seen {
        if cited_anywhere.contains(source) {
            continue;
        }
        diags.push(Diag::new(
            "E-DEAD-SOURCE",
            file,
            *line,
            format!(
                "source!({}) is cited by no reason anywhere. A source exists so that reasons lean on one item; one nothing leans on is either a citation somebody forgot or a thing already declared under another name",
                source
            ),
        ));
    }
    exported.sort();
    let mut reasons_for: Vec<(String, String, usize)> = Vec::new();
    for (item, file, line) in &cited_at {
        reasons_for.push((item.clone(), file.clone(), *line));
    }
    reasons_for.sort();
    for pair in reasons_for.windows(2) {
        let (first_item, first_file, first_line) = &pair[0];
        let (next_item, next_file, next_line) = &pair[1];
        if first_item == next_item {
            diags.push(Diag::new(
                "E-DUP-REASON",
                next_file,
                *next_line,
                format!(
                    "{} already has a reason at {}:{}; one item, one reason",
                    next_item.rsplit(config::SLASH).next().unwrap_or(next_item),
                    first_file,
                    first_line
                ),
            ));
        }
    }

    let mut by_item: HashMap<String, Vec<usize>> = HashMap::new();
    for (at, (name, _, _)) in exported.iter().enumerate() {
        let item = name.rsplit(config::SLASH).next().unwrap_or(name).to_string();
        by_item.entry(item).or_default().push(at);
    }
    let mut items: Vec<&String> = by_item.keys().collect();
    items.sort();
    for item in items {
        let same = &by_item[item];
        for x in 0..same.len() {
            for y in (x + 1)..same.len() {
                let (na, fa, la) = &exported[same[x]];
                let (nb, fb, lb) = &exported[same[y]];
                let scope_a = na.split_once(config::SLASH).map(|(s, _)| s).unwrap_or("");
                let scope_b = nb.split_once(config::SLASH).map(|(s, _)| s).unwrap_or("");
                let same_scope = scope_a == scope_b || scope_a.is_empty() || scope_b.is_empty();
                if !same_scope {
                    continue;
                }
                diags.push(Diag::new(
                    "E-DUP-NAME",
                    fb,
                    *lb,
                    format!("{} is also declared at {}:{}; one name, one fact", item, fa, la),
                ));
            }
        }
    }

    diags.extend(overlapping_vocabularies(&vocabs));

    let meta = cargo::read(root);
    if meta.is_none() && root.join("Cargo.toml").is_file() {
        diags.push(Diag::new(
            "E-TARGETS-UNRESOLVED",
            "Cargo.toml",
            0,
            "cargo metadata did not answer, so the checker fell back to reading the manifest itself; that fallback is a second opinion about which files are the program and it has been wrong before, so treat this run as unchecked rather than clean",
        ));
    }
    if let Some(m) = &meta {
        for skipped in &m.skipped {
            diags.push(Diag::new(
                "E-DEFAULT-MEMBERS",
                "Cargo.toml",
                0,
                format!(
                    "default-members leaves {} out of every gate step, type checking included, while leaving it on disk for the checker to read",
                    skipped
                ),
            ));
        }
    }

    for owner in &zones.crates {
        let governed = matches!(
            zones.zone_named(&owner.name),
            Some(zone::Zone::Spec) | Some(zone::Zone::App) | Some(zone::Zone::Patterns)
        );
        for other in &zones.crates {
            let head = format!("{}/", other.dir);
            if other.name != owner.name && owner.dir.starts_with(&head) {
                diags.push(Diag::new(
                    "E-CRATE-NESTED",
                    &format!("{}/Cargo.toml", owner.dir),
                    0,
                    format!(
                        "this crate sits inside {}, so a file under it answers to whichever crate has the longer path rather than to the one rustc compiles it into",
                        other.dir
                    ),
                ));
            }
        }
        if !governed {
            continue;
        }
        let home = root.join(&owner.dir);
        let src = modules::real(&home.join("src"));
        if let Some(pkg) = meta.as_ref().and_then(|m| m.of(&owner.name)) {
            for t in &pkg.targets {
                let links = t.is("lib") || t.is("rlib") || t.is("proc-macro") || t.is("bin");
                if !links || modules::real(&t.src).starts_with(&src) {
                    continue;
                }
                diags.push(Diag::new(
                    "E-OUTSIDE-SRC",
                    &format!("{}/Cargo.toml", owner.dir),
                    0,
                    format!(
                        "a compiled target resolves to {}, outside this crate's src, so the code it names is checked under another zone's laws or under none",
                        t.src.display()
                    ),
                ));
            }
        }
        let manifest =
            std::fs::read_to_string(home.join("Cargo.toml")).unwrap_or_default();
        for dep in manifest::path_deps(&manifest) {
            let where_ = modules::real(&home.join(&dep));
            if where_.starts_with(modules::real(root)) || zones.holds_library(&where_) {
                continue;
            }
            diags.push(Diag::new(
                "E-OUTSIDE-WORKSPACE",
                &format!("{}/Cargo.toml", owner.dir),
                0,
                format!(
                    "the path dependency {} resolves outside the workspace, where no zone reaches it, so its code answers to no law while this crate re-exports it",
                    dep
                ),
            ));
        }

    }

    for owner in &zones.crates {
        let home = root.join(&owner.dir);

        let tree = match meta.as_ref().and_then(|m| tree_of(m, &owner.name, false)) {
            Some(t) => t,
            None => continue,
        };
        let live_real: std::collections::HashSet<PathBuf> =
            tree.live.iter().map(|p| modules::real(p)).collect();
        if tree.live.is_empty() {
            continue;
        }
        for (path, name) in &tree.miscased {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            diags.push(Diag::new(
                "E-MODULE-CASE",
                &rel,
                0,
                format!(
                    "mod {} finds this file only because the filesystem ignores case; on a case sensitive one rustc would not compile it",
                    name
                ),
            ));
        }
        for f in &files {
            if !modules::real(f).starts_with(modules::real(&home.join("src"))) {
                continue;
            }
            if live_real.contains(&modules::real(f)) {
                continue;
            }
            let rel = f
                .strip_prefix(root)
                .unwrap_or(f)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            diags.push(Diag::new(
                "E-ORPHAN-FILE",
                &rel,
                0,
                "no module declares this file, so rustc never compiles it while the checker reads it and reports on it",
            ));
        }
    }

    for owner in &zones.crates {
        let zone = match zones.zone_named(&owner.name) {
            Some(z) => z,
            None => continue,
        };
        let at = format!("{}/Cargo.toml", owner.dir);
        for dep in &owner.deps {
            let theirs = zones.zone_named(dep);
            let upward = matches!(zone, Zone::Patterns | Zone::Tools)
                && matches!(theirs, Some(Zone::Spec) | Some(Zone::App));
            if upward {
                diags.push(Diag::new(
                    "E-ZONE-DEPENDS",
                    &at,
                    0,
                    format!("{} depends on {}; only app may", owner.name, dep),
                ));
            }
            let outward = matches!(zone, Zone::Spec | Zone::App)
                && theirs.is_none()
                && owner.runtime_deps.contains(dep);
            if outward {
                diags.push(Diag::new(
                    "E-ZONE-DEPENDS",
                    &at,
                    0,
                    format!(
                        "{} depends on {}, which is in no zone, so anything it re-exports, aliases or wraps is a fact from outside every law",
                        owner.name, dep
                    ),
                ));
            }
        }
    }

    diags.sort_by(|a, b| (&a.file, a.line, a.code).cmp(&(&b.file, b.line, b.code)));
    Report {
        diags,
        exempt,
        checked,
    }
}

pub fn tree_of(meta: &cargo::Meta, name: &str, linked_only: bool) -> Option<modules::Tree> {
    let pkg = meta.of(name)?;
    let roots: Vec<PathBuf> = pkg
        .targets
        .iter()
        .filter(|t| if linked_only { t.linked() } else { t.compiled() })
        .map(|t| t.src.clone())
        .collect();
    Some(modules::from_roots(&roots))
}

pub fn compiled_sources(root: &Path, want: zone::Zone) -> Vec<PathBuf> {
    let zones = config::Zones::load(root);
    let meta = cargo::read(root);
    let mut out: Vec<PathBuf> = Vec::new();
    for owner in &zones.crates {
        if zones.zone_named(&owner.name) != Some(want) {
            continue;
        }
        let is_macro_crate = meta
            .as_ref()
            .and_then(|m| m.of(&owner.name))
            .map(|p| p.targets.iter().any(|t| t.is("proc-macro")))
            .unwrap_or(false);
        if is_macro_crate {
            continue;
        }
        if let Some(tree) = meta.as_ref().and_then(|m| tree_of(m, &owner.name, true)) {
            out.extend(tree.reachable);
        }
    }
    if out.is_empty() {
        for f in rust_files(root) {
            let rel = f
                .strip_prefix(root)
                .unwrap_or(&f)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            if zones.of(&rel) == Some(want) {
                out.push(f);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

pub const ENVIRONMENT: &[&str] = &["E-TARGETS-UNRESOLVED"];
because!(
    ENVIRONMENT,
    "the codes that report a tool the checker could not reach rather than a law the workspace broke, so a reader of a failing log knows which of the two they are looking at"
);

pub fn explain(code: &str) -> String {
    let manual = include_str!("../../../README.md");
    let head = format!("| `{}` |", code);
    let mut row = String::new();
    for line in manual.lines() {
        if line.starts_with(&head) {
            row = line
                .trim_start_matches(&head)
                .trim_end_matches('|')
                .trim()
                .to_string();
        }
    }
    if row.is_empty() {
        return format!("no check named {} is documented", code);
    }
    let mut out = vec![row];
    let mut section = String::new();
    let mut keep = false;
    for line in manual.lines() {
        if line.starts_with("## ") {
            keep = false;
            section = line.trim_start_matches("## ").trim_start_matches(|c: char| !c.is_ascii_alphanumeric()).to_string();
        }
        if line.starts_with("| ") || line.starts_with("|-") {
            continue;
        }
        if line.contains(code) && !section.is_empty() {
            keep = true;
            out.push(format!("\nfrom \"{}\":", section));
        }
        if keep && !line.trim().is_empty() {
            out.push(line.to_string());
        }
        if keep && line.trim().is_empty() && out.len() > 2 {
            keep = false;
        }
    }
    out.join("\n")
}

pub fn patterns_seen(root: &Path) -> usize {
    let mut n = 0usize;
    for f in compiled_sources(root, zone::Zone::Patterns) {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        if let Ok(parsed) = parsed::of(&text) {
            n += public_fns(&parsed.items);
        }
    }
    n
}

fn public_fns(items: &[syn::Item]) -> usize {
    let mut n = 0usize;
    for it in items {
        match it {
            syn::Item::Fn(f) if facts::is_public(&f.vis) => n += 1,
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    n += public_fns(inner);
                }
            }
            _ => {}
        }
    }
    n
}

pub const SHIPPED: &[&str] = &["premise_checker", "premise_gate"];
because!(
    SHIPPED,
    "the toolchain crates an adopter copies in; they grow when the tools improve, and that growth is not the signal the census is watching for"
);

pub struct Census {
    pub governed: usize,
    pub escaped: usize,
    pub patterns: usize,
    pub provisional: usize,
    pub untested: usize,
}

fn lines_in(files: &[PathBuf]) -> usize {
    let mut n = 0usize;
    for f in files {
        n += std::fs::read_to_string(f).unwrap_or_default().lines().count();
    }
    n
}

pub fn census(root: &Path) -> Census {
    let here = root.to_path_buf();
    on_big_stack(move || census_here(&here))
}

fn census_here(root: &Path) -> Census {
    let zones = config::Zones::load(root);
    let mut governed = Vec::new();
    let mut escaped = Vec::new();
    let mut ungoverned = Vec::new();
    for f in rust_files(root) {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(&f)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        match zones.of(&rel) {
            Some(zone::Zone::Spec) | Some(zone::Zone::App) | Some(zone::Zone::Patterns) => {
                governed.push(f)
            }
            Some(zone::Zone::Tests) => ungoverned.push(f),
            Some(zone::Zone::Tools) => {
                let owned = zones
                    .crates
                    .iter()
                    .find(|c| rel.starts_with(&format!("{}/", c.dir)));
                let shipped = owned
                    .map(|c| SHIPPED.contains(&c.name.as_str()))
                    .unwrap_or(false);
                if !shipped {
                    escaped.push(f);
                }
            }
            _ => {}
        }
    }
    Census {
        governed: lines_in(&governed),
        escaped: lines_in(&escaped),
        patterns: patterns_seen(root),
        provisional: undecided(root),
        untested: lines_in(&ungoverned),
    }
}

pub fn undecided(root: &Path) -> usize {
    let mut n = 0usize;
    for f in compiled_sources(root, zone::Zone::Spec) {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        if let Ok(parsed) = parsed::of(&text) {
            n += because::collect(&parsed).provisional.len();
        }
    }
    n
}

pub fn module_path(root: &Path, file: &Path) -> String {
    module_path_in(&config::Zones::load(root), root, file)
}

pub fn module_path_in(zones: &config::Zones, root: &Path, file: &Path) -> String {
    let rel = file
        .strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/");
    let owner = zones.crate_of(&rel);
    let inside = rel
        .split_once("/src/")
        .map(|(_, tail)| tail)
        .unwrap_or(rel.as_str())
        .trim_end_matches(".rs");
    let mut parts: Vec<&str> = inside.split('/').collect();
    if matches!(parts.last(), Some(&"lib") | Some(&"mod") | Some(&"main")) {
        parts.pop();
    }
    let head = match owner.is_empty() {
        true => String::from("patterns"),
        false => owner,
    };
    std::iter::once(head.as_str())
        .chain(parts)
        .collect::<Vec<&str>>()
        .join("::")
}

pub const CENSUS_FILE: &str = "premise.census";
because!(
    CENSUS_FILE,
    "the one tracked line the checker and the gate both rewrite, named once so the two cannot drift apart about where the count lives"
);

fn crate_root_with_manifest(root: &Path, rel: &str) -> bool {
    let stem = match rel.strip_suffix("/src/lib.rs").or_else(|| rel.strip_suffix("/src/main.rs")) {
        Some(s) => s,
        None => return false,
    };
    root.join(stem).join("Cargo.toml").is_file()
}

fn forbids_unknown_lints(file: &syn::File) -> bool {
    file.attrs.iter().any(|a| {
        matches!(a.style, syn::AttrStyle::Inner(_))
            && a.path().is_ident("forbid")
            && a.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .map(|list| list.iter().any(|m| m.path().is_ident("unknown_lints")))
                .unwrap_or(false)
    })
}

pub fn refresh_census(root: &Path) -> Result<Option<(String, String)>, String> {
    let c = census(root);
    let now = format!(
        "{} pattern functions, {} governed lines, {} escaped lines, {} test lines, {} provisional",
        c.patterns, c.governed, c.escaped, c.untested, c.provisional
    );
    let file = root.join(CENSUS_FILE);
    let was = std::fs::read_to_string(&file)
        .unwrap_or_default()
        .trim()
        .to_string();
    if let Err(e) = std::fs::write(&file, format!("{}\n", now)) {
        return Err(format!("{} could not be written: {}", CENSUS_FILE, e));
    }
    match !was.is_empty() && was != now {
        true => Ok(Some((was, now))),
        false => Ok(None),
    }
}

#[cfg(test)]
mod messages {
    #[test]
    fn every_template_names_its_hole() {
        for said in [super::DIGITS_WRITTEN, super::VALUE_SPELLED] {
            assert_eq!(said.matches(super::HOLE).count(), 1, "{}", said);
            assert!(said.replace(super::HOLE, "digits").contains("digits"));
        }
    }
}
