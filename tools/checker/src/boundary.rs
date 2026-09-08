use quote::ToTokens;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use patterns_macros::because;

pub const SHARED: &str = "shared";
because!(
    SHARED,
    "the one context every other context may reach into, so a fact two of them need has a home"
);

const BEYOND_VOCABULARY: usize = 4;
because!(
    BEYOND_VOCABULARY,
    "the distance from the contexts segment to the first segment past a vocabulary item, so a reach that goes deeper than an item in the vocabulary is judged on the module it goes through"
);

const SRC_MARKER: &str = "/src/";
because!(
    SRC_MARKER,
    "the cargo layout boundary between a crate root and the module tree beneath it"
);

pub const CONTEXT_DRIVERS: &[&str] = &["handle", "handle_all", "replay", "replay_from"];
because!(
    CONTEXT_DRIVERS,
    "the four library functions that run a context's reducer from its marker, so a turbofish naming another context's marker on one of them drives that context from outside it"
);

pub const OUTSIDE_ROOTS: &[&str] = &["std", "core", "alloc", "patterns", "patterns_macros"];
because!(
    OUTSIDE_ROOTS,
    "the crate roots a path in spec may start from besides its own tree, so a path that resolves to none of them and to no context is an unresolved name, which no context owns"
);

fn segments(rel: &str) -> Vec<String> {
    let cut = match rel.find(SRC_MARKER) {
        Some(at) => &rel[at + SRC_MARKER.len()..],
        None => rel,
    };
    let mut out: Vec<String> = cut.split('/').map(|s| s.to_string()).collect();
    if let Some(last) = out.pop() {
        let stem = last.trim_end_matches(".rs").to_string();
        if stem != "mod" && stem != "lib" && stem != "main" {
            out.push(stem);
        }
    }
    out
}

pub fn context_of(rel: &str) -> Option<String> {
    let mods = segments(rel);
    for (i, m) in mods.iter().enumerate() {
        if m == "contexts" && i + 1 < mods.len() {
            return Some(mods[i + 1].clone());
        }
    }
    None
}

fn tree_paths(t: &syn::UseTree, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
    match t {
        syn::UseTree::Path(p) => {
            let mut next = prefix;
            next.push(crate::names::plain(&p.ident));
            tree_paths(&p.tree, next, out);
        }
        syn::UseTree::Name(n) => {
            let mut v = prefix;
            v.push(crate::names::plain(&n.ident));
            out.push(v);
        }
        syn::UseTree::Rename(r) => {
            let mut v = prefix;
            v.push(crate::names::plain(&r.ident));
            out.push(v);
        }
        syn::UseTree::Group(g) => {
            for i in &g.items {
                tree_paths(i, prefix.clone(), out);
            }
        }
        syn::UseTree::Glob(_) => out.push(prefix),
    }
}

fn tree_globs(t: &syn::UseTree, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
    match t {
        syn::UseTree::Path(p) => {
            let mut next = prefix;
            next.push(crate::names::plain(&p.ident));
            tree_globs(&p.tree, next, out);
        }
        syn::UseTree::Group(g) => {
            for i in &g.items {
                tree_globs(i, prefix.clone(), out);
            }
        }
        syn::UseTree::Glob(_) => out.push(prefix),
        _ => {}
    }
}

fn resolve(segs: &[String], here: &[String]) -> Vec<String> {
    let mut ups = 0usize;
    let mut rest: Vec<String> = Vec::new();
    let mut started = false;
    let mut from_here = false;
    for s in segs {
        if !started && s == "super" {
            ups += 1;
            continue;
        }
        if !started && s == "self" {
            started = true;
            from_here = true;
            continue;
        }
        if !started && s == "crate" {
            started = true;
            continue;
        }
        started = true;
        rest.push(s.clone());
    }
    if ups == 0 && !from_here {
        return rest;
    }
    let keep = here.len().saturating_sub(ups);
    let mut out: Vec<String> = here[..keep].to_vec();
    out.extend(rest);
    out
}

fn reach(p: &[String]) -> Option<(String, Option<String>)> {
    for (i, seg) in p.iter().enumerate() {
        if seg == "contexts" && i + 1 < p.len() {
            let part = p.get(i + 2).cloned();
            let module_like = p
                .get(i + BEYOND_VOCABULARY - 1)
                .and_then(|s| s.chars().next())
                .map(|c| c.is_lowercase())
                .unwrap_or(false);
            let deeper = part.as_deref() == Some("vocabulary") && p.get(i + BEYOND_VOCABULARY).is_some() && module_like;
            return match deeper {
                true => Some((p[i + 1].clone(), p.get(i + BEYOND_VOCABULARY - 1).cloned())),
                false => Some((p[i + 1].clone(), part)),
            };
        }
    }
    None
}

