use proc_macro::{TokenStream, TokenTree};

fn parts(input: TokenStream) -> (String, Vec<String>) {
    let mut name = String::new();
    let mut lits: Vec<String> = Vec::new();
    for t in input {
        match t {
            TokenTree::Ident(i) => {
                if name.is_empty() {
                    name = i.to_string();
                }
            }
            TokenTree::Literal(l) => lits.push(l.to_string()),
            _ => {}
        }
    }
    (name, lits)
}

fn nth(lits: &[String], at: usize) -> Option<String> {
    lits.get(at).cloned()
}

fn complain(what: &str) -> TokenStream {
    format!("compile_error!({:?});", what)
        .parse()
        .expect("complaint expands")
}

#[proc_macro]
pub fn because(input: TokenStream) -> TokenStream {
    let (names, lits) = idents(input);
    let name = match nth(&names, 0) {
        Some(v) => v,
        None => return complain("because! needs an item and a reason: because!(ITEM, \"why this value\")"),
    };
    let bare = name.trim_start_matches("r#").to_string();
    let why = match nth(&lits, 0) {
        Some(v) => v,
        None => return complain("because! needs an item and a reason: because!(ITEM, \"why this value\")"),
    };
    let mut cite = String::new();
    for (at, s) in names.iter().enumerate().skip(1) {
        cite.push_str(&format!(
            "#[allow(unused_imports)] use self::{} as _; #[allow(non_upper_case_globals)] const __from{}_{}: &str = stringify!({});",
            s, at, bare, s
        ));
    }
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __why_{}: &str = {}; #[allow(unused_imports)] use self::{} as _; {}",
        bare, why, name, cite
    );
    rendered.parse().expect("because expands")
}


#[proc_macro]
pub fn rejected(input: TokenStream) -> TokenStream {
    let (name, lits) = parts(input);
    let (alt, cost) = match (nth(&lits, 0), nth(&lits, 1)) {
        (Some(a), Some(c)) => (a, c),
        _ => {
            return complain(
                "rejected! needs an item, the alternative and its cost: rejected!(ITEM, \"alt\", \"cost\")",
            )
        }
    };
    let rendered = format!(
        "const _: (&str, &str, &str) = (stringify!({}), {}, {});",
        name, alt, cost
    );
    rendered.parse().expect("rejected expands")
}

fn idents(input: TokenStream) -> (Vec<String>, Vec<String>) {
    let mut names: Vec<String> = Vec::new();
    let mut lits: Vec<String> = Vec::new();
    for t in input {
        match t {
            TokenTree::Ident(i) => names.push(i.to_string()),
            TokenTree::Literal(l) => lits.push(l.to_string()),
            _ => {}
        }
    }
    (names, lits)
}

#[proc_macro]
pub fn supersedes(input: TokenStream) -> TokenStream {
    let (names, lits) = idents(input);
    let (now, was, why) = match (nth(&names, 0), nth(&names, 1), nth(&lits, 0)) {
        (Some(a), Some(b), Some(w)) => (a, b, w),
        _ => {
            return complain(
                "supersedes! needs the new item, the one it replaces, optionally the constant that says when, and why: supersedes!(NEW, OLD, TARIFF_DAY, \"the review that set it\")",
            )
        }
    };
    let mut uses = String::new();
    for (at, n) in names.iter().enumerate() {
        uses.push_str(&format!(
            "#[allow(unused_imports)] use self::{} as _; #[allow(non_upper_case_globals)] const __sup{}_{}: &str = stringify!({});",
            n, at, now, n
        ));
    }
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __was_{}: (&str, &str) = (stringify!({}), {}); {}",
        now, was, why, uses
    );
    rendered.parse().expect("supersedes expands")
}

fn tagged(input: TokenStream, tag: &str, usage: &str) -> TokenStream {
    let (names, lits) = idents(input);
    let (name, text) = match (nth(&names, 0), nth(&lits, 0)) {
        (Some(n), Some(t)) => (n, t),
        _ => return complain(usage),
    };
    let bare = name.trim_start_matches("r#").to_string();
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __{}_{}: &str = {}; #[allow(unused_imports)] use self::{} as _;",
        tag, bare, text, name
    );
    rendered.parse().expect("tagged expands")
}

#[proc_macro]
pub fn source(input: TokenStream) -> TokenStream {
    tagged(
        input,
        "source",
        "source! needs an item and what it is: source!(WearTest, \"hinge fatigue, forty doors\")",
    )
}


#[proc_macro]
pub fn provisional(input: TokenStream) -> TokenStream {
    tagged(
        input,
        "provisional",
        "provisional! needs an item and what would settle it: provisional!(COLLECT_DAYS, \"the branch has not told us its visit cycle\")",
    )
}
