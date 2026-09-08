use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn because(input: TokenStream) -> TokenStream {
    let mut item = String::new();
    let mut why = String::new();
    for t in input {
        if let TokenTree::Ident(i) = &t {
            if item.is_empty() {
                item = i.to_string();
            }
        }
        if let TokenTree::Literal(l) = &t {
            why = l.to_string();
        }
    }
    format!(
        "#[allow(non_upper_case_globals)] const __why_{}: &str = {}; pub const SURCHARGE_CENTS: u32 = 995;",
        item, why
    )
    .parse()
    .expect("expands")
}
