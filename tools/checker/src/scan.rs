use proc_macro2::TokenTree;
use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

const BANG: char = 33u8 as char;

pub const PULLS_FROM_OUTSIDE: &[&str] = &["include", "include_str", "include_bytes", "env", "option_env"];
patterns_macros::because!(
    PULLS_FROM_OUTSIDE,
    "the std macros that read a file or the build environment at compile time, so a value enters the program from somewhere no check reads"
);

pub const QUIET_ATTRIBUTES: &[&str] = &["cfg", "cfg_attr", "derive", "test", "path", "doc", "should_panic", "ignore"];
patterns_macros::because!(
    QUIET_ATTRIBUTES,
    "the attributes whose string arguments are names the compiler reads rather than prose a person reads; any other attribute carrying a string, a deprecated note, a must_use message or a lint reason, is a comment by another route"
);

pub const LINT_ATTRIBUTES: &[&str] = &["allow", "warn", "deny", "forbid", "expect"];
patterns_macros::because!(
    LINT_ATTRIBUTES,
    "the five attributes that name lints, read in spec and app for a name under a tool prefix, which the compiler never validates, and for unknown_lints itself, which only the crate root may set and only as a forbid"
);

pub const TOOL_PREFIXES: &[&str] = &["clippy", "rustdoc", "rustc"];
patterns_macros::because!(
    TOOL_PREFIXES,
    "the lint namespaces rustc accepts without checking the name, so a lint under one of them is prose to the compiler; a binding zone allows none"
);

pub const CRATE_ROOTS: &[&str] = &["patterns", "patterns_macros", "std", "core", "alloc"];
patterns_macros::because!(
    CRATE_ROOTS,
    "the crates a governed path may start from, so a local item wearing one of their names would turn every path through it into a path to anything"
);

fn shadows_root(id: &proc_macro2::Ident) -> bool {
    let name = crate::names::plain(id);
    CRATE_ROOTS.contains(&name.as_str()) || crate::roots::is(&name)
}

pub const MAX_IDENT_WORDS: usize = 8;
patterns_macros::because!(
    MAX_IDENT_WORDS,
    "a ceiling with slack over any name a domain needs: a context prefix, a two-word entity, a two-word unit and a period fit in seven, and a sentence that says something takes more; the tree's own longest name is four"
);

pub const STOP_WORDS: &[&str] = &["the", "which", "nobody", "it"];
patterns_macros::because!(
    STOP_WORDS,
    "the words no name in any domain starts or ends with and a sentence fragment does, so a sentence split across short identifiers is caught at the seam; prepositions and conjunctions are absent because is_after, days_before and AND_GATE are names, so a fragment seamed on one of those passes, and the bound is a bound"
);

pub fn fragment(name: &str) -> bool {
    let words: Vec<String> = name
        .split(UNDERSCORE)
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();
    let ends = [words.first(), words.last()];
    words.len() > 1 && ends.iter().flatten().any(|w| STOP_WORDS.contains(&w.as_str()))
}

pub const UNDERSCORE: char = '_';
patterns_macros::because!(
    UNDERSCORE,
    "the prefix that tells the compiler a binding is deliberately unread, so a string under such a name is read by nothing and is a comment"
);

fn holds_string(ts: proc_macro2::TokenStream) -> bool {
    ts.into_iter().any(|t| match t {
        TokenTree::Literal(l) => matches!(syn::parse_str::<syn::Lit>(&l.to_string()), Ok(syn::Lit::Str(_))),
        TokenTree::Group(g) => holds_string(g.stream()),
        _ => false,
    })
}

fn primitive_name(word: &str) -> bool {
    PRIMITIVES.contains(&word)
}

fn local_name(p: &syn::Pat) -> Option<String> {
    match p {
        syn::Pat::Ident(i) => Some(crate::names::plain(&i.ident)),
        syn::Pat::Type(t) => local_name(&t.pat),
        _ => None,
    }
}

fn unread_name(p: &syn::Pat) -> bool {
    match p {
        syn::Pat::Ident(i) => crate::names::plain(&i.ident).starts_with(UNDERSCORE),
        syn::Pat::Type(t) => unread_name(&t.pat),
        _ => false,
    }
}

pub fn carries_number(said: &str, types_exempt: bool) -> bool {
    said.split(|c: char| !(c.is_alphanumeric() || c == UNDERSCORE))
        .filter(|w| w.chars().any(char::is_numeric))
        .any(|w| !(types_exempt && primitive_name(w)))
}

pub fn word_count(name: &str) -> usize {
    let mut words = 0usize;
    let mut prev: Option<char> = None;
    for c in name.chars() {
        let starts = match prev {
            None => c != '_',
            Some('_') => c != '_',
            Some(p) => c.is_uppercase() && !p.is_uppercase(),
        };
        if starts {
            words += 1;
        }
        prev = Some(c);
    }
    words
}

pub const CALLABLE_MACROS: &[&str] = &["vec"];
patterns_macros::because!(
    CALLABLE_MACROS,
    "the one std macro a binding or a reducer may invoke, because an empty or listed collection has no other spelling that is not a constructor call; every other macro carries a body no check reads"
);

pub const PRIMITIVES: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
    "f64", "char", "bool", "consts",
];
patterns_macros::because!(
    PRIMITIVES,
    "the numeric types whose associated constants, MAX and MIN and the like, are values from nowhere when a binding names them, exactly as a literal would be"
);

