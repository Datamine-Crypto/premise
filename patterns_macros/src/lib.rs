use proc_macro::{TokenStream, TokenTree};

enum Slot {
    Name(String),
    Text(String),
}

enum Want {
    Name,
    Text,
    Names,
}

struct Shape {
    call: &'static str,
    usage: &'static str,
    wants: &'static [Want],
}

const BECAUSE: Shape = Shape {
    call: "because",
    usage: "because!(ITEM, [CITE, ...], \"reason\")",
    wants: &[Want::Name, Want::Names, Want::Text],
};

const DECIDED: Shape = Shape {
    call: "decided",
    usage: "decided!(TYPE, TRAIT, \"reason\")",
    wants: &[Want::Name, Want::Name, Want::Text],
};

const SOURCE: Shape = Shape {
    call: "source",
    usage: "source!(ITEM, \"what it is\")",
    wants: &[Want::Name, Want::Text],
};

const PROVISIONAL: Shape = Shape {
    call: "provisional",
    usage: "provisional!(ITEM, [CITE, ...], \"what would settle it\")",
    wants: &[Want::Name, Want::Names, Want::Text],
};

const REJECTED: Shape = Shape {
    call: "rejected",
    usage: "rejected!(ITEM, \"alternative\", \"cost\")",
    wants: &[Want::Name, Want::Text, Want::Text],
};

const SUPERSEDES: Shape = Shape {
    call: "supersedes",
    usage: "supersedes!(NEW, OLD, WHEN, \"why\")",
    wants: &[Want::Name, Want::Name, Want::Name, Want::Text],
};

fn is_keyword(spelled: &str) -> bool {
    matches!(spelled, "self" | "super" | "crate" | "Self")
}

fn complain(what: &str) -> TokenStream {
    format!("compile_error!({:?});", what)
        .parse()
        .expect("complaint expands")
}

fn refuse(shape: &Shape, problem: &str) -> TokenStream {
    complain(&format!("{}! {}; usage: {}", shape.call, problem, shape.usage))
}

fn is_text(lit: &str) -> bool {
    lit.starts_with("\"") || lit.starts_with("r\"") || lit.starts_with("r#")
}

fn kind(text: bool) -> &'static str {
    if text {
        "a string"
    } else {
        "an item name"
    }
}

fn split(input: TokenStream, shape: &Shape) -> Result<Vec<Slot>, TokenStream> {
    let mut out: Vec<Slot> = Vec::new();
    let mut open: Option<Slot> = None;
    let mut angled = 0usize;
    for t in input {
        if angled > 0 {
            match (&t, &mut open) {
                (TokenTree::Punct(p), Some(Slot::Name(n))) if p.to_string() == ">" => {
                    angled -= 1;
                    n.push_str(&p.to_string());
                }
                (TokenTree::Punct(p), Some(Slot::Name(n))) if p.to_string() == "<" => {
                    angled += 1;
                    n.push_str(&p.to_string());
                }
                (TokenTree::Ident(i), Some(Slot::Name(n))) => n.push_str(&i.to_string()),
                (TokenTree::Punct(p), Some(Slot::Name(n))) if p.to_string() == "," => n.push_str(&p.to_string()),
                _ => return Err(refuse(shape, "takes item names, commas and string literals only")),
            }
            continue;
        }
        if let (TokenTree::Punct(p), Some(Slot::Name(n))) = (&t, &mut open) {
            if p.to_string() == "<" {
                angled += 1;
                n.push_str(&p.to_string());
                continue;
            }
        }
        let next = match t {
            TokenTree::Ident(i) => {
                let spelled = i.to_string();
                if is_keyword(&spelled) {
                    return Err(refuse(shape, "takes item names, not keywords; self, super, crate and Self are paths"));
                }
                Slot::Name(spelled)
            }
            TokenTree::Literal(l) => {
                let s = l.to_string();
                if !is_text(&s) {
                    return Err(refuse(
                        shape,
                        "takes string literals only; a number or a char is not a sentence",
                    ));
                }
                Slot::Text(s)
            }
            TokenTree::Punct(p) if p.to_string() == "," => {
                match open.take() {
                    Some(s) => out.push(s),
                    None => return Err(refuse(shape, "found an empty slot before a comma")),
                }
                continue;
            }
            TokenTree::Punct(p) if p.to_string() == ":" => {
                return Err(refuse(
                    shape,
                    "takes item names, not paths; bring the item into scope and name it",
                ))
            }
            _ => {
                return Err(refuse(
                    shape,
                    "takes item names, commas and string literals only",
                ))
            }
        };
        if open.is_some() {
            return Err(refuse(shape, "found two slots with no comma between them"));
        }
        open = Some(next);
    }
    if let Some(s) = open {
        out.push(s);
    }
    Ok(out)
}

