use std::collections::{HashMap, HashSet};
use syn::spanned::Spanned;

use patterns_macros::because;

const MIN_REASON_WORDS: usize = 4;
because!(
    MIN_REASON_WORDS,
    "four words is the shortest phrase that can state a cause rather than name a thing"
);

const MIN_NOVEL_WORDS: usize = 3;
because!(
    MIN_NOVEL_WORDS,
    "three words the item name does not already contain is the least that adds anything a reader did not have"
);

pub struct Found {
    pub chosen: Vec<(String, usize, String)>,
    pub named: Vec<(String, usize, bool, bool)>,
    pub values: HashMap<String, i128>,
    pub declared: HashSet<String>,
    pub impls: Vec<(String, String)>,
    pub defined: HashSet<String>,
    pub cited: Vec<String>,
    pub reasons: Vec<(String, String, usize)>,
    pub written_as: HashMap<String, String>,
    pub prose: Vec<(String, String, String, String, usize)>,
    pub decisions: Vec<(String, String, Vec<String>, String, usize)>,
    pub unit_sums: Vec<(String, usize)>,
    pub fns: Vec<(String, usize, bool)>,
    pub sources: HashSet<String>,
    pub value_items: HashSet<String>,
    pub provisional: Vec<(String, usize)>,
    pub rests_on: Vec<(String, Vec<String>, usize)>,
}

const STOP: &[&str] = &[
    "the", "a", "an", "of", "for", "to", "is", "it", "this", "that", "and", "in", "on", "at", "by",
    "with", "its", "be", "are", "was", "from", "as", "so", "we", "our",
];
because!(
    STOP,
    "the function words English cannot do without, discounted so a reason is judged on the words that carry meaning rather than on its articles and prepositions"
);

fn words(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect()
}

pub fn weak(item: &str, reason: &str) -> Option<String> {
    let name: HashSet<String> = item.split('_').map(|w| w.to_lowercase()).collect();
    let stop: HashSet<&str> = STOP.iter().copied().collect();
    let w = words(reason);
    if w.len() < MIN_REASON_WORDS {
        return Some(format!("reason is {} words", w.len()));
    }
    let novel = w
        .iter()
        .filter(|x| !name.contains(*x) && !stop.contains(x.as_str()))
        .count();
    if novel < MIN_NOVEL_WORDS {
        return Some(String::from("reason restates the name"));
    }
    None
}

fn type_head(t: &syn::Type) -> proc_macro2::Ident {
    match t {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| s.ident.clone())
            .unwrap_or_else(|| proc_macro2::Ident::new("unknown", proc_macro2::Span::call_site())),
        _ => proc_macro2::Ident::new("unknown", proc_macro2::Span::call_site()),
    }
}

fn unit_literal(l: &syn::Lit) -> bool {
    match l {
        syn::Lit::Int(v) => v.base10_digits() == "0" || v.base10_digits() == "1",
        syn::Lit::Bool(_) | syn::Lit::Str(_) | syn::Lit::ByteStr(_) | syn::Lit::Char(_) => true,
        _ => false,
    }
}

fn names_anything(e: &syn::Expr) -> bool {
    match e {
        syn::Expr::Path(p) => crate::scan::primitive_constant(p).is_none(),
        syn::Expr::Lit(l) => unit_literal(&l.lit),
        syn::Expr::Paren(p) => names_anything(&p.expr),
        syn::Expr::Group(g) => names_anything(&g.expr),
        syn::Expr::Unary(u) => names_anything(&u.expr),
        syn::Expr::Reference(r) => names_anything(&r.expr),
        syn::Expr::Binary(b) => names_anything(&b.left) && names_anything(&b.right),
        syn::Expr::Cast(c) => names_anything(&c.expr),
        syn::Expr::Tuple(t) => t.elems.iter().all(names_anything),
        syn::Expr::Array(a) => a.elems.iter().all(names_anything),
        syn::Expr::Repeat(r) => names_anything(&r.expr) && names_anything(&r.len),
        syn::Expr::Call(c) => !c.args.is_empty() && c.args.iter().all(names_anything),
        syn::Expr::MethodCall(m) => {
            names_anything(&m.receiver) && m.args.iter().all(names_anything)
        }
        syn::Expr::Field(f) => names_anything(&f.base),
        syn::Expr::Index(i) => names_anything(&i.expr) && names_anything(&i.index),
        syn::Expr::Struct(s) => s.fields.iter().all(|f| names_anything(&f.expr)),
        syn::Expr::Block(b) => b.block.stmts.iter().all(|s| match s {
            syn::Stmt::Expr(inner, _) => names_anything(inner),
            syn::Stmt::Local(l) => l
                .init
                .as_ref()
                .map(|i| names_anything(&i.expr))
                .unwrap_or(false),
            _ => false,
        }),
        _ => false,
    }
}