pub struct Scan {
    pub lits: Vec<(usize, String)>,
    pub const_lits: Vec<(usize, String)>,
    pub logic: Vec<(usize, &'static str)>,
    pub foreign_macros: Vec<(usize, String)>,
    pub primitive_consts: Vec<(usize, String)>,
    pub ambiguous_roots: Vec<(usize, String)>,
    pub named_lits: std::collections::HashMap<String, Vec<(usize, String)>>,
    pub check_refs: HashSet<String>,
    pub compound_consts: HashSet<String>,
    pub const_aliases: HashMap<String, String>,
    pub laundering: HashSet<String>,
    pub laundering_aliases: Vec<(String, usize)>,
    pub const_refs: std::collections::HashMap<String, Vec<String>>,
    current_const: String,
    pub docs: Vec<usize>,
    pub macros: Vec<(usize, String, bool)>,
    pub splices: Vec<(usize, String)>,
    pub conditionals: Vec<(usize, String)>,
    pub from_patterns: HashSet<String>,
    pub patterns_shadowed: Option<usize>,
    pub patterns_glob: bool,
    pub spec_glob: bool,
    pub elsewhere_glob: bool,
    pub repeated: Vec<usize>,
    pub from_elsewhere: HashSet<String>,
    pub local_fns: HashSet<String>,
    pub splice_aliases: HashSet<String>,
    pub bindings: Vec<(String, String)>,
    pub check_calls: bool,
    pub tables_ok: bool,
    pub shouty: HashSet<String>,
    pub library_fns: HashSet<String>,
    pub spec_types: HashSet<String>,
    pub library_types: HashSet<String>,
    pub from_spec: HashSet<String>,
    pub type_aliases: Vec<usize>,
    pub foreign_idents: Vec<(usize, String)>,
    pub allow_strings: bool,
    pub lints_named: bool,
    pub digits_watched: bool,
    pub idents_watched: bool,
    pub prose_idents: Vec<(usize, String)>,
    pub fragment_idents: Vec<(usize, String)>,
    pub tool_lints: Vec<(usize, String)>,
    pub lifted_lints: Vec<usize>,
    pub fact_strings: Vec<(usize, String)>,
    pub stray_strings: Vec<usize>,
    pub underscored: Vec<usize>,
    pub literal_tables: Vec<(usize, String, usize)>,
    pub string_locals: HashSet<String>,
    pub valued_strings: HashSet<(usize, usize)>,
    pub mentions: HashMap<String, usize>,
    pub allow_lexical: bool,
    pub allow_embed: bool,
    pub assembled: Vec<usize>,
    in_assembled: bool,
    in_const: bool,
    in_trait_impl: bool,
    in_anon_const: bool,
}

impl Scan {
    fn note_fragment(&mut self, id: &proc_macro2::Ident) {
        let spelled = crate::names::plain(id);
        if self.idents_watched && fragment(&spelled) && !self.fragment_idents.iter().any(|(_, seen)| seen == &spelled) {
            self.fragment_idents.push((id.span().start().line, spelled));
        }
    }

