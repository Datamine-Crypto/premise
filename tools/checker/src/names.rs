use syn::ext::IdentExt;

pub const QUOTE: char = 34u8 as char;

pub fn plain(i: &proc_macro2::Ident) -> String {
    i.unraw().to_string()
}

pub fn is(i: &proc_macro2::Ident, spelled: &str) -> bool {
    plain(i) == spelled
}
