use checker::shape;

const CMPS: &[(&str, &str)] = &[
    ("above", ">"), ("below", "<"), ("atleast", ">="), ("atmost", "<="),
    ("equal", "=="), ("unlike", "!="),
];
const OPS: &[(&str, &str)] = &[
    ("sum", "+"), ("gap", "-"), ("scale", "*"), ("share", "/"), ("rest", "%"),
];
const PICKS: &[&str] = &["min", "max", "clamp_low", "clamp_high", "wrap", "saturate"];
const SEQS: &[&str] = &["first", "last", "single", "head", "tail", "sole"];
const FOLDS: &[&str] = &["total", "product", "widest", "narrowest", "spread", "count_of"];

fn guards(out: &mut Vec<String>) {
    for (cn, c) in CMPS {
        for (on, o) in OPS {
            out.push(format!(
                "pub fn {}_{}(v: u32, hi: u32, step: u32) -> u32 {{ if v {} hi {{ v {} step }} else {{ hi {} step }} }}",
                cn, on, c, o, o
            ));
        }
    }
}

fn picks(out: &mut Vec<String>) {
    for p in PICKS {
        for (cn, c) in CMPS {
            out.push(format!(
                "pub fn {}_{}(v: u32, hi: u32, lo: u32) -> u32 {{ if v {} hi {{ {}(v, lo) }} else {{ {}(hi, lo) }} }}",
                p, cn, c, p, p
            ));
        }
    }
}

fn seqs(out: &mut Vec<String>) {
    for s in SEQS {
        for f in FOLDS {
            out.push(format!(
                "pub fn {}_{}(xs: &[u32], hi: u32) -> u32 {{ let held = xs.iter().{}(); let seed = {}(xs); match held {{ Some(v) => seed + v + hi, None => seed }} }}",
                s, f, s, f
            ));
        }
    }
}

fn folds(out: &mut Vec<String>) {
    for f in FOLDS {
        for (on, o) in OPS {
            out.push(format!(
                "pub fn {}_{}(xs: &[u32], seed: u32, hi: u32) -> u32 {{ let mut acc = seed; for x in xs {{ acc = {}(acc, *x {} hi); }} acc }}",
                f, on, f, o
            ));
        }
    }
}

const GATES: &[&str] = &["allow", "deny", "defer", "escalate", "retry", "drop_it"];

fn gates(out: &mut Vec<String>) {
    for g in GATES {
        for (cn, c) in CMPS {
            for (on, o) in OPS.iter().take(3) {
                out.push(format!(
                    "pub fn {}_{}_{}(v: u32, hi: u32, lo: u32) -> u32 {{ let bar = lo {} hi; if v {} bar {{ {}(v, bar) }} else {{ {}(bar, v) }} }}",
                    g, cn, on, o, c, g, g
                ));
            }
        }
    }
}


const MIN_SHAPE_TOKENS: usize = 12;
const MAX_DRIFT: usize = 2;

fn flagged_pairs(src: &str) -> (usize, usize, Vec<String>) {
    let parsed = syn::parse_file(src).expect("generated library parses");
    let mut shapes: Vec<(shape::Profile, String)> = Vec::new();
    for item in &parsed.items {
        if let syn::Item::Fn(f) = item {
            shapes.push((shape::profile(&f.sig, &f.block), f.sig.ident.to_string()));
        }
    }
    let mut flagged = Vec::new();
    let mut pairs = 0usize;
    for a in 0..shapes.len() {
        for b in (a + 1)..shapes.len() {
            pairs += 1;
            let (pa, na) = &shapes[a];
            let (pb, nb) = &shapes[b];
            if !pa.composed || !pb.composed {
                continue;
            }
            if pa.tokens.len().min(pb.tokens.len()) < MIN_SHAPE_TOKENS {
                continue;
            }
            let apart = shape::drift(pa, pb);
            if apart == 0 || apart > MAX_DRIFT {
                continue;
            }
            if !shape::only_names_differ(pa, pb) {
                continue;
            }
            if pa.callees.is_disjoint(&pb.callees) {
                continue;
            }
            flagged.push(format!("{} ~ {}", na, nb));
        }
    }
    (shapes.len(), pairs, flagged)
}

#[test]
fn patterns_that_differ_by_an_operator_never_collide_at_scale() {
    let mut out = Vec::new();
    guards(&mut out);
    picks(&mut out);
    folds(&mut out);
    gates(&mut out);
    let (n, pairs, flagged) = flagged_pairs(&format!("{}\n", out.join("\n")));
    assert!(n >= 200, "wanted 200 patterns, built {}", n);
    assert!(
        flagged.is_empty(),
        "{} of {} pairs across {} patterns collided; the rule blocks correct work at scale: {:?}",
        flagged.len(),
        pairs,
        n,
        flagged.iter().take(10).collect::<Vec<&String>>()
    );
}

#[test]
fn patterns_that_differ_only_by_a_called_name_do_collide() {
    let mut out = Vec::new();
    seqs(&mut out);
    let (_, _, flagged) = flagged_pairs(&format!("{}\n", out.join("\n")));
    assert!(
        !flagged.is_empty(),
        "a family differing only by a callee name went unflagged, so the rule has stopped working"
    );
}