fn unit_leaves(e: &syn::Expr, values: &HashMap<String, i128>, out: &mut usize) -> bool {
    match e {
        syn::Expr::Path(p) => {
            let name = p
                .path
                .segments
                .last()
                .map(|s| crate::names::plain(&s.ident).to_lowercase())
                .unwrap_or_default();
            *out += 1;
            matches!(values.get(&name), Some(0) | Some(1))
        }
        syn::Expr::Paren(p) => unit_leaves(&p.expr, values, out),
        syn::Expr::Group(g) => unit_leaves(&g.expr, values, out),
        syn::Expr::Binary(b) => unit_leaves(&b.left, values, out) && unit_leaves(&b.right, values, out),
        _ => false,
    }
}

fn unit_shifted(e: &syn::Expr, values: &HashMap<String, i128>) -> bool {
    match e {
        syn::Expr::Binary(b) if matches!(b.op, syn::BinOp::Shl(_) | syn::BinOp::Shr(_)) => {
            let mut count = 0usize;
            unit_leaves(&b.left, values, &mut count) || unit_shifted(&b.left, values) || unit_shifted(&b.right, values)
        }
        syn::Expr::Binary(b) => unit_shifted(&b.left, values) || unit_shifted(&b.right, values),
        syn::Expr::Paren(p) => unit_shifted(&p.expr, values),
        syn::Expr::Group(g) => unit_shifted(&g.expr, values),
        _ => false,
    }
}

fn sums_units(e: &syn::Expr, values: &HashMap<String, i128>) -> bool {
    let mut count = 0usize;
    matches!(e, syn::Expr::Binary(_)) && unit_leaves(e, values, &mut count) && count >= 2
}

fn discriminated(e: &syn::ItemEnum) -> bool {
    e.variants.iter().any(|v| v.discriminant.is_some())
}

fn nested_items(block: &syn::Block, f: &mut Found) {
    for stmt in &block.stmts {
        if let syn::Stmt::Item(it) = stmt {
            scan_items(std::slice::from_ref(it), f);
        }
    }
}

fn trait_name(im: &syn::ItemImpl) -> String {
    let last = match im.trait_.as_ref().and_then(|(_, p, _)| p.segments.last()) {
        Some(s) => s,
        None => return String::new(),
    };
    let mut key = crate::names::plain(&last.ident);
    if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
        for arg in &args.args {
            if let syn::GenericArgument::Type(syn::Type::Path(tp)) = arg {
                if let Some(seg) = tp.path.segments.last() {
                    key.push('_');
                    key.push_str(&crate::names::plain(&seg.ident));
                }
            }
        }
    }
    key
}

fn gated_item(it: &syn::Item) -> bool {
    let attrs = match it {
        syn::Item::Const(c) => &c.attrs,
        syn::Item::Static(s) => &s.attrs,
        syn::Item::Enum(e) => &e.attrs,
        syn::Item::Trait(t) => &t.attrs,
        syn::Item::Impl(i) => &i.attrs,
        syn::Item::Mod(m) => &m.attrs,
        syn::Item::Struct(s) => &s.attrs,
        syn::Item::Fn(fun) => &fun.attrs,
        _ => return false,
    };
    attrs.iter().any(|a| {
        let head = a
            .path()
            .segments
            .last()
            .map(|s| crate::names::plain(&s.ident))
            .unwrap_or_default();
        head == "cfg" || head == "cfg_attr"
    })
}

pub const THREAD_LOCAL: &str = "thread_local";
patterns_macros::because!(
    THREAD_LOCAL,
    "the one macro in the standard library that declares a value item, so a reason attached to what it declares names something that is there rather than nothing"
);