    pub fn new() -> Scan {
        Scan {
            lits: Vec::new(),
            const_lits: Vec::new(),
            logic: Vec::new(),
            foreign_macros: Vec::new(),
            primitive_consts: Vec::new(),
            ambiguous_roots: Vec::new(),
            named_lits: std::collections::HashMap::new(),
            check_refs: HashSet::new(),
            compound_consts: HashSet::new(),
            const_aliases: HashMap::new(),
            laundering: HashSet::new(),
            laundering_aliases: Vec::new(),
            const_refs: std::collections::HashMap::new(),
            current_const: String::new(),
            docs: Vec::new(),
            macros: Vec::new(),
            splices: Vec::new(),
            conditionals: Vec::new(),
            from_patterns: HashSet::new(),
            patterns_shadowed: None,
            patterns_glob: false,
            spec_glob: false,
            elsewhere_glob: false,
            repeated: Vec::new(),
            from_elsewhere: HashSet::new(),
            local_fns: HashSet::new(),
            splice_aliases: HashSet::new(),
            bindings: Vec::new(),
            check_calls: false,
            tables_ok: false,
            shouty: HashSet::new(),
            library_fns: HashSet::new(),
            spec_types: HashSet::new(),
            library_types: HashSet::new(),
            from_spec: HashSet::new(),
            type_aliases: Vec::new(),
            foreign_idents: Vec::new(),
            allow_strings: false,
            lints_named: false,
            digits_watched: false,
            idents_watched: false,
            prose_idents: Vec::new(),
            fragment_idents: Vec::new(),
            tool_lints: Vec::new(),
            lifted_lints: Vec::new(),
            fact_strings: Vec::new(),
            stray_strings: Vec::new(),
            underscored: Vec::new(),
            literal_tables: Vec::new(),
            string_locals: HashSet::new(),
            valued_strings: HashSet::new(),
            mentions: HashMap::new(),
            allow_lexical: false,
            allow_embed: false,
            assembled: Vec::new(),
            in_assembled: false,
            in_const: false,
            in_trait_impl: false,
            in_anon_const: false,
        }
    }
}

impl Default for Scan {
    fn default() -> Scan {
        Scan::new()
    }
}

fn leaves(t: &syn::UseTree, out: &mut HashSet<String>) {
    match t {
        syn::UseTree::Path(p) => leaves(&p.tree, out),
        syn::UseTree::Name(n) => {
            out.insert(crate::names::plain(&n.ident));
        }
        syn::UseTree::Rename(r) => {
            out.insert(crate::names::plain(&r.rename));
        }
        syn::UseTree::Group(g) => {
            for i in &g.items {
                leaves(i, out);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

fn pairs(t: &syn::UseTree, out: &mut Vec<(String, String)>) {
    match t {
        syn::UseTree::Path(p) => pairs(&p.tree, out),
        syn::UseTree::Name(n) => {
            let id = crate::names::plain(&n.ident);
            out.push((id.clone(), id));
        }
        syn::UseTree::Rename(r) => {
            out.push((crate::names::plain(&r.ident), crate::names::plain(&r.rename)));
        }
        syn::UseTree::Group(g) => {
            for i in &g.items {
                pairs(i, out);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

fn globbed(t: &syn::UseTree) -> bool {
    match t {
        syn::UseTree::Path(p) => globbed(&p.tree),
        syn::UseTree::Group(g) => g.items.iter().any(globbed),
        syn::UseTree::Glob(_) => true,
        _ => false,
    }
}

fn use_root(t: &syn::UseTree) -> String {
    match t {
        syn::UseTree::Path(p) => crate::names::plain(&p.ident),
        syn::UseTree::Name(n) => crate::names::plain(&n.ident),
        syn::UseTree::Rename(r) => crate::names::plain(&r.ident),
        _ => String::new(),
    }
}

fn gather(items: &[syn::Item], s: &mut Scan) {
    for it in items {
        match it {
            syn::Item::Use(u) => {
                let mut bound = Vec::new();
                pairs(&u.tree, &mut bound);
                for (origin, local) in &bound {
                    s.bindings.push((origin.clone(), local.clone()));
                }
                let mut names = HashSet::new();
                leaves(&u.tree, &mut names);
                let from_library = crate::roots::is(&use_root(&u.tree));
                if from_library && globbed(&u.tree) {
                    s.patterns_glob = true;
                }
                if !from_library && bound.iter().any(|(origin, local)| crate::roots::is(local) && origin != local) {
                    s.patterns_shadowed = Some(u.span().start().line);
                }
                let root = use_root(&u.tree);
                if from_library {
                    s.from_patterns.extend(names);
                } else if root == "spec" || root == "crate" || root == "super" || root == "self" {
                    if globbed(&u.tree) {
                        s.spec_glob = true;
                    }
                    s.from_spec.extend(names);
                } else {
                    if globbed(&u.tree) {
                        s.elsewhere_glob = true;
                    }
                    s.from_elsewhere.extend(names);
                }
            }
            syn::Item::ExternCrate(e) => {
                let renamed = e.rename.as_ref().map(|(_, r)| shadows_root(r)).unwrap_or(false);
                if renamed && crate::names::plain(&e.ident) != crate::names::plain(e.rename.as_ref().map(|(_, r)| r).unwrap_or(&e.ident)) {
                    s.patterns_shadowed = Some(e.span().start().line);
                }
            }
            syn::Item::Struct(st) if shadows_root(&st.ident) => {
                s.patterns_shadowed = Some(st.span().start().line);
            }
            syn::Item::Enum(en) if shadows_root(&en.ident) => {
                s.patterns_shadowed = Some(en.span().start().line);
            }
            syn::Item::Type(ty) => {
                if shadows_root(&ty.ident) {
                    s.patterns_shadowed = Some(ty.span().start().line);
                }
                s.type_aliases.push(ty.span().start().line);
                if launders(&ty.ty) {
                    s.laundering_aliases.push((crate::names::plain(&ty.ident), ty.span().start().line));
                }
            }
            syn::Item::Trait(t) if shadows_root(&t.ident) => {
                s.patterns_shadowed = Some(t.span().start().line);
            }
            syn::Item::Const(c) => {
                if let syn::Expr::Path(p) = &*c.expr {
                    s.const_aliases.insert(crate::names::plain(&c.ident), p.to_token_stream().to_string());
                }
            }
            syn::Item::Fn(f) => {
                if shadows_root(&f.sig.ident) {
                    s.patterns_shadowed = Some(f.span().start().line);
                }
                s.local_fns.insert(crate::names::plain(&f.sig.ident));
            }
            syn::Item::Mod(m) => {
                if shadows_root(&m.ident) {
                    s.patterns_shadowed = Some(m.span().start().line);
                }
                if let Some((_, inner)) = &m.content {
                    gather(inner, s);
                }
            }
            _ => {}
        }
    }
}

pub fn imports(file: &syn::File, s: &mut Scan) {
    gather(&file.items, s);
    s.splice_aliases.insert(String::from("include"));
    s.splice_aliases.insert(String::from("path"));
    let bound = s.bindings.len() + 1;
    let links = s.bindings.clone();
    for _ in 0..bound {
        let before = s.splice_aliases.len();
        for (origin, local) in &links {
            if s.splice_aliases.contains(origin) {
                s.splice_aliases.insert(local.clone());
            }
        }
        if s.splice_aliases.len() == before {
            break;
        }
    }
}

fn exempt(l: &syn::Lit, strings_ok: bool, lexical_ok: bool) -> bool {
    match l {
        syn::Lit::Int(v) => {
            let d = v.base10_digits();
            d == "0" || d == "1" || (lexical_ok && d == "2")
        }
        syn::Lit::Bool(_) => true,
        syn::Lit::Char(_) => lexical_ok,
        syn::Lit::Str(_) => strings_ok,
        _ => false,
    }
}

fn text(l: &syn::Lit) -> String {
    match l {
        syn::Lit::Int(v) => v.base10_digits().to_string(),
        syn::Lit::Float(v) => v.base10_digits().to_string(),
        syn::Lit::Str(v) => format!("{:?}", v.value()),
        syn::Lit::Char(v) => format!("{:?}", v.value()),
        syn::Lit::ByteStr(_) => String::from("a byte string, which is a number"),
        _ => String::from("literal"),
    }
}

fn tokens(
    ts: proc_macro2::TokenStream,
    out: &mut Vec<(usize, String)>,
    strings_ok: bool,
    lexical_ok: bool,
) {
    for t in ts {
        match t {
            TokenTree::Literal(l) => {
                let s = l.to_string();
                let parsed: Option<syn::Lit> = syn::parse_str(&s).ok();
                let fine = parsed
                    .as_ref()
                    .map(|lit| exempt(lit, strings_ok, lexical_ok))
                    .unwrap_or(false);
                if !fine {
                    out.push((
                        l.span().start().line,
                        parsed.as_ref().map(text).unwrap_or(s),
                    ));
                }
            }
            TokenTree::Group(g) => tokens(g.stream(), out, strings_ok, lexical_ok),
            _ => {}
        }
    }
}

fn listed_arguments(ts: proc_macro2::TokenStream) -> Vec<syn::Expr> {
    type Listed = syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>;
    if let Ok(list) = syn::parse::Parser::parse2(Listed::parse_terminated, ts.clone()) {
        return list.into_iter().collect();
    }
    let repeated = |input: syn::parse::ParseStream| -> syn::Result<(syn::Expr, syn::Expr)> {
        let each: syn::Expr = input.parse()?;
        input.parse::<syn::Token![;]>()?;
        let times: syn::Expr = input.parse()?;
        Ok((each, times))
    };
    match syn::parse::Parser::parse2(repeated, ts) {
        Ok((each, times)) => vec![each, times],
        Err(_) => Vec::new(),
    }
}

struct Imports<'a> {
    from_spec: &'a HashSet<String>,
    laundering: &'a HashSet<String>,
    from_patterns: &'a HashSet<String>,
    from_elsewhere: &'a HashSet<String>,
    spec_types: &'a HashSet<String>,
    library_types: &'a HashSet<String>,
    patterns_glob: bool,
    in_spec: bool,
    elsewhere_glob: bool,
}

enum Verdict {
    Foreign,
    Ambiguous,
}

fn foreign_constant(p: &syn::ExprPath, scope: &Imports) -> Option<(String, Verdict)> {
    let mut segs: Vec<String> = p.path.segments.iter().map(|s| crate::names::plain(&s.ident)).collect();
    if let Some(q) = &p.qself {
        if let syn::Type::Path(inner) = &*q.ty {
            if let Some(head) = inner.path.segments.last() {
                segs.insert(0, crate::names::plain(&head.ident));
            }
        }
    }
    let last = segs.last()?.clone();
    let shouted = last.chars().all(|c| c.is_ascii_uppercase() || c == '_') && last.chars().any(|c| c.is_ascii_uppercase());
    if !shouted {
        return None;
    }
    if p.qself.is_some() && scope.in_spec {
        return None;
    }
    let shown = match p.qself.is_some() && segs.len() > 1 {
        true => format!("<{} as {}>::{}", segs[0], segs[1..segs.len() - 1].join("::"), last),
        false => segs.join("::"),
    };
    let root = segs[0].as_str();
    if segs.len() > 1 && scope.laundering.contains(root) {
        return Some((segs.join("::"), Verdict::Foreign));
    }
    let named_by_spec = scope.from_spec.contains(root) || scope.spec_types.contains(root);
    if segs.len() == 1 {
        let stray = scope.from_elsewhere.contains(root)
            || (scope.elsewhere_glob && !named_by_spec && !scope.from_patterns.contains(root));
        return match stray {
            true => Some((root.to_string(), Verdict::Foreign)),
            false => None,
        };
    }
    let named = matches!(root, "Self" | "crate" | "spec" | "self" | "super")
        || crate::roots::is(root)
        || scope.from_spec.contains(root)
        || scope.from_patterns.contains(root);
    if named {
        return None;
    }
    let imported_elsewhere = scope.from_elsewhere.contains(root);
    let candidate = (scope.patterns_glob && scope.library_types.contains(root)) || scope.spec_types.contains(root);
    if candidate && !imported_elsewhere && scope.elsewhere_glob {
        return Some((shown, Verdict::Ambiguous));
    }
    match candidate && !imported_elsewhere {
        true => None,
        false => Some((shown, Verdict::Foreign)),
    }
}

pub fn primitive_constant(p: &syn::ExprPath) -> Option<String> {
    let mut segs: Vec<String> = p
        .path
        .segments
        .iter()
        .map(|s| crate::names::plain(&s.ident))
        .collect();
    if let Some(q) = &p.qself {
        if let syn::Type::Path(inner) = &*q.ty {
            if let Some(head) = inner.path.segments.last() {
                segs.insert(0, crate::names::plain(&head.ident));
            }
        }
    }
    let last = match segs.last() {
        Some(l) if segs.len() >= 2 => l.clone(),
        _ => return None,
    };
    let shouted = last.chars().all(|c| c.is_ascii_uppercase() || c == '_') && last.chars().any(|c| c.is_ascii_uppercase());
    let through_primitive = segs[..segs.len() - 1].iter().any(|s| PRIMITIVES.contains(&s.as_str()));
    match through_primitive && shouted {
        true => Some(segs.join("::")),
        false => None,
    }
}

fn splices_in(
    ts: proc_macro2::TokenStream,
    out: &mut Vec<(usize, String)>,
    aliases: &HashSet<String>,
) {
    let items: Vec<TokenTree> = ts.into_iter().collect();
    for (at, t) in items.iter().enumerate() {
        match t {
            TokenTree::Ident(id) => {
                let bang = matches!(items.get(at + 1), Some(TokenTree::Punct(p)) if p.as_char() == BANG);
                let spelled = crate::names::plain(id);
                let handed = spelled == "include" || aliases.contains(&spelled);
                let invoked = bang && PULLS_FROM_OUTSIDE.contains(&spelled.as_str());
                if handed || invoked {
                    out.push((
                        id.span().start().line,
                        String::from("include! reached through a macro pulls another file into this zone, past every check"),
                    ));
                }
            }
            TokenTree::Group(g) => {
                if g.delimiter() == proc_macro2::Delimiter::Bracket {
                    for inner in g.stream() {
                        if let TokenTree::Ident(id) = inner {
                            if crate::names::is(&id, "path") {
                                out.push((
                                    id.span().start().line,
                                    String::from("a path attribute inside a macro body pulls another file into this zone, past every check"),
                                ));
                            }
                        }
                    }
                }
                splices_in(g.stream(), out, aliases);
            }
            _ => {}
        }
    }
}

fn idents_in(ts: proc_macro2::TokenStream) -> Vec<String> {
    let mut out = Vec::new();
    for t in ts {
        match t {
            TokenTree::Ident(id) => out.push(crate::names::plain(&id)),
            TokenTree::Group(g) => out.extend(idents_in(g.stream())),
            _ => {}
        }
    }
    out
}

fn carries_text(ts: proc_macro2::TokenStream) -> bool {
    for t in ts {
        match t {
            TokenTree::Literal(l) if l.to_string().contains(crate::names::QUOTE) => return true,
            TokenTree::Group(g) if carries_text(g.stream()) => return true,
            _ => {}
        }
    }
    false
}

fn doc_in(ts: proc_macro2::TokenStream) -> bool {
    for t in ts {
        match t {
            TokenTree::Ident(id) => {
                if crate::names::is(&id, "doc") {
                    return true;
                }
            }
            TokenTree::Group(g) if doc_in(g.stream()) => return true,
            _ => {}
        }
    }
    false
}

fn upper(name: &str) -> bool {
    name.chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}

fn all_literals(e: &syn::Expr) -> bool {
    match e {
        syn::Expr::Lit(_) => true,
        syn::Expr::Paren(p) => all_literals(&p.expr),
        syn::Expr::Group(g) => all_literals(&g.expr),
        syn::Expr::Unary(u) => all_literals(&u.expr),
        syn::Expr::Binary(b) => all_literals(&b.left) && all_literals(&b.right),
        _ => false,
    }
}

fn path_leaves(e: &syn::Expr, out: &mut Vec<String>) -> bool {
    match e {
        syn::Expr::Path(p) => {
            out.push(p.to_token_stream().to_string());
            true
        }
        syn::Expr::Paren(p) => path_leaves(&p.expr, out),
        syn::Expr::Group(g) => path_leaves(&g.expr, out),
        syn::Expr::Binary(b) if matches!(b.op, syn::BinOp::Add(_)) => {
            path_leaves(&b.left, out) && path_leaves(&b.right, out)
        }
        _ => false,
    }
}

fn canonical(name: &str, aliases: &HashMap<String, String>) -> String {
    let mut at = name.to_string();
    let mut hops = 0usize;
    while let Some(next) = aliases.get(&at) {
        if hops > aliases.len() {
            break;
        }
        at = next.clone();
        hops += 1;
    }
    at
}

fn counted_literals(ts: proc_macro2::TokenStream) -> usize {
    let mut n = 0usize;
    for t in ts {
        match t {
            TokenTree::Literal(l) => {
                let parsed: Option<syn::Lit> = syn::parse_str(&l.to_string()).ok();
                let unit = parsed.map(|p| exempt(&p, false, false)).unwrap_or(false);
                if !unit {
                    n += 1;
                }
            }
            TokenTree::Group(g) => n += counted_literals(g.stream()),
            _ => {}
        }
    }
    n
}

fn self_cancelling(e: &syn::Expr, aliases: &HashMap<String, String>) -> bool {
    match e {
        syn::Expr::Binary(b) => {
            let cancels = matches!(b.op, syn::BinOp::Sub(_) | syn::BinOp::Div(_) | syn::BinOp::Rem(_) | syn::BinOp::BitXor(_));
            let same = canonical(&b.left.to_token_stream().to_string(), aliases)
                == canonical(&b.right.to_token_stream().to_string(), aliases);
            let named_only = !b.left.to_token_stream().into_iter().any(|t| matches!(t, TokenTree::Literal(_)));
            (cancels && same && named_only)
                || self_cancelling(&b.left, aliases)
                || self_cancelling(&b.right, aliases)
        }
        syn::Expr::Paren(p) => self_cancelling(&p.expr, aliases),
        syn::Expr::Group(g) => self_cancelling(&g.expr, aliases),
        _ => false,
    }
}

fn repeated_name(e: &syn::Expr, aliases: &HashMap<String, String>) -> bool {
    if self_cancelling(e, aliases) {
        return true;
    }
    let mut seen = Vec::new();
    if !matches!(e, syn::Expr::Binary(_)) || !path_leaves(e, &mut seen) {
        return false;
    }
    let first = canonical(&seen[0], aliases);
    seen.len() >= 2 && seen.iter().all(|s| canonical(s, aliases) == first)
}

struct HasBinary(bool);

impl<'ast> Visit<'ast> for HasBinary {
    fn visit_expr_binary(&mut self, _: &'ast syn::ExprBinary) {
        self.0 = true;
    }
}

fn has_binary(e: &syn::Expr) -> bool {
    let mut h = HasBinary(false);
    h.visit_expr(e);
    h.0
}

pub fn launders(t: &syn::Type) -> bool {
    match t {
        syn::Type::Path(p) => {
            let root = p.path.segments.first().map(|s| crate::names::plain(&s.ident)).unwrap_or_default();
            (PRIMITIVES.contains(&root.as_str()) && root != "consts") || root == "std" || root == "core" || root == "alloc"
        }
        syn::Type::Tuple(tu) => !tu.elems.is_empty() && tu.elems.iter().all(launders),
        syn::Type::Array(a) => launders(&a.elem),
        syn::Type::Slice(s) => launders(&s.elem),
        syn::Type::Reference(r) => launders(&r.elem),
        syn::Type::Paren(p) => launders(&p.elem),
        syn::Type::Group(g) => launders(&g.elem),
        _ => false,
    }
}

fn plain_scrutinee(e: &syn::Expr) -> bool {
    match e {
        syn::Expr::Path(_) => true,
        syn::Expr::Field(f) => plain_scrutinee(&f.base),
        syn::Expr::Reference(r) => plain_scrutinee(&r.expr),
        syn::Expr::Paren(p) => plain_scrutinee(&p.expr),
        syn::Expr::Unary(u) => matches!(u.op, syn::UnOp::Deref(_)) && plain_scrutinee(&u.expr),
        _ => false,
    }
}

fn variant_shaped(p: &syn::Pat) -> bool {
    match p {
        syn::Pat::Ident(i) => {
            i.subpat.is_none()
                && i.by_ref.is_none()
                && i.mutability.is_none()
                && crate::names::plain(&i.ident)
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
        }
        syn::Pat::Path(_) => true,
        syn::Pat::TupleStruct(_) => true,
        syn::Pat::Struct(_) => true,
        syn::Pat::Reference(r) => variant_shaped(&r.pat),
        syn::Pat::Paren(p) => variant_shaped(&p.pat),
        syn::Pat::Or(o) => o.cases.iter().all(variant_shaped),
        _ => false,
    }
}

fn total_arms(m: &syn::ExprMatch) -> bool {
    for arm in &m.arms {
        if arm.guard.is_some() {
            return false;
        }
        if !variant_shaped(&arm.pat) {
            return false;
        }
    }
    true
}

impl Scan {
    fn check_call(&mut self, c: &syn::ExprCall, line: usize) {
        let path = match &*c.func {
            syn::Expr::Path(p) if p.qself.is_some() => {
                let trait_named = p
                    .path
                    .segments
                    .first()
                    .map(|s| crate::names::plain(&s.ident))
                    .unwrap_or_default();
                if self.tables_ok && self.from_spec.contains(&trait_named) {
                    return;
                }
                self.logic.push((line, "a call through a qualified path into spec logic"));
                return;
            }
            syn::Expr::Path(p) => &p.path,
            _ => {
                self.logic.push((line, "a call through an expression"));
                return;
            }
        };
        let segs: Vec<String> = path.segments.iter().map(|s| crate::names::plain(&s.ident)).collect();
        let first = segs.first().cloned().unwrap_or_default();
        let last = segs.last().cloned().unwrap_or_default();

        let in_library = self.library_fns.is_empty() || self.library_fns.contains(&last);
        if crate::roots::is(&first) && (path.leading_colon.is_some() || self.patterns_shadowed.is_none()) {
            if !in_library {
                self.logic.push((line, "a call to a name the patterns crate does not export"));
            }
            return;
        }
        if self.from_elsewhere.contains(&last) {
            self.logic.push((line, "a call to a non-pattern"));
            return;
        }
        if segs.len() > 1 {
            if segs.iter().all(|s| upper(s)) && !self.shouty.contains(&last) {
                return;
            }
            if segs.len() == 2 && self.from_patterns.contains(&first) && !upper(&first) && in_library {
                return;
            }
            self.logic.push((line, "a call to a non-pattern"));
            return;
        }
        if self.local_fns.contains(&last) || upper(&last) {
            return;
        }
        let named_in = self.from_patterns.contains(&last) || self.patterns_glob;
        if named_in && in_library {
            return;
        }
        self.logic.push((line, "a call to a non-pattern"));
    }
}

impl<'ast> Visit<'ast> for Scan {
    fn visit_item_struct(&mut self, s: &'ast syn::ItemStruct) {
        self.note_fragment(&s.ident);
        visit::visit_item_struct(self, s);
    }

    fn visit_item_type(&mut self, t: &'ast syn::ItemType) {
        self.note_fragment(&t.ident);
        visit::visit_item_type(self, t);
    }

    fn visit_field(&mut self, f: &'ast syn::Field) {
        if let Some(id) = &f.ident {
            self.note_fragment(id);
        }
        visit::visit_field(self, f);
    }

    fn visit_pat_ident(&mut self, p: &'ast syn::PatIdent) {
        self.note_fragment(&p.ident);
        visit::visit_pat_ident(self, p);
    }

    fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
        let spelled = crate::names::plain(i);
        *self.mentions.entry(spelled.clone()).or_default() += 1;
        if !spelled.is_ascii() && !self.foreign_idents.iter().any(|(_, seen)| seen == &spelled) {
            self.foreign_idents.push((i.span().start().line, spelled.clone()));
        }
        if self.idents_watched && word_count(&spelled) > MAX_IDENT_WORDS && !self.prose_idents.iter().any(|(_, seen)| seen == &spelled) {
            self.prose_idents.push((i.span().start().line, spelled.clone()));
        }
    }

    fn visit_attribute(&mut self, a: &'ast syn::Attribute) {
        let head = a
            .path()
            .segments
            .last()
            .map(|s| crate::names::plain(&s.ident))
            .unwrap_or_default();
        let documented = head == "doc" || doc_in(a.meta.to_token_stream());
        let prosed = !QUIET_ATTRIBUTES.contains(&head.as_str()) && carries_text(a.meta.to_token_stream());
        if documented || prosed {
            self.docs.push(a.span().start().line);
        }
        if self.lints_named && LINT_ATTRIBUTES.contains(&head.as_str()) {
            let named = a
                .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .map(|list| list.iter().map(|m| m.path().clone()).collect::<Vec<syn::Path>>())
                .unwrap_or_default();
            let line = a.span().start().line;
            for p in &named {
                let root = p.segments.first().map(|s| crate::names::plain(&s.ident)).unwrap_or_default();
                if p.segments.len() > 1 && TOOL_PREFIXES.contains(&root.as_str()) {
                    self.tool_lints.push((line, p.to_token_stream().to_string().replace(' ', "")));
                }
                let forbidding = head == "forbid" && matches!(a.style, syn::AttrStyle::Inner(_));
                if p.is_ident("unknown_lints") && !forbidding {
                    self.lifted_lints.push(line);
                }
            }
        }
        if head == "path" {
            self.splices
                .push((a.span().start().line, String::from("a path attribute pulls another file into this zone, past every check")));
        }
        if head == "cfg" || head == "cfg_attr" {
            let inner = match &a.meta {
                syn::Meta::List(l) => l.tokens.to_string(),
                _ => String::new(),
            };
            if inner != "test" {
                self.conditionals.push((
                    a.span().start().line,
                    format!("{} makes this true in some builds and not others, so rustc compiles one program while the checker reads another; only cfg(test) is allowed, because the gate runs cargo test", head),
                ));
            }
        }
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        self.note_fragment(&i.ident);
        if i.content.is_none() {
            for a in &i.attrs {
                let head = a
                    .path()
                    .segments
                    .last()
                    .map(|s| crate::names::plain(&s.ident))
                    .unwrap_or_default();
                if head == "cfg" || head == "cfg_attr" {
                    self.conditionals.push((
                        a.span().start().line,
                        String::from("a cfg on a file module keeps it out of a normal build while the checker still reads it; cfg(test) is allowed on an inline mod, never on a file"),
                    ));
                }
            }
        }
        visit::visit_item_mod(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        let prev = self.in_const;
        self.in_const = true;
        visit::visit_item_static(self, i);
        self.in_const = prev;
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        self.note_fragment(&i.ident);
        let prev = self.in_const;
        let prev_anon = self.in_anon_const;
        self.in_const = true;
        let flag = match &*i.ty {
            syn::Type::Path(tp) => tp.path.is_ident("bool"),
            _ => false,
        };
        let unit = matches!(&*i.ty, syn::Type::Tuple(t) if t.elems.is_empty()) || flag;
        if crate::names::plain(&i.ident).starts_with(UNDERSCORE) && !unit {
            self.underscored.push(i.span().start().line);
        }
        if matches!(&*i.expr, syn::Expr::Array(_) | syn::Expr::Tuple(_) | syn::Expr::Struct(_) | syn::Expr::Repeat(_)) {
            let leaves = counted_literals(i.expr.to_token_stream());
            if leaves > 1 {
                self.literal_tables.push((i.span().start().line, crate::names::plain(&i.ident), leaves));
            }
        }
        let compared = matches!(&*i.expr, syn::Expr::Binary(b) if matches!(b.op, syn::BinOp::Eq(_) | syn::BinOp::Ne(_) | syn::BinOp::Lt(_) | syn::BinOp::Le(_) | syn::BinOp::Gt(_) | syn::BinOp::Ge(_)));
        let checked = matches!(&*i.expr, syn::Expr::Macro(_)) || compared;
        self.in_anon_const = crate::names::plain(&i.ident) == "_" || unit || checked;
        if has_binary(&i.expr) && !compared {
            self.compound_consts.insert(crate::names::plain(&i.ident));
        }
        let prev_name = std::mem::replace(&mut self.current_const, crate::names::plain(&i.ident));
        visit::visit_item_const(self, i);
        self.current_const = prev_name;
        self.in_const = prev;
        self.in_anon_const = prev_anon;
    }

    fn visit_impl_item_const(&mut self, i: &'ast syn::ImplItemConst) {
        let prev = self.in_const;
        self.in_const = true;
        visit::visit_impl_item_const(self, i);
        self.in_const = prev;
    }

    fn visit_trait_item_const(&mut self, i: &'ast syn::TraitItemConst) {
        let prev = self.in_const;
        self.in_const = true;
        visit::visit_trait_item_const(self, i);
        self.in_const = prev;
    }

    fn visit_variant(&mut self, v: &'ast syn::Variant) {
        self.note_fragment(&v.ident);
        let prev = self.in_const;
        self.in_const = true;
        visit::visit_variant(self, v);
        self.in_const = prev;
    }

    fn visit_arm(&mut self, a: &'ast syn::Arm) {
        if let syn::Expr::Lit(l) = &*a.body {
            self.valued_strings.insert((l.span().start().line, l.span().start().column));
        }
        visit::visit_arm(self, a);
    }

    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if let Some(syn::Stmt::Expr(syn::Expr::Lit(l), None)) = i.block.stmts.last() {
            self.valued_strings.insert((l.span().start().line, l.span().start().column));
        }
        self.note_fragment(&i.sig.ident);
        if i.sig.asyncness.is_some() && self.check_calls {
            self.logic.push((i.span().start().line, "an async function"));
        }
        visit::visit_impl_item_fn(self, i);
    }

    fn visit_trait_item_fn(&mut self, i: &'ast syn::TraitItemFn) {
        self.note_fragment(&i.sig.ident);
        if i.sig.asyncness.is_some() && self.check_calls {
            self.logic.push((i.span().start().line, "an async function"));
        }
        visit::visit_trait_item_fn(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.note_fragment(&i.sig.ident);
        if i.sig.asyncness.is_some() && self.check_calls {
            self.logic.push((i.span().start().line, "an async function"));
        }
        let kind = i.attrs.iter().find_map(|a| {
            let head = a
                .path()
                .segments
                .last()
                .map(|s| crate::names::plain(&s.ident))
                .unwrap_or_default();
            match head.as_str() {
                "proc_macro" | "proc_macro_derive" | "proc_macro_attribute" => Some(head),
                _ => None,
            }
        });
        if let Some(kind) = kind {
            let name = crate::names::plain(&i.sig.ident);
            let body = i.block.to_token_stream().to_string();
            let makes_facts = body.contains("pub const") || body.contains("pub static");
            let derives = kind == "proc_macro_derive" && !makes_facts;
            if (!crate::because::LANGUAGE.contains(&name.as_str()) && !derives) || makes_facts {
                self.macros
                    .push((i.span().start().line, format!("{} {}", kind, name), true));
            }
        }
        visit::visit_item_fn(self, i);
    }

    fn visit_item_macro(&mut self, i: &'ast syn::ItemMacro) {
        if let Some(id) = &i.ident {
            let shared = i.attrs.iter().any(|a| {
                a.path()
                    .segments
                    .last()
                    .map(|s| crate::names::is(&s.ident, "macro_export"))
                    .unwrap_or(false)
            });
            self.macros
                .push((i.span().start().line, crate::names::plain(id), shared));
        }
        visit::visit_item_macro(self, i);
    }

    fn visit_item_enum(&mut self, e: &'ast syn::ItemEnum) {
        self.note_fragment(&e.ident);
        let leaves = e
            .variants
            .iter()
            .filter_map(|v| v.discriminant.as_ref().map(|(_, d)| d.to_token_stream()))
            .map(counted_literals)
            .sum::<usize>();
        if leaves > 1 {
            self.literal_tables.push((e.span().start().line, crate::names::plain(&e.ident), leaves));
        }
        visit::visit_item_enum(self, e);
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        let wild = matches!(l.pat, syn::Pat::Wild(_)) || unread_name(&l.pat);
        let stringed = l
            .init
            .as_ref()
            .map(|i| matches!(&*i.expr, syn::Expr::Lit(e) if matches!(e.lit, syn::Lit::Str(_))))
            .unwrap_or(false);
        let discarded = (matches!(l.pat, syn::Pat::Wild(_)) || unread_name(&l.pat))
            && l
                .init
                .as_ref()
                .map(|i| match &*i.expr {
                    syn::Expr::Path(p) => p.path.get_ident().map(|id| self.string_locals.contains(&crate::names::plain(id))).unwrap_or(false),
                    _ => false,
                })
                .unwrap_or(false);
        let let_string = self.tables_ok && l.init.as_ref().map(|i| holds_string(i.expr.to_token_stream())).unwrap_or(false);
        if (wild && stringed) || discarded || let_string {
            self.underscored.push(l.span().start().line);
        }
        if stringed {
            if let Some(name) = local_name(&l.pat) {
                self.string_locals.insert(name);
            }
        }
        if let Some(init) = &l.init {
            if init.diverge.is_some() {
                self.logic.push((l.span().start().line, "let else"));
            }
        }
        visit::visit_local(self, l);
    }

    fn visit_item_trait(&mut self, t: &'ast syn::ItemTrait) {
        self.note_fragment(&t.ident);
        let leaves = t
            .items
            .iter()
            .filter_map(|it| match it {
                syn::TraitItem::Const(c) => c.default.as_ref().map(|(_, e)| e.to_token_stream()),
                _ => None,
            })
            .map(counted_literals)
            .sum::<usize>();
        if leaves > 1 {
            self.literal_tables.push((t.span().start().line, crate::names::plain(&t.ident), leaves));
        }
        visit::visit_item_trait(self, t);
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let leaves = i
            .items
            .iter()
            .filter_map(|it| match it {
                syn::ImplItem::Const(c) => Some(c.expr.to_token_stream()),
                _ => None,
            })
            .map(counted_literals)
            .sum::<usize>();
        if leaves > 1 {
            let shown = format!("the impl on {}", i.self_ty.to_token_stream().to_string().replace(' ', ""));
            self.literal_tables.push((i.span().start().line, shown, leaves));
        }
        let prev = self.in_trait_impl;
        self.in_trait_impl = i.trait_.is_some();
        visit::visit_item_impl(self, i);
        self.in_trait_impl = prev;
    }

    fn visit_lit(&mut self, l: &'ast syn::Lit) {
        if let syn::Lit::Str(v) = l {
            let said = v.value();
            if self.digits_watched && carries_number(&said, !self.tables_ok) {
                let line = l.span().start().line;
                if !self.fact_strings.iter().any(|(at, seen)| *at == line && seen == &said) {
                    self.fact_strings.push((line, said));
                }
            }
            let at = (l.span().start().line, l.span().start().column);
            if self.tables_ok && (!self.in_trait_impl || !self.valued_strings.contains(&at)) {
                self.stray_strings.push(l.span().start().line);
            }
        }
        if exempt(l, self.allow_strings, self.allow_lexical) {
            return;
        }
        let at = (l.span().start().line, text(l));
        if self.in_const && !self.in_anon_const {
            self.named_lits.entry(self.current_const.clone()).or_default().push(at.clone());
            self.const_lits.push(at);
        } else {
            self.lits.push(at);
        }
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        let head = m
            .path
            .segments
            .last()
            .map(|s| crate::names::plain(&s.ident))
            .unwrap_or_default();
        let pulls = PULLS_FROM_OUTSIDE.contains(&head.as_str()) || self.splice_aliases.contains(&head);
        if pulls && !(self.allow_embed && head != "include") {
            self.splices.push((
                m.span().start().line,
                format!("{}! pulls a value in from outside the program, past every check", head),
            ));
        }

        if head == "cfg" && m.tokens.to_string() != "test" {
            self.conditionals
                .push((m.span().start().line, String::from("cfg! makes this true in some builds and not others, so rustc compiles one program while the checker reads another; only cfg(test) is allowed, because the gate runs cargo test")));
        }

        if self.in_anon_const {
            for name in idents_in(m.tokens.clone()) {
                self.check_refs.insert(name);
            }
        }
        let language = crate::because::LANGUAGE.contains(&head.as_str());
        let listed = CALLABLE_MACROS.contains(&head.as_str());
        let mut parsed_args = false;
        if listed && self.check_calls {
            for e in listed_arguments(m.tokens.clone()) {
                parsed_args = true;
                self.visit_expr(&e);
            }
        }
        let const_assert = head == "assert" && self.in_const;
        if self.check_calls && !language && !listed && !const_assert {
            self.foreign_macros.push((m.span().start().line, head.clone()));
        }

        let mut found = Vec::new();
        if !parsed_args {
            tokens(m.tokens.clone(), &mut found, self.allow_strings, self.allow_lexical);
        }
        match self.in_const && !self.in_anon_const {
            true => self.const_lits.extend(found),
            false => self.lits.extend(found),
        }
        splices_in(m.tokens.clone(), &mut self.splices, &self.splice_aliases);
        visit::visit_macro(self, m);
    }

    fn visit_expr(&mut self, e: &'ast syn::Expr) {
        let line = e.span().start().line;
        let mut opened = false;
        let spelled = all_literals(e) || repeated_name(e, &self.const_aliases);
        if !self.in_assembled && matches!(e, syn::Expr::Binary(_)) && spelled {
            match all_literals(e) {
                true => self.assembled.push(line),
                false => self.repeated.push(line),
            }
            self.in_assembled = true;
            opened = true;
        }
        let hit: Option<&'static str> = match e {
            syn::Expr::If(_) => Some("if"),
            syn::Expr::Match(m) => {
                let table = self.tables_ok && self.in_trait_impl && total_arms(m);
                if table && plain_scrutinee(&m.expr) {
                    None
                } else if table {
                    Some("a match on a call")
                } else {
                    Some("match")
                }
            }
            syn::Expr::Path(p) => {
                if let Some(one) = p.path.get_ident() {
                    if self.in_anon_const {
                        self.check_refs.insert(crate::names::plain(one));
                    } else if self.in_const {
                        self.const_refs.entry(self.current_const.clone()).or_default().push(crate::names::plain(one));
                    }
                }
                if let Some(name) = primitive_constant(p) {
                    if !self.in_const {
                        self.primitive_consts.push((line, name));
                    }
                } else if self.check_calls && !self.in_const {
                    let scope = Imports {
                        from_spec: &self.from_spec,
                        laundering: &self.laundering,
                        from_patterns: &self.from_patterns,
                        from_elsewhere: &self.from_elsewhere,
                        spec_types: &self.spec_types,
                        library_types: &self.library_types,
                        patterns_glob: self.patterns_glob,
                        in_spec: self.tables_ok,
                        elsewhere_glob: self.elsewhere_glob,
                    };
                    match foreign_constant(p, &scope) {
                        Some((name, Verdict::Ambiguous)) => self.ambiguous_roots.push((line, name)),
                        Some((name, _)) => self.primitive_consts.push((line, name)),
                        None => {}
                    }
                }
                None
            }
            syn::Expr::ForLoop(_) => Some("for"),
            syn::Expr::While(_) => Some("while"),
            syn::Expr::Loop(_) => Some("loop"),
            syn::Expr::Binary(_) => Some("operator"),
            syn::Expr::Unary(u) => match u.op {
                syn::UnOp::Deref(_) => None,
                _ => Some("operator"),
            },
            syn::Expr::MethodCall(_) => Some("method call"),
            syn::Expr::Cast(_) => Some("cast"),
            syn::Expr::Index(_) => Some("index"),
            syn::Expr::Range(_) => Some("range"),
            syn::Expr::Try(_) => Some("try"),
            syn::Expr::Closure(_) => Some("closure"),
            syn::Expr::Assign(_) => Some("assignment"),
            syn::Expr::Unsafe(_) => Some("an unsafe block"),
            syn::Expr::Await(_) => Some("an await"),
            syn::Expr::Async(_) => Some("an async block"),
            syn::Expr::Yield(_) => Some("a yield"),
            syn::Expr::Call(c) => {
                if self.check_calls {
                    self.check_call(c, line);
                }
                None
            }
            _ => None,
        };
        if let Some(k) = hit {
            let value_shaped = matches!(k, "operator" | "method call" | "range");
            if !(self.in_const && value_shaped) {
                self.logic.push((line, k));
            }
        }
        visit::visit_expr(self, e);
        if opened {
            self.in_assembled = false;
        }
    }
}
