fn gated(attrs: &[syn::Attribute]) -> bool {
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

pub fn owner_of(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| crate::names::plain(&s.ident))
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn gather(items: &[syn::Item], prefix: &str, out: &mut Vec<String>) {
    for it in items {
        match it {
            syn::Item::Const(c) if is_public(&c.vis) && !gated(&c.attrs) => {
                out.push(crate::names::plain(&c.ident));
            }
            syn::Item::Static(s) if is_public(&s.vis) && !gated(&s.attrs) => {
                out.push(crate::names::plain(&s.ident));
            }
            syn::Item::Enum(e) if is_public(&e.vis) && !gated(&e.attrs) => {
                out.push(format!("{}{}", prefix, crate::names::plain(&e.ident)));
            }
            syn::Item::Trait(t) if is_public(&t.vis) && !gated(&t.attrs) => {
                let owner = crate::names::plain(&t.ident);
                for item in &t.items {
                    if let syn::TraitItem::Const(c) = item {
                        if c.default.is_some() {
                            out.push(format!("{}{}_{}", prefix, owner, crate::names::plain(&c.ident)));
                        }
                    }
                }
            }
            syn::Item::Impl(im) if !gated(&im.attrs) => {
                let owner = owner_of(&im.self_ty);
                if owner.is_empty() || bound_here(im, &owner) {
                    continue;
                }
                for item in &im.items {
                    if let syn::ImplItem::Const(c) = item {
                        out.push(format!("{}{}_{}", prefix, owner, crate::names::plain(&c.ident)));
                    }
                }
            }
            syn::Item::Mod(m) if is_public(&m.vis) && !gated(&m.attrs) => {
                if let Some((_, inner)) = &m.content {
                    gather(inner, prefix, out);
                }
            }
            _ => {}
        }
    }
}

pub fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

pub fn public_in(file: &syn::File, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    gather(&file.items, prefix, &mut out);
    out.sort();
    out.dedup();
    out
}

pub fn bound_here(im: &syn::ItemImpl, head: &str) -> bool {
    im.generics.params.iter().any(|p| match p {
        syn::GenericParam::Type(t) => crate::names::plain(&t.ident) == head,
        _ => false,
    })
}