fn fit(slots: &[Slot], shape: &Shape) -> Option<TokenStream> {
    let stretch = shape.wants.iter().any(|w| matches!(w, Want::Names));
    let fixed = shape.wants.len() - usize::from(stretch);
    if slots.len() < fixed {
        return Some(refuse(shape, "is missing a slot"));
    }
    if !stretch && slots.len() > fixed {
        return Some(refuse(shape, "has more slots than its shape takes"));
    }
    let extra = slots.len() - fixed;
    let mut laid: Vec<bool> = Vec::new();
    for w in shape.wants {
        match w {
            Want::Name => laid.push(false),
            Want::Text => laid.push(true),
            Want::Names => laid.extend(std::iter::repeat_n(false, extra)),
        }
    }
    let last = slots.len() - 1;
    let order = std::iter::once(last).chain(0..last);
    for at in order {
        let (slot, wants_text) = (&slots[at], laid[at]);
        let got_text = matches!(slot, Slot::Text(_));
        if got_text != wants_text {
            return Some(refuse(
                shape,
                &format!(
                    "slot {} wants {}, got {}",
                    at + 1,
                    kind(wants_text),
                    kind(got_text)
                ),
            ));
        }
    }
    None
}

fn parse(input: TokenStream, shape: &Shape) -> Result<Vec<Slot>, TokenStream> {
    let slots = split(input, shape)?;
    match fit(&slots, shape) {
        Some(err) => Err(err),
        None => Ok(slots),
    }
}

fn spelled(slot: &Slot) -> &str {
    match slot {
        Slot::Name(s) => s,
        Slot::Text(s) => s,
    }
}

fn bare(name: &str) -> &str {
    name.trim_start_matches("r#")
}

fn reasoned(input: TokenStream, shape: &Shape, tag: &str, from: &str) -> TokenStream {
    let slots = match parse(input, shape) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let item = spelled(&slots[0]);
    let text = spelled(&slots[slots.len() - 1]);
    let mut cite = String::new();
    for (at, c) in slots[1..slots.len() - 1].iter().enumerate() {
        let name = spelled(c);
        cite.push_str(&format!(
            "#[allow(unused_imports)] use self::{} as _; #[allow(non_upper_case_globals)] const __{}{}_{}: &str = stringify!({});",
            name,
            from,
            at + 1,
            bare(item),
            name
        ));
    }
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __{}_{}: &str = {}; #[allow(unused_imports)] use self::{} as _; {}",
        tag,
        bare(item),
        text,
        item,
        cite
    );
    rendered.parse().expect("reason expands")
}

#[proc_macro]
pub fn because(input: TokenStream) -> TokenStream {
    reasoned(input, &BECAUSE, "why", "from")
}

#[proc_macro]
pub fn decided(input: TokenStream) -> TokenStream {
    let slots = match parse(input, &DECIDED) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let (ty, tr, why) = match slots.as_slice() {
        [a, b, c] => (spelled(a), spelled(b), spelled(c)),
        _ => return refuse(&DECIDED, "is missing a slot"),
    };
    let parts: Vec<&str> = tr.split(|c: char| !c.is_alphanumeric() && c.to_string() != "_").filter(|s| !s.is_empty()).collect();
    let key: Vec<String> = parts.iter().map(|p| bare(p).to_string()).collect();
    let mut uses = String::new();
    for p in &parts {
        uses.push_str(&format!("#[allow(unused_imports)] use self::{} as _; ", p));
    }
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __why_{}_{}: &str = {}; #[allow(unused_imports)] use self::{} as _; {}",
        bare(ty), key.join("_"), why, ty, uses
    );
    rendered.parse().expect("decided expands")
}