fn renames(t: &syn::UseTree, prefix: Vec<String>, out: &mut Vec<(String, Vec<String>)>) {
    match t {
        syn::UseTree::Path(p) => {
            let mut next = prefix;
            next.push(crate::names::plain(&p.ident));
            renames(&p.tree, next, out);
        }
        syn::UseTree::Rename(r) => {
            let mut v = prefix;
            v.push(crate::names::plain(&r.ident));
            out.push((crate::names::plain(&r.rename), v));
        }
        syn::UseTree::Group(g) => {
            for i in &g.items {
                renames(i, prefix.clone(), out);
            }
        }
        _ => {}
    }
}

fn named_leaves(t: &syn::UseTree, prefix: Vec<String>, out: &mut Vec<(String, Vec<String>)>) {
    match t {
        syn::UseTree::Path(p) => {
            let mut next = prefix;
            next.push(crate::names::plain(&p.ident));
            named_leaves(&p.tree, next, out);
        }
        syn::UseTree::Name(n) => {
            let mut v = prefix;
            v.push(crate::names::plain(&n.ident));
            out.push((crate::names::plain(&n.ident), v));
        }
        syn::UseTree::Rename(r) => {
            let mut v = prefix;
            v.push(crate::names::plain(&r.ident));
            out.push((crate::names::plain(&r.rename), v));
        }
        syn::UseTree::Group(g) => {
            for i in &g.items {
                named_leaves(i, prefix.clone(), out);
            }
        }
        _ => {}
    }
}

type Bound = (String, Vec<String>);

fn collected(file: &syn::File, here: &[String], leaves: fn(&syn::UseTree, Vec<String>, &mut Vec<Bound>)) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    for it in &file.items {
        if let syn::Item::Use(u) = it {
            leaves(&u.tree, Vec::new(), &mut out);
        }
    }
    out.iter()
        .map(|(name, segs)| (name.clone(), resolve(segs, here)))
        .collect()
}

struct Reach<'a> {
    here: &'a str,
    mods: Vec<String>,
    known: Vec<(String, Vec<String>)>,
    vocabulary: bool,
    in_pub_item: bool,
    local_mods: Vec<String>,
    globbed: Vec<String>,
    imported: Vec<(String, Vec<String>)>,
    local_types: Vec<(String, Vec<String>)>,
    local_decls: Vec<String>,
    generic_params: Vec<String>,
    markers: &'a HashMap<String, Vec<String>>,
    out: Vec<(usize, String)>,
}

impl<'a> Reach<'a> {
    fn expand(&self, segs: &[String]) -> Vec<String> {
        let mut out = segs.to_vec();
        let mut hops = 0usize;
        while hops <= self.known.len() {
            let head = match out.first() {
                Some(h) => h.clone(),
                None => return out,
            };
            let target = match self.known.iter().find(|(name, _)| name == &head) {
                Some((_, full)) if full.first() != Some(&head) => full.clone(),
                _ => return out,
            };
            let mut next = target;
            next.extend(out.iter().skip(1).cloned());
            out = next;
            hops += 1;
        }
        out
    }

    fn judge(&mut self, segs: &[String], line: usize) {
        let full = self.expand(&resolve(segs, &self.mods));
        let (other, part) = match reach(&full) {
            Some(v) => v,
            None => return self.judge_bare(&full, line),
        };
        if other == self.here {
            let own_state = part.is_some() && part.as_deref() != Some("vocabulary");
            if self.vocabulary && self.in_pub_item && own_state {
                self.out.push((
                    line,
                    format!(
                        "{} names {} in a public item of its vocabulary, which lets every other context reach it",
                        self.here,
                        full.join("::")
                    ),
                ));
            }
            return;
        }
        if part.as_deref() == Some("vocabulary") {
            return;
        }
        let who = if self.here.is_empty() {
            String::from("a module outside any context")
        } else {
            self.here.to_string()
        };
        self.out.push((
            line,
            format!("{} reaches into {}", who, full.join("::")),
        ));
    }
}

impl<'a> Reach<'a> {
    fn foreign_marker(&mut self, name: &str, line: usize) {
        let owners = match self.markers.get(name) {
            Some(o) if !o.iter().any(|c| c == self.here) => o,
            _ => return,
        };
        self.out.push((
            line,
            format!(
                "{} names {}, the marker {} runs on; a marker is named in its own context and in app, and nowhere else in spec, so no alias, bound, projection or re-export can carry that context's state here",
                self.here,
                name,
                owners.join(" and ")
            ),
        ));
    }

