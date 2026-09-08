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
    let (name, lits) = parts(input);
    let bare = name.trim_start_matches("r#");
    let why = match nth(&lits, 0) {
        Some(v) => v,
        None => return complain("because! needs an item and a reason: because!(ITEM, \"why this value\")"),
    };
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __why_{}: &str = {}; #[allow(unused_imports)] use self::{} as _;",
        bare, why, name
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
