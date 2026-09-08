use quote::ToTokens;
use std::path::PathBuf;

fn tidy(s: String) -> String {
    s.replace(" ,", ",")
        .replace(" ;", ";")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" (", "(")
        .replace("( ", "(")
        .replace(" )", ")")
        .replace(" ::", "::")
        .replace(":: ", "::")
}

fn reexported(t: &syn::UseTree, out: &mut Vec<String>) {
    match t {
        syn::UseTree::Path(p) => {
            if p.ident == "patterns_macros" {
                reexported(&p.tree, out);
            }
        }
        syn::UseTree::Name(n) => out.push(n.ident.to_string()),
        syn::UseTree::Rename(r) => out.push(r.rename.to_string()),
        syn::UseTree::Group(g) => {
            for i in &g.items {
                reexported(i, out);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

fn emit(items: &[syn::Item], lines: &mut Vec<String>) {
    for it in items {
        match it {
            syn::Item::Fn(f) if checker::facts::is_public(&f.vis) => {
                lines.push(format!("  {}", tidy(f.sig.to_token_stream().to_string())));
            }
            syn::Item::Trait(t) if checker::facts::is_public(&t.vis) => {
                let mut parts: Vec<String> = Vec::new();
                for i in &t.items {
                    match i {
                        syn::TraitItem::Fn(f) => {
                            parts.push(tidy(f.sig.to_token_stream().to_string()))
                        }
                        syn::TraitItem::Const(c) => parts.push(format!("const {}", c.ident)),
                        syn::TraitItem::Type(ty) => parts.push(format!("type {}", ty.ident)),
                        _ => {}
                    }
                }
                let generics = tidy(t.generics.to_token_stream().to_string());
                lines.push(format!("  trait {}{} {{ {} }}", t.ident, generics, parts.join("; ")));
            }
            syn::Item::Macro(m) => {
                if let Some(id) = &m.ident {
                    lines.push(format!("  macro {}!", id));
                }
            }
            syn::Item::Use(u) if checker::facts::is_public(&u.vis) => {
                let mut names = Vec::new();
                reexported(&u.tree, &mut names);
                for name in names {
                    lines.push(format!("  macro {}!", name));
                }
            }
            syn::Item::Const(c) if checker::facts::is_public(&c.vis) => {
                lines.push(format!("  const {}: {}", c.ident, tidy(c.ty.to_token_stream().to_string())));
            }
            syn::Item::Type(t) if checker::facts::is_public(&t.vis) => {
                let generics = tidy(t.generics.to_token_stream().to_string());
                lines.push(format!("  type {}{} = {}", t.ident, generics, tidy(t.ty.to_token_stream().to_string())));
            }
            syn::Item::Struct(s) if checker::facts::is_public(&s.vis) => {
                lines.push(format!("  struct {}", tidy(s.ident.to_string())));
            }
            syn::Item::Enum(e) if checker::facts::is_public(&e.vis) => {
                let names: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();
                lines.push(format!("  enum {} {{ {} }}", e.ident, names.join(", ")));
            }
            syn::Item::Impl(im) => {
                let target = match &*im.self_ty {
                    syn::Type::Path(p) => p
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default(),
                    _ => String::new(),
                };
                for sub in &im.items {
                    if let syn::ImplItem::Fn(f) = sub {
                        let open = im.trait_.is_some() || checker::facts::is_public(&f.vis);
                        if open {
                            let shown = tidy(f.sig.to_token_stream().to_string());
                            lines.push(format!(
                                "  {}::{}",
                                target,
                                shown.trim_start_matches("fn ")
                            ));
                        }
                    }
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    emit(inner, lines);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let looking: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with('.') && !a.contains('/') && !a.contains(':'))
        .map(|a| a.to_lowercase())
        .collect();
    let root = args
        .iter()
        .find(|a| a.starts_with('.') || a.contains('/') || a.contains(':'))
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("cwd"));
    let zones = checker::config::Zones::load(&root);
    let mut files = checker::compiled_sources(&root, checker::zone::Zone::Patterns);
    files.extend(zones.library_sources());
    let mut total = 0usize;
    let mut shown = 0usize;
    for f in &files {
        let src = std::fs::read_to_string(f).unwrap_or_default();
        let parsed = match syn::parse_file(&src) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let mut lines = Vec::new();
        emit(&parsed.items, &mut lines);
        total += lines.len();
        let file = checker::module_path_in(&zones, &root, f);
        let keep: Vec<&String> = lines
            .iter()
            .filter(|l| {
                looking.is_empty()
                    || looking
                        .iter()
                        .all(|w| l.to_lowercase().contains(w) || file.to_lowercase().contains(w))
            })
            .collect();
        if keep.is_empty() {
            continue;
        }
        println!("{}", file);
        for l in &keep {
            println!("{}", l);
        }
        shown += keep.len();
        println!();
    }
    if looking.is_empty() {
        println!("{} public items: functions, traits and their methods", total);
        return;
    }
    println!(
        "{} of {} public items match {}",
        shown,
        total,
        looking.join(" and ")
    );
}