    fn resolved(&self, ty: &syn::Type) -> Option<Vec<String>> {
        let tp = match ty {
            syn::Type::Path(tp) if tp.qself.is_none() => tp,
            _ => return None,
        };
        let mut full: Vec<String> = tp.path.segments.iter().map(|s| crate::names::plain(&s.ident)).collect();
        let mut hops = 0usize;
        while full.len() == 1 && hops <= self.local_types.len() + self.imported.len() {
            let next = self
                .local_types
                .iter()
                .chain(self.imported.iter())
                .find(|(name, _)| name == &full[0])
                .map(|(_, target)| target.clone())?;
            full = next;
            hops += 1;
        }
        Some(self.expand(&resolve(&full, &self.mods)))
    }

    fn owned(&self, ty: &syn::Type) -> bool {
        if let syn::Type::Path(tp) = ty {
            if tp.qself.is_some() {
                return false;
            }
            if tp.path.segments.len() == 1 && tp.path.get_ident().map(|id| self.local_decls.contains(&crate::names::plain(id))).unwrap_or(false) {
                return true;
            }
        }
        let full = match self.resolved(ty) {
            Some(f) => f,
            None => {
                let single = match ty {
                    syn::Type::Path(tp) => tp.path.segments.len() == 1,
                    _ => false,
                };
                return single && self.globbed.iter().any(|g| g == SHARED);
            }
        };
        match reach(&full) {
            Some((ctx, _)) => ctx == self.here || ctx == SHARED,
            None => full
                .first()
                .map(|root| (OUTSIDE_ROOTS.contains(&root.as_str()) || crate::roots::is(root)) && !self.local_mods.contains(root))
                .unwrap_or(false),
        }
    }

    fn driver_call(&self, p: &syn::Path) -> Option<String> {
        let segs: Vec<String> = p.segments.iter().map(|s| crate::names::plain(&s.ident)).collect();
        let last = segs.last()?;
        if !CONTEXT_DRIVERS.contains(&last.as_str()) {
            return None;
        }
        let from_library = match segs.len() {
            1 => self
                .imported
                .iter()
                .any(|(name, full)| name == last && full.first().map(|r| crate::roots::is(r)).unwrap_or(false)),
            _ => crate::roots::is(&segs[0]),
        };
        match from_library {
            true => Some(last.clone()),
            false => None,
        }
    }

    fn other_context_of(&self, ty: &syn::Type) -> Option<String> {
        let full = self.resolved(ty)?;
        let (ctx, _) = reach(&full)?;
        match ctx != self.here && ctx != SHARED {
            true => Some(ctx),
            false => None,
        }
    }

    fn judge_bare(&mut self, full: &[String], line: usize) {
        let root = match full.first() {
            Some(r) if full.len() > 1 => r,
            _ => return,
        };
        let module = root.as_str();
        let guarded = crate::CONTEXT_FILES
            .iter()
            .any(|f| f.trim_end_matches(".rs") == module && module != "vocabulary" && module != "mod");
        if !guarded || self.local_mods.iter().any(|m| m == module) {
            return;
        }
        if self.vocabulary && self.in_pub_item {
            self.out.push((
                line,
                format!(
                    "{} names {} in a public item of its vocabulary, which lets every other context reach it",
                    self.here,
                    full.join("::")
                ),
            ));
            return;
        }
        if let Some(other) = self.globbed.first() {
            self.out.push((
                line,
                format!(
                    "{} reaches {} through a glob of {}'s vocabulary, which is a module handed over and not an item",
                    self.here,
                    full.join("::"),
                    other
                ),
            ));
        }
    }
}