const STATIC_WORD: &str = "static";
patterns_macros::because!(
    STATIC_WORD,
    "the word a declaration inside that macro begins with, read from tokens because a macro body is not parsed into items until it expands"
);

fn statics_in(tokens: proc_macro2::TokenStream) -> Vec<String> {
    let mut out = Vec::new();
    let mut after = false;
    let mut quoted = false;
    for t in tokens {
        match t {
            proc_macro2::TokenTree::Ident(id) => {
                let word = crate::names::plain(&id);
                if after {
                    out.push(word);
                    after = false;
                } else {
                    after = word == STATIC_WORD && !quoted;
                }
                quoted = false;
            }
            proc_macro2::TokenTree::Punct(p) => {
                quoted = p.as_char() == '\'';
            }
            _ => quoted = false,
        }
    }
    out
}

fn scan_items(items: &[syn::Item], f: &mut Found) {
    for it in items {
        if gated_item(it) {
            if let syn::Item::Fn(fun) = it {
                f.fns.push((
                    crate::names::plain(&fun.sig.ident),
                    fun.span().start().line,
                    false,
                ));
            }
            continue;
        }
        match it {
            syn::Item::Static(st) => {
                f.declared.insert(crate::names::plain(&st.ident));
                f.defined.insert(crate::names::plain(&st.ident));
                f.value_items.insert(crate::names::plain(&st.ident));
                f.named.push((
                    crate::names::plain(&st.ident),
                    st.span().start().line,
                    false,
                    matches!(st.vis, syn::Visibility::Public(_)),
                ));
                if !names_anything(&st.expr) {
                    f.chosen.push((
                        crate::names::plain(&st.ident),
                        st.span().start().line,
                        crate::names::plain(&type_head(&st.ty)),
                    ));
                }
            }
            syn::Item::Const(c) => {
                f.declared.insert(crate::names::plain(&c.ident));
                f.defined.insert(crate::names::plain(&c.ident));
                f.value_items.insert(crate::names::plain(&c.ident));
                if crate::names::plain(&c.ident) != "_" {
                    f.named.push((
                        crate::names::plain(&c.ident),
                        c.span().start().line,
                        false,
                        matches!(c.vis, syn::Visibility::Public(_)),
                    ));
                }
                if let syn::Expr::Lit(l) = &*c.expr {
                    if let syn::Lit::Int(v) = &l.lit {
                        if let Ok(n) = v.base10_digits().parse::<i128>() {
                            f.values.insert(crate::names::plain(&c.ident).to_lowercase(), n);
                        }
                    }
                }
                if sums_units(&c.expr, &f.values) || unit_shifted(&c.expr, &f.values) {
                    f.unit_sums.push((crate::names::plain(&c.ident), c.span().start().line));
                }
                let lone_literal = matches!(&*c.expr, syn::Expr::Lit(_));
                if (!names_anything(&c.expr) || lone_literal) && crate::names::plain(&c.ident) != "_" {
                    f.chosen.push((
                        crate::names::plain(&c.ident),
                        c.span().start().line,
                        crate::names::plain(&type_head(&c.ty)),
                    ));
                }
            }
            syn::Item::Fn(fun) => {
                f.fns.push((
                    crate::names::plain(&fun.sig.ident),
                    fun.span().start().line,
                    matches!(fun.vis, syn::Visibility::Public(_))
                        && !crate::is_entry_point(&fun.attrs),
                ));
                f.declared.insert(crate::names::plain(&fun.sig.ident));
                f.defined.insert(crate::names::plain(&fun.sig.ident));
                nested_items(&fun.block, f);
            }
            syn::Item::Impl(im) => {
                let target = match &*im.self_ty {
                    syn::Type::Path(p) => p
                        .path
                        .segments
                        .last()
                        .map(|s| crate::names::plain(&s.ident))
                        .unwrap_or_default(),
                    _ => String::new(),
                };
                let blanket = !target.is_empty() && crate::facts::bound_here(im, &target);
                let owner = if blanket { trait_name(im) } else { target.clone() };
                let inherent = im.trait_.is_none();
                for sub in &im.items {
                    if let syn::ImplItem::Fn(m) = sub {
                        if im.trait_.is_none() {
                            f.fns.push((
                                crate::names::plain(&m.sig.ident),
                                m.span().start().line,
                                false,
                            ));
                        }
                        nested_items(&m.block, f);
                    }
                }
                if !blanket && !target.is_empty() {
                    for sub in &im.items {
                        if let syn::ImplItem::Const(c) = sub {
                            let marked = trait_name(im);
                            let under = match marked.contains('_') {
                                true => format!("{}_{}_{}", target, marked, crate::names::plain(&c.ident)),
                                false => format!("{}_{}", target, crate::names::plain(&c.ident)),
                            };
                            f.named.push((
                                under,
                                c.span().start().line,
                                true,
                                true,
                            ));
                        }
                    }
                }
                if !owner.is_empty() {
                    let picks = im.items.iter().any(|sub| match sub {
                        syn::ImplItem::Const(c) => !names_anything(&c.expr) || matches!(&c.expr, syn::Expr::Lit(_)),
                        _ => false,
                    });
                    if picks {
                        let shape = match (inherent, blanket) {
                            (true, _) => String::from("inherent"),
                            (false, true) => format!("blanket:{}", trait_name(im)),
                            (false, false) => format!("impl:{}", trait_name(im)),
                        };
                        f.chosen.push((owner.clone(), im.span().start().line, shape));
                    }
                }
                if !target.is_empty() {
                    if im.trait_.is_some() {
                        f.impls.push((target.clone(), trait_name(im)));
                    }
                    f.declared.insert(target);
                }
            }
            syn::Item::Struct(s) => {
                f.declared.insert(crate::names::plain(&s.ident));
                f.defined.insert(crate::names::plain(&s.ident));
            }
            syn::Item::Enum(e) => {
                f.declared.insert(crate::names::plain(&e.ident));
                f.defined.insert(crate::names::plain(&e.ident));
                if discriminated(e) {
                    f.chosen.push((
                        crate::names::plain(&e.ident),
                        e.span().start().line,
                        String::from("enum"),
                    ));
                }
            }
            syn::Item::Trait(t) => {
                let owner = crate::names::plain(&t.ident);
                f.declared.insert(owner.clone());
                f.defined.insert(owner.clone());
                let picks = t.items.iter().any(|item| match item {
                    syn::TraitItem::Const(c) => match &c.default {
                        Some((_, e)) => !names_anything(e),
                        None => false,
                    },
                    _ => false,
                });
                if picks {
                    f.chosen.push((owner, t.span().start().line, String::from("trait")));
                }
                for item in &t.items {
                    if let syn::TraitItem::Fn(m) = item {
                        if let Some(body) = &m.default {
                            f.fns.push((
                                crate::names::plain(&m.sig.ident),
                                m.span().start().line,
                                false,
                            ));
                            nested_items(body, f);
                        }
                    }
                }
            }
            syn::Item::Type(t) => {
                f.declared.insert(crate::names::plain(&t.ident));
                f.defined.insert(crate::names::plain(&t.ident));
            }
            syn::Item::Use(u) => {
                brought_in(&u.tree, &mut f.declared);
            }
            syn::Item::Macro(m) => {
                let name = m
                    .mac
                    .path
                    .segments
                    .last()
                    .map(|s| crate::names::plain(&s.ident))
                    .unwrap_or_default();
                if name == THREAD_LOCAL {
                    for held in statics_in(m.mac.tokens.clone()) {
                        f.declared.insert(held.clone());
                        f.defined.insert(held.clone());
                        f.value_items.insert(held.clone());
                        f.named.push((held, m.span().start().line, false, false));
                    }
                    continue;
                }
                if !LANGUAGE.contains(&name.as_str()) {
                    continue;
                }
                let line = m.span().start().line;
                let mut it = m.mac.tokens.clone().into_iter();
                let item = match it.next() {
                    Some(proc_macro2::TokenTree::Ident(id)) => crate::names::plain(&id),
                    _ => continue,
                };
                let mut names: Vec<String> = Vec::new();
                let mut said: Vec<String> = Vec::new();
                for t in it {
                    match t {
                        proc_macro2::TokenTree::Ident(id) => names.push(crate::names::plain(&id)),
                        proc_macro2::TokenTree::Literal(l) => {
                            said.push(l.to_string().trim_matches(crate::names::QUOTE).to_string());
                        }
                        _ => {}
                    }
                }
                let text = said.join(" ");
                if name == "rejected" || name == "supersedes" {
                    let last = said.last().cloned().unwrap_or_default();
                    f.prose.push((name, item, text, last, line));
                    continue;
                }
                if name == "decided" {
                    let on_trait = names.join("_");
                    f.decisions.push((item, on_trait, names.clone(), text, line));
                    continue;
                }
                f.written_as.insert(item.clone(), name.clone());
                if name == "source" {
                    f.sources.insert(item.clone());
                    f.reasons.push((item, text, line));
                    continue;
                }
                f.cited.push(item.clone());
                if name == "provisional" {
                    f.provisional.push((item.clone(), line));
                }
                f.reasons.push((item.clone(), text, line));
                f.rests_on.push((item, names, line));
            }
            syn::Item::Mod(md) => {
                if let Some((_, inner)) = &md.content {
                    scan_items(inner, f);
                }
            }
            _ => {}
        }
    }
}

