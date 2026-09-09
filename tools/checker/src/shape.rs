use proc_macro2::{Delimiter, TokenTree};
use quote::ToTokens;
use syn::visit::{self, Visit};
use std::collections::HashMap;

const DOT: char = 46u8 as char;
const COLON: char = 58u8 as char;
pub const MARK: char = 64u8 as char;
const LENGTH_GAP_CUTOFF: usize = 8;
patterns_macros::because!(
    LENGTH_GAP_CUTOFF,
    "the length difference beyond which the pair is a different function"
);

pub struct Profile {
    pub tokens: Vec<String>,
    pub composed: bool,
    pub callees: std::collections::HashSet<String>,
}

impl Profile {
    pub fn key(&self) -> String {
        self.tokens.join(" ")
    }
}

struct State {
    names: HashMap<String, usize>,
    after_selector: bool,
    prev_colon: bool,
}

fn walk(ts: proc_macro2::TokenStream, st: &mut State, out: &mut Vec<String>) {
    let items: Vec<TokenTree> = ts.into_iter().collect();
    for (at, t) in items.iter().enumerate() {
        let callee = matches!(
            items.get(at + 1),
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis
        );
        match t {
            TokenTree::Ident(id) => {
                if st.after_selector || callee {
                    out.push(format!("{}{}", MARK, crate::names::plain(id)));
                } else {
                    let s = crate::names::plain(id);
                    let next = st.names.len();
                    let idx = *st.names.entry(s).or_insert(next);
                    out.push(format!("i{}", idx));
                }
                st.after_selector = false;
                st.prev_colon = false;
            }
            TokenTree::Literal(_) => {
                out.push(String::from("L"));
                st.after_selector = false;
                st.prev_colon = false;
            }
            TokenTree::Punct(p) => {
                let c = p.as_char();
                out.push(c.to_string());
                let colon = c == COLON;
                st.after_selector = c == DOT || (colon && st.prev_colon);
                st.prev_colon = colon;
            }
            TokenTree::Group(g) => {
                out.push(String::from(match g.delimiter() {
                    Delimiter::Parenthesis => "(",
                    Delimiter::Brace => "{",
                    Delimiter::Bracket => "[",
                    Delimiter::None => "N",
                }));
                st.after_selector = false;
                st.prev_colon = false;
                walk(g.stream(), st, out);
                out.push(String::from("}"));
                st.after_selector = false;
                st.prev_colon = false;
            }
        }
    }
}

struct HasCall {
    found: bool,
    names: std::collections::HashSet<String>,
}

impl<'ast> Visit<'ast> for HasCall {
    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        self.found = true;
        if let syn::Expr::Path(p) = &*c.func {
            if let Some(seg) = p.path.segments.last() {
                self.names.insert(crate::names::plain(&seg.ident));
            }
        }
        visit::visit_expr_call(self, c);
    }

    fn visit_expr_method_call(&mut self, c: &'ast syn::ExprMethodCall) {
        self.found = true;
        self.names.insert(crate::names::plain(&c.method));
        visit::visit_expr_method_call(self, c);
    }
}

fn calls(block: &syn::Block) -> HasCall {
    let mut h = HasCall {
        found: false,
        names: std::collections::HashSet::new(),
    };
    h.visit_block(block);
    h
}

pub fn distance(a: &[String], b: &[String]) -> usize {
    let far = if a.len() > b.len() {
        a.len() - b.len()
    } else {
        b.len() - a.len()
    };
    if far > LENGTH_GAP_CUTOFF {
        return far;
    }
    let n = b.len();
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut cur = vec![0usize; n + 1];
    for (i, x) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, y) in b.iter().enumerate() {
            let cost = usize::from(x != y);
            let sub = prev[j] + cost;
            let ins = cur[j] + 1;
            let del = prev[j + 1] + 1;
            cur[j + 1] = sub.min(ins).min(del);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[n]
}

pub const OWN_NAME: &str = "f";
patterns_macros::because!(
    OWN_NAME,
    "every function is profiled under one fixed name, so two bodies that differ only in what they are called compare as the same shape"
);

fn bound_name(p: &syn::Pat) -> Option<proc_macro2::Ident> {
    match p {
        syn::Pat::Ident(i) if i.subpat.is_none() => Some(i.ident.clone()),
        syn::Pat::Type(t) => bound_name(&t.pat),
        syn::Pat::Paren(inner) => bound_name(&inner.pat),
        _ => None,
    }
}

fn unwrapped_tail(stmts: &[syn::Stmt]) -> Option<Vec<syn::Stmt>> {
    let last = stmts.last()?;
    if let syn::Stmt::Expr(syn::Expr::Block(inner), None) = last {
        if inner.label.is_none() {
            let mut out = stmts[..stmts.len() - 1].to_vec();
            out.extend(inner.block.stmts.iter().cloned());
            return Some(out);
        }
    }
    let unwrapped = match last {
        syn::Stmt::Expr(syn::Expr::Paren(p), None) => Some((*p.expr).clone()),
        syn::Stmt::Expr(syn::Expr::Return(r), _) => r.expr.as_ref().map(|e| (**e).clone()),
        _ => None,
    };
    if let Some(e) = unwrapped {
        let mut out = stmts[..stmts.len() - 1].to_vec();
        out.push(syn::Stmt::Expr(e, None));
        return Some(out);
    }
    if stmts.len() < 2 {
        return None;
    }
    let before = &stmts[stmts.len() - 2];
    let (name, init) = match before {
        syn::Stmt::Local(l) => match (&l.init, bound_name(&l.pat)) {
            (Some(init), Some(name)) if init.diverge.is_none() => (name, (*init.expr).clone()),
            _ => return None,
        },
        _ => return None,
    };
    let returned = match last {
        syn::Stmt::Expr(syn::Expr::Path(p), None) => p.path.is_ident(&name),
        _ => false,
    };
    if !returned {
        return None;
    }
    let mut out = stmts[..stmts.len() - 2].to_vec();
    out.push(syn::Stmt::Expr(init, None));
    Some(out)
}