#[proc_macro]
pub fn source(input: TokenStream) -> TokenStream {
    reasoned(input, &SOURCE, "source", "sourcefrom")
}

#[proc_macro]
pub fn provisional(input: TokenStream) -> TokenStream {
    reasoned(input, &PROVISIONAL, "provisional", "provisionalfrom")
}

#[proc_macro]
pub fn rejected(input: TokenStream) -> TokenStream {
    let slots = match parse(input, &REJECTED) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let rendered = match slots.as_slice() {
        [item, alt, cost] => format!(
            "const _: (&str, &str, &str) = (stringify!({}), {}, {}); #[allow(unused_imports)] use self::{} as _;",
            spelled(item),
            spelled(alt),
            spelled(cost),
            spelled(item)
        ),
        _ => return refuse(&REJECTED, "is missing a slot"),
    };
    rendered.parse().expect("rejected expands")
}

#[proc_macro]
pub fn supersedes(input: TokenStream) -> TokenStream {
    let slots = match parse(input, &SUPERSEDES) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let now = spelled(&slots[0]);
    let why = spelled(&slots[slots.len() - 1]);
    let mut uses = String::new();
    for (at, n) in slots[..slots.len() - 1].iter().enumerate() {
        let name = spelled(n);
        uses.push_str(&format!(
            "#[allow(unused_imports)] use self::{} as _; #[allow(non_upper_case_globals)] const __sup{}_{}: &str = stringify!({});",
            name,
            at,
            bare(now),
            name
        ));
    }
    let rendered = format!(
        "#[allow(non_upper_case_globals)] const __was_{}: (&str, &str) = (stringify!({}), {}); {}",
        bare(now),
        spelled(&slots[1]),
        why,
        uses
    );
    rendered.parse().expect("supersedes expands")
}

#[proc_macro_derive(Record)]
pub fn record(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &parsed.ident;
    let mut generics = parsed.generics.clone();
    for param in generics.params.iter_mut() {
        if let syn::GenericParam::Type(t) = param {
            t.bounds.push(syn::parse_quote!(patterns::record::Fielded));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = match &parsed.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(named) => {
                let names: Vec<&syn::Ident> = named.named.iter().map(|f| f.ident.as_ref().expect("named")).collect();
                let texts: Vec<String> = names.iter().map(|n| n.to_string()).collect();
                quote::quote! {
                    fn field(&self) -> patterns::record::Field {
                        patterns::record::Field::Table(vec![#((String::from(#texts), patterns::record::Fielded::field(&self.#names))),*])
                    }
                    fn refielded(field: &patterns::record::Field) -> Option<Self> {
                        let table = patterns::record::table_of(field)?;
                        Some(Self { #(#names: patterns::record::Fielded::refielded(patterns::record::looked_field(table, #texts))?),* })
                    }
                }
            }
            _ => return complain("Record derives on structs with named fields and on enums of unit variants only"),
        },
        syn::Data::Enum(e) => {
            if e.variants.iter().any(|v| !matches!(v.fields, syn::Fields::Unit)) {
                return complain("Record derives on enums of unit variants only");
            }
            let variants: Vec<&syn::Ident> = e.variants.iter().map(|v| &v.ident).collect();
            let texts: Vec<String> = variants.iter().map(|v| v.to_string()).collect();
            quote::quote! {
                fn field(&self) -> patterns::record::Field {
                    match self {
                        #(Self::#variants => patterns::record::Field::Text(String::from(#texts))),*
                    }
                }
                fn refielded(field: &patterns::record::Field) -> Option<Self> {
                    let text = patterns::record::text_of_field(field)?;
                    match text {
                        #(#texts => Some(Self::#variants),)*
                        _ => None,
                    }
                }
            }
        }
        syn::Data::Union(_) => return complain("Record does not derive on unions"),
    };
    let out = quote::quote! {
        impl #impl_generics patterns::record::Fielded for #name #ty_generics #where_clause {
            #body
        }
    };
    out.into()
}