pub fn collect(file: &syn::File) -> Found {
    collect_with(file, &HashMap::new())
}

pub fn collect_with(file: &syn::File, seed: &HashMap<String, i128>) -> Found {
    let mut f = Found {
        chosen: Vec::new(),
        named: Vec::new(),
        values: seed.clone(),
        declared: std::collections::HashSet::new(),
        impls: Vec::new(),
        defined: HashSet::new(),
        cited: Vec::new(),
        reasons: Vec::new(),
        written_as: HashMap::new(),
        prose: Vec::new(),
        decisions: Vec::new(),
        unit_sums: Vec::new(),
        fns: Vec::new(),
        sources: HashSet::new(),
        value_items: HashSet::new(),
        provisional: Vec::new(),
        rests_on: Vec::new(),
    };
    scan_items(&file.items, &mut f);
    f
}



fn gather_enums(items: &[syn::Item], out: &mut Vec<(String, Vec<String>, usize)>) {
    for it in items {
        match it {
            syn::Item::Enum(e) => {
                let mut names: Vec<String> =
                    e.variants.iter().map(|v| crate::names::plain(&v.ident)).collect();
                names.sort();
                out.push((crate::names::plain(&e.ident), names, e.span().start().line));
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    gather_enums(inner, out);
                }
            }
            _ => {}
        }
    }
}