impl<'ast, 'a> Visit<'ast> for Reach<'a> {
    fn visit_item_use(&mut self, u: &'ast syn::ItemUse) {
        let mut found = Vec::new();
        tree_paths(&u.tree, Vec::new(), &mut found);
        let line = u.span().start().line;
        let mut globs = Vec::new();
        tree_globs(&u.tree, Vec::new(), &mut globs);
        let public = matches!(u.vis, syn::Visibility::Public(_));
        for g in &globs {
            let full = self.expand(&resolve(g, &self.mods));
            if full.last().map(|s| s == "contexts").unwrap_or(false) {
                self.out.push((
                    line,
                    format!(
                        "{} imports {}::*, which hands over every context module at once, so any bridge among them is reached",
                        self.here,
                        full.join("::")
                    ),
                ));
            }
            if let Some((other, part)) = reach(&full) {
                if other != self.here && other != SHARED && part.as_deref() == Some("vocabulary") {
                    self.out.push((
                        line,
                        format!(
                            "{} imports {}::*, which hands over every item {} exports now or later; import each item by name, or the module under a name",
                            self.here,
                            full.join("::"),
                            other
                        ),
                    ));
                }
                if other != self.here {
                    self.globbed.push(other);
                }
            }
            if self.vocabulary && public {
                self.out.push((
                    line,
                    format!(
                        "{} re-exports {}::* through its vocabulary; a vocabulary names each item it exports, since a glob hands over whatever the module holds now or later",
                        self.here,
                        full.join("::")
                    ),
                ));
            }
        }
        for p in &found {
            self.judge(p, line);
            if let Some(last) = p.last() {
                self.foreign_marker(last, line);
            }
            if !matches!(u.vis, syn::Visibility::Public(_)) {
                continue;
            }
            let full = self.expand(&resolve(p, &self.mods));
            let own = reach(&full)
                .map(|(ctx, part)| ctx == self.here && part.as_deref() != Some("vocabulary"))
                .unwrap_or(false);
            if own {
                self.out.push((
                    line,
                    format!(
                        "{} re-exports {} through its vocabulary, which lets every other context reach it",
                        self.here,
                        full.join("::")
                    ),
                ));
            }
        }
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        let open = |v: &syn::Visibility| !matches!(v, syn::Visibility::Inherited);
        let public = match i {
            syn::Item::Type(t) => open(&t.vis),
            syn::Item::Struct(s) => open(&s.vis),
            syn::Item::Enum(e) => open(&e.vis),
            syn::Item::Trait(t) => open(&t.vis),
            syn::Item::Fn(f) => open(&f.vis),
            syn::Item::Const(c) => open(&c.vis),
            syn::Item::Static(s) => open(&s.vis),
            syn::Item::Impl(_) => true,
            _ => false,
        };
        let prev = self.in_pub_item;
        self.in_pub_item = public;
        let depth = self.generic_params.len();
        self.generic_params.extend(generics_of(i));
        visit::visit_item(self, i);
        self.generic_params.truncate(depth);
        self.in_pub_item = prev;
    }

    fn visit_path(&mut self, p: &'ast syn::Path) {
        let segs: Vec<String> = p.segments.iter().map(|s| crate::names::plain(&s.ident)).collect();
        let line = p.span().start().line;
        self.judge(&segs, line);
        if let Some(last) = segs.last() {
            self.foreign_marker(last, line);
        }
        if segs.len() > 1 && self.generic_params.contains(&segs[0]) {
            self.out.push((
                line,
                format!(
                    "{} projects through the type parameter {}, as {}; whatever type is passed, its associated items are reached without a path, so a context declares fields by named types only",
                    self.here,
                    segs[0],
                    segs.join("::")
                ),
            ));
        }
        visit::visit_path(self, p);
    }

    fn visit_expr_path(&mut self, e: &'ast syn::ExprPath) {
        if let Some(driver) = self.driver_call(&e.path) {
            self.out.push((
                e.span().start().line,
                format!(
                    "{} calls {}, which runs a reducer; spec declares contexts and app drives them, so no context runs another from inside spec",
                    self.here,
                    driver
                ),
            ));
        }
        visit::visit_expr_path(self, e);
    }

    fn visit_generics(&mut self, g: &'ast syn::Generics) {
        let bounds = g.type_params().flat_map(|p| p.bounds.iter());
        let where_bounds = g
            .where_clause
            .iter()
            .flat_map(|w| w.predicates.iter())
            .filter_map(|p| match p {
                syn::WherePredicate::Type(t) => Some(t.bounds.iter()),
                _ => None,
            })
            .flatten();
        for b in bounds.chain(where_bounds) {
            if let syn::TypeParamBound::Trait(t) = b {
                let last = t.path.segments.last().map(|s| crate::names::plain(&s.ident)).unwrap_or_default();
                if last == "Context" {
                    self.out.push((
                        t.path.span().start().line,
                        format!(
                            "{} bounds a type parameter by Context, which projects any context's state through it; a context names its own marker and reaches no other",
                            self.here
                        ),
                    ));
                }
            }
        }
        visit::visit_generics(self, g);
    }

    fn visit_item_type(&mut self, t: &'ast syn::ItemType) {
        let public = !matches!(t.vis, syn::Visibility::Inherited);
        if public {
            if let Some(other) = self.other_context_of(&t.ty) {
                self.out.push((
                    t.span().start().line,
                    format!(
                        "{} aliases {}'s {} as {}, a second name under which every context reaches it; a context names its own items and reaches another's through that context's vocabulary only",
                        self.here,
                        other,
                        t.ty.to_token_stream().to_string().replace(' ', ""),
                        crate::names::plain(&t.ident)
                    ),
                ));
            }
        }
        visit::visit_item_type(self, t);
    }