struct Flatten;

fn chosen_arm(c: &syn::ExprCall) -> Option<syn::Expr> {
    let callee = match &*c.func {
        syn::Expr::Path(p) => p.path.segments.last().map(|s| crate::names::plain(&s.ident))?,
        _ => return None,
    };
    if callee != "either" {
        return None;
    }
    let args: Vec<&syn::Expr> = c.args.iter().collect();
    let (first, second) = match args.as_slice() {
        [_, first, second] => (*first, *second),
        _ => return None,
    };
    match args[0] {
        syn::Expr::Lit(l) => match &l.lit {
            syn::Lit::Bool(b) if b.value => Some(first.clone()),
            syn::Lit::Bool(_) => Some(second.clone()),
            _ => None,
        },
        _ => match first.to_token_stream().to_string() == second.to_token_stream().to_string() {
            true => Some(first.clone()),
            false => None,
        },
    }
}

impl syn::fold::Fold for Flatten {
    fn fold_expr(&mut self, e: syn::Expr) -> syn::Expr {
        let e = syn::fold::fold_expr(self, e);
        match &e {
            syn::Expr::Call(c) => chosen_arm(c).unwrap_or(e),
            _ => e,
        }
    }

    fn fold_block(&mut self, b: syn::Block) -> syn::Block {
        let mut b = syn::fold::fold_block(self, b);
        while let Some(stmts) = unwrapped_tail(&b.stmts) {
            b.stmts = stmts;
        }
        b
    }
}

pub fn flattened(block: &syn::Block) -> syn::Block {
    syn::fold::Fold::fold_block(&mut Flatten, block.clone())
}

pub fn profile(sig: &syn::Signature, block: &syn::Block) -> Profile {
    let block = &flattened(block);
    let mut st = State {
        names: HashMap::new(),
        after_selector: false,
        prev_colon: false,
    };
    let mut tokens = Vec::new();
    let mut unnamed = sig.clone();
    unnamed.ident = proc_macro2::Ident::new(OWN_NAME, sig.ident.span());
    walk(unnamed.to_token_stream(), &mut st, &mut tokens);
    walk(block.to_token_stream(), &mut st, &mut tokens);
    let found = calls(block);
    Profile {
        tokens,
        composed: found.found,
        callees: found.names,
    }
}

pub fn drift(a: &Profile, b: &Profile) -> usize {
    distance(&a.tokens, &b.tokens)
}

fn is_name(t: &str) -> bool {
    if t.starts_with(MARK) {
        return true;
    }
    let mut chars = t.chars();
    if chars.next() != Some('i') {
        return false;
    }
    let rest: String = chars.collect();
    !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
}

pub fn only_names_differ(a: &Profile, b: &Profile) -> bool {
    let mut tally: HashMap<String, i32> = HashMap::new();
    for t in &a.tokens {
        *tally.entry(t.clone()).or_insert(0) += 1;
    }
    for t in &b.tokens {
        *tally.entry(t.clone()).or_insert(0) -= 1;
    }
    tally
        .iter()
        .filter(|(_, n)| **n != 0)
        .all(|(t, _)| is_name(t))
}

pub fn set_apart(a: &[String], b: &[String]) -> usize {
    let left: std::collections::HashSet<&String> = a.iter().collect();
    let right: std::collections::HashSet<&String> = b.iter().collect();
    left.difference(&right).count() + right.difference(&left).count()
}

pub fn differing(a: &[String], b: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            i += 1;
            j += 1;
            continue;
        }
        let skipped_a = i + 1 < a.len() && a[i + 1] == b[j];
        let skipped_b = j + 1 < b.len() && a[i] == b[j + 1];
        if skipped_a && !skipped_b {
            out.push((a[i].clone(), String::new()));
            i += 1;
            continue;
        }
        if skipped_b && !skipped_a {
            out.push((String::new(), b[j].clone()));
            j += 1;
            continue;
        }
        out.push((a[i].clone(), b[j].clone()));
        i += 1;
        j += 1;
    }
    for left in a.iter().skip(i) {
        out.push((left.clone(), String::new()));
    }
    for right in b.iter().skip(j) {
        out.push((String::new(), right.clone()));
    }
    out
}

fn legible(t: &str) -> String {
    let rest = match t.strip_prefix("i") {
        Some(r) => r,
        None => return t.to_string(),
    };
    match rest.chars().all(|c| c.is_ascii_digit()) && !rest.is_empty() {
        true => format!("the value bound {}", ordinal(rest)),
        false => t.to_string(),
    }
}

fn ordinal(digits: &str) -> String {
    let at: usize = digits.parse().unwrap_or_default();
    format!("{}th", at + 1)
}

pub fn shown(pairs: &[(String, String)]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (was, now) in pairs {
        let text = match (was.is_empty(), now.is_empty()) {
            (false, false) => format!("{} not {}", legible(now), legible(was)),
            (true, false) => format!("{} added", legible(now)),
            (false, true) => format!("{} dropped", legible(was)),
            _ => continue,
        };
        parts.push(text);
    }
    parts.join(", ")
}