pub fn enums(file: &syn::File) -> Vec<(String, Vec<String>, usize)> {
    let mut out = Vec::new();
    gather_enums(&file.items, &mut out);
    out
}

pub const LANGUAGE: &[&str] = &["because", "rejected", "supersedes", "source", "provisional", "decided"];
patterns_macros::because!(
    LANGUAGE,
    "the proc macros Premise itself is built from; every other one defines a body no check can read and emits items no tool can see"
);

pub const LEANS_ON: &[&str] = &[
    "review", "survey", "study", "contract", "agreement", "regulation", "minutes", "audit",
];
because!(
    LEANS_ON,
    "the words that name a document somebody else wrote and almost nothing else in ordinary prose, which is the case source! exists for; policy, standard and release were left out because a standard bay or a release of a hold is plain English, and a word that fires on plain English gets the check switched off"
);

pub fn leans_on(reason: &str) -> Option<String> {
    words(reason)
        .into_iter()
        .find(|w| LEANS_ON.contains(&w.as_str()))
}

pub fn states_a_fact(reason: &str) -> Option<String> {
    let mut found: Vec<String> = Vec::new();
    let mut run = String::new();
    for c in reason.chars() {
        if c.is_alphanumeric() {
            run.push(c);
            continue;
        }
        if run.starts_with(|d: char| d.is_ascii_digit()) {
            found.push(run.clone());
        }
        run.clear();
    }
    if run.starts_with(|d: char| d.is_ascii_digit()) {
        found.push(run);
    }
    match found.is_empty() {
        true => None,
        false => Some(found.join(" and ")),
    }
}

fn brought_in(t: &syn::UseTree, out: &mut HashSet<String>) {
    match t {
        syn::UseTree::Name(n) => {
            out.insert(crate::names::plain(&n.ident));
        }
        syn::UseTree::Rename(r) => {
            out.insert(crate::names::plain(&r.rename));
        }
        syn::UseTree::Path(p) => brought_in(&p.tree, out),
        syn::UseTree::Group(g) => {
            for one in &g.items {
                brought_in(one, out);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}
