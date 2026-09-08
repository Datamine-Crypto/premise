use std::io::Write;
use std::path::{Path, PathBuf};

const ORIENT_USAGE: &str = "usage: orient [<workspace root>] [--answers | --quiz]\n       orient prints orientation questions generated from the spec\n       --answers prints each question with its answer\n       --quiz asks them one at a time, times each, and appends the run to premise.orient";
patterns_macros::because!(
    ORIENT_USAGE,
    "the whole command line, printed for any argument the tool does not know, so a mistyped flag is refused rather than silently run as a plain listing"
);

const ORIENT_BAD_ARGUMENT: i32 = 2;
patterns_macros::because!(
    ORIENT_BAD_ARGUMENT,
    "the exit status for a command line the tool could not read, matching the checker and the gate so a script can tell a refused flag from a real result"
);

const LOG: &str = "premise.orient";
patterns_macros::because!(
    LOG,
    "one tracked file beside the census, holding one line per timed run, so orientation cost is visible in git history rather than in anyone's memory"
);

const REASON_MACROS: &[&str] = &["because", "provisional"];
patterns_macros::because!(
    REASON_MACROS,
    "the two macros whose text answers why a value is what it is; a source describes a document and a rejection an alternative, and neither is what a newcomer is asked to find"
);

const MILLIS: u128 = 1000;
patterns_macros::because!(
    MILLIS,
    "milliseconds in a second, used to print a duration a person can read against a log line that records whole seconds"
);

struct Question {
    ask: String,
    answer: String,
}

fn context_of(rel: &str) -> String {
    checker::boundary::context_of(rel).unwrap_or_else(|| String::from("the crate root"))
}

fn text_of(tokens: proc_macro2::TokenStream) -> (Option<String>, Vec<String>) {
    let mut item = None;
    let mut said = Vec::new();
    for t in tokens {
        match t {
            proc_macro2::TokenTree::Ident(id) if item.is_none() => {
                item = Some(checker::names::plain(&id));
            }
            proc_macro2::TokenTree::Literal(l) => {
                said.push(l.to_string().trim_matches(checker::names::QUOTE).to_string());
            }
            _ => {}
        }
    }
    (item, said)
}

fn gather(items: &[syn::Item], ctx: &str, out: &mut Vec<Question>) {
    for it in items {
        match it {
            syn::Item::Macro(m) => {
                let head = m
                    .mac
                    .path
                    .segments
                    .last()
                    .map(|s| checker::names::plain(&s.ident))
                    .unwrap_or_default();
                if !REASON_MACROS.contains(&head.as_str()) {
                    continue;
                }
                if let (Some(item), said) = text_of(m.mac.tokens.clone()) {
                    if said.is_empty() {
                        continue;
                    }
                    out.push(Question {
                        ask: format!("Why is {} in {} the value it is?", item, ctx),
                        answer: said.join(" "),
                    });
                }
            }
            syn::Item::Enum(e) => {
                let names: Vec<String> = e
                    .variants
                    .iter()
                    .map(|v| checker::names::plain(&v.ident))
                    .collect();
                if names.len() < 2 {
                    continue;
                }
                out.push(Question {
                    ask: format!(
                        "Which variants does {} in {} have, and so which cases must every match cover?",
                        checker::names::plain(&e.ident),
                        ctx
                    ),
                    answer: names.join(", "),
                });
            }
            syn::Item::Const(c) => {
                out.push(Question {
                    ask: format!("Which context declares {}?", checker::names::plain(&c.ident)),
                    answer: ctx.to_string(),
                });
            }
            syn::Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    gather(inner, ctx, out);
                }
            }
            _ => {}
        }
    }
}

fn questions(root: &Path) -> Vec<Question> {
    let mut out = Vec::new();
    for f in checker::compiled_sources(root, checker::zone::Zone::Spec) {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(&f)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        if let Ok(parsed) = syn::parse_file(&text) {
            gather(&parsed.items, &context_of(&rel), &mut out);
        }
    }
    out
}

fn quiz(root: &Path, asked: &[Question]) {
    let mut taken: Vec<u128> = Vec::new();
    let stdin = std::io::stdin();
    println!("Answer each question by reading the tree, then press enter. Nothing you type is checked; the answer is shown so you can judge yourself.");
    for (at, q) in asked.iter().enumerate() {
        println!("\n{}. {}", at + 1, q.ask);
        let _ = std::io::stdout().flush();
        let started = std::time::Instant::now();
        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            break;
        }
        let ms = started.elapsed().as_millis();
        taken.push(ms);
        println!("   answer: {}\n   took {}.{:03} s", q.answer, ms / MILLIS, ms % MILLIS);
    }
    let total: u128 = taken.iter().sum();
    let line = format!(
        "{} questions, {} s total, per question {}\n",
        taken.len(),
        total / MILLIS,
        taken
            .iter()
            .map(|ms| (ms / MILLIS).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    let file = root.join(LOG);
    let written = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .and_then(|mut log| log.write_all(line.as_bytes()));
    if let Err(e) = written {
        eprintln!("{} could not be written: {}", file.display(), e);
        std::process::exit(ORIENT_BAD_ARGUMENT);
    }
    println!("\nrecorded in {}: {}", LOG, line.trim());
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut answers = false;
    let mut timed = false;
    let mut root: Option<PathBuf> = None;
    for a in &args {
        match a.as_str() {
            "--answers" => answers = true,
            "--quiz" => timed = true,
            "--help" | "-h" => {
                println!("{}", ORIENT_USAGE);
                return;
            }
            flag if flag.starts_with('-') => {
                eprintln!("unknown argument {}\n{}", flag, ORIENT_USAGE);
                std::process::exit(ORIENT_BAD_ARGUMENT);
            }
            path => {
                if root.is_some() {
                    eprintln!("one workspace root at most\n{}", ORIENT_USAGE);
                    std::process::exit(ORIENT_BAD_ARGUMENT);
                }
                root = Some(PathBuf::from(path));
            }
        }
    }
    let root = root.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
    let asked = questions(&root);
    if timed {
        quiz(&root, &asked);
        return;
    }
    for (at, q) in asked.iter().enumerate() {
        println!("{}. {}", at + 1, q.ask);
        if answers {
            println!("   {}", q.answer);
        }
    }
    println!(
        "{} questions, every one answerable from the spec alone; run with --quiz to time yourself",
        asked.len()
    );
}