    fn visit_qself(&mut self, q: &'ast syn::QSelf) {
        if !self.owned(&q.ty) {
            self.out.push((
                q.ty.span().start().line,
                format!(
                    "{} projects through <{} as ...>::, a type it does not own; a projection reaches a type by what it implements rather than by its path, so only a context's own types and shared's stand there, named by path",
                    self.here,
                    q.ty.to_token_stream().to_string().replace(" :: ", "::").replace("< ", "<").replace(" >", ">")
                ),
            ));
        }
        visit::visit_qself(self, q);
    }
}

pub fn leaks(file: &syn::File, rel: &str, markers: &HashMap<String, Vec<String>>) -> Vec<(usize, String)> {
    let here = context_of(rel).unwrap_or_default();
    let mut nested = Vec::new();
    if rel.contains("/vocabulary/") {
        nested.push((0, format!("{} keeps a module under its vocabulary, which every other context may reach as vocabulary", here)));
    }
    if rel.ends_with("/vocabulary.rs") {
        for it in &file.items {
            if let syn::Item::Mod(m) = it {
                nested.push((
                    m.span().start().line,
                    format!("{} declares a module inside its vocabulary, which every other context may reach as vocabulary", here),
                ));
            }
        }
    }
    let mods = segments(rel);
    let known = collected(file, &mods, renames);
    let imported = collected(file, &mods, named_leaves);
    let local_decls: Vec<String> = file
        .items
        .iter()
        .filter_map(|it| match it {
            syn::Item::Struct(s) => Some(crate::names::plain(&s.ident)),
            syn::Item::Enum(e) => Some(crate::names::plain(&e.ident)),
            syn::Item::Trait(t) => Some(crate::names::plain(&t.ident)),
            syn::Item::Union(u) => Some(crate::names::plain(&u.ident)),
            _ => None,
        })
        .collect();
    let local_types: Vec<(String, Vec<String>)> = file
        .items
        .iter()
        .filter_map(|it| match it {
            syn::Item::Type(t) => match &*t.ty {
                syn::Type::Path(tp) => Some((
                    crate::names::plain(&t.ident),
                    resolve(&tp.path.segments.iter().map(|s| crate::names::plain(&s.ident)).collect::<Vec<String>>(), &mods),
                )),
                _ => None,
            },
            _ => None,
        })
        .collect();
    let local_mods = file
        .items
        .iter()
        .filter_map(|it| match it {
            syn::Item::Mod(m) => Some(crate::names::plain(&m.ident)),
            _ => None,
        })
        .collect();
    let mut r = Reach {
        here: &here,
        mods,
        known,
        vocabulary: rel.ends_with("/vocabulary.rs"),
        in_pub_item: false,
        local_mods,
        globbed: Vec::new(),
        imported,
        local_types,
        local_decls,
        generic_params: Vec::new(),
        markers,
        out: Vec::new(),
    };
    r.visit_file(file);
    nested.extend(r.out);
    nested
}

fn generics_of(i: &syn::Item) -> Vec<String> {
    let g = match i {
        syn::Item::Struct(s) => &s.generics,
        syn::Item::Enum(e) => &e.generics,
        syn::Item::Trait(t) => &t.generics,
        syn::Item::Type(t) => &t.generics,
        syn::Item::Fn(f) => &f.sig.generics,
        syn::Item::Impl(im) => &im.generics,
        syn::Item::Union(u) => &u.generics,
        _ => return Vec::new(),
    };
    g.type_params().map(|p| crate::names::plain(&p.ident)).collect()
}

pub fn is_contexts_module(rel: &str) -> bool {
    let mods = segments(rel);
    mods.last().map(|m| m == "contexts").unwrap_or(false) && !rel.contains("/contexts/")
}

pub fn within_context(rel: &str) -> String {
    let parts: Vec<&str> = rel.split(crate::config::SLASH).collect();
    for (i, p) in parts.iter().enumerate() {
        if *p == "contexts" && i + 1 < parts.len() {
            return parts[i + 2..].join(&crate::config::SLASH.to_string());
        }
    }
    String::new()
}

pub fn context_prefix(rel: &str) -> String {
    match context_of(rel) {
        Some(name) => {
            let mut cs = name.chars();
            match cs.next() {
                Some(f) => format!("{}{}", f.to_uppercase(), cs.as_str()),
                None => String::new(),
            }
        }
        None => String::new(),
    }
}
