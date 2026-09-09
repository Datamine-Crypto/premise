use checker::tally;
use std::path::PathBuf;

const USAGE: &str = "usage: checker [<workspace root>]\n       checker --explain <E-CODE>\n       checker --help";
patterns_macros::because!(
    USAGE,
    "the whole command line on three lines, printed for --help and for any argument the checker does not know, so a mistyped flag is refused rather than run as a check of nothing"
);

const BAD_ARGUMENT: i32 = 2;
patterns_macros::because!(
    BAD_ARGUMENT,
    "the exit status for a command line the checker could not read, kept apart from the status a failed check exits with so a script can tell them apart"
);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{}", USAGE);
        return;
    }
    if let Some(code) = args.iter().position(|a| a == "--explain") {
        let wanted = args.get(code + 1).cloned().unwrap_or_default();
        println!("{}", checker::explain(&wanted));
        return;
    }
    if let Some(flag) = args.iter().find(|a| a.starts_with('-')) {
        eprintln!("unknown argument {}\n{}", flag, USAGE);
        std::process::exit(BAD_ARGUMENT);
    }
    if args.len() > 1 {
        eprintln!("one workspace root at most, got {}\n{}", args.len(), USAGE);
        std::process::exit(BAD_ARGUMENT);
    }
    let root: PathBuf = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("cwd"));
    if !root.is_dir() {
        eprintln!("{} is not a directory\n{}", root.display(), USAGE);
        std::process::exit(BAD_ARGUMENT);
    }

    let r = checker::report(&root);
    for d in &r.diags {
        println!("{}", d);
    }

    match checker::refresh_census(&root) {
        Ok(Some((was, now))) => println!("census moved: [{}] is now [{}]; {} is rewritten and the change is in your diff", was, now, checker::CENSUS_FILE),
        Ok(None) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    let where_ = format!("{}, {} in tools", tally(r.checked, "file", "files"), r.exempt);
    if r.diags.is_empty() {
        println!("{}, clean", where_);
        return;
    }
    println!("{}, {}", where_, tally(r.diags.len(), "diagnostic", "diagnostics"));
    let env = r
        .diags
        .iter()
        .filter(|d| checker::ENVIRONMENT.contains(&d.code))
        .count();
    if env > 0 {
        println!(
            "{} of those report a tool the checker could not reach, not a law the workspace broke",
            env
        );
    }
    println!("run: checker --explain <code> for what to do about one");
    std::process::exit(1);
}
