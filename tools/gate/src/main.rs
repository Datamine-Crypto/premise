use std::process::Command;

const SELF_EXCLUDED: &str = "premise_gate";
patterns_macros::because!(
    SELF_EXCLUDED,
    "the gate excludes itself from the compiler step because it is the running process and cargo cannot relink a binary it is executing; that it compiled is proven by it running"
);

const LAWS_STEP: &str = "four laws";
patterns_macros::because!(
    LAWS_STEP,
    "the step whose failure makes the rule tests fail on the same lines, since the smuggle suite refuses a tree that is not clean; naming it lets the gate skip the echo rather than report one defect twice"
);

const RULES_STEP: &str = "rules";
patterns_macros::because!(
    RULES_STEP,
    "the step the gate skips when the four laws failed, so one diagnostic reddens one step"
);

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Compiler,
    Laws,
    Library,
    Rules,
    Project,
}

const STEPS: &[(&str, Step)] = &[
    ("compiler", Step::Compiler),
    (LAWS_STEP, Step::Laws),
    ("library", Step::Library),
    (RULES_STEP, Step::Rules),
    ("project", Step::Project),
];

fn members_in(zones: &checker::config::Zones, wanted: &[checker::zone::Zone]) -> Vec<String> {
    zones
        .crates
        .iter()
        .filter(|c| zones.zone_named(&c.name).map(|z| wanted.contains(&z)).unwrap_or(false))
        .map(|c| c.name.clone())
        .collect()
}

fn is_member(zones: &checker::config::Zones, name: &str) -> bool {
    zones.crates.iter().any(|c| c.name == name)
}

fn packaged(head: &[&str], names: &[String]) -> Vec<String> {
    let mut out: Vec<String> = head.iter().map(|s| s.to_string()).collect();
    for name in names {
        out.push(String::from("-p"));
        out.push(name.clone());
    }
    out
}

fn args_of(step: Step, zones: &checker::config::Zones) -> Option<Vec<String>> {
    let test = ["test", "-q"];
    match step {
        Step::Compiler => {
            let mut args = packaged(&["test", "-q", "--workspace"], &[]);
            if is_member(zones, SELF_EXCLUDED) {
                args.push(String::from("--exclude"));
                args.push(String::from(SELF_EXCLUDED));
            }
            args.push(String::from("--no-run"));
            args.push(String::from("--all-targets"));
            Some(args)
        }
        Step::Laws => Some(Vec::new()),
        Step::Library => {
            let names = members_in(zones, &[checker::zone::Zone::Patterns]);
            match names.is_empty() {
                true => None,
                false => Some(packaged(&test, &names)),
            }
        }
        Step::Rules => match is_member(zones, RULES_PACKAGE) {
            true => Some(packaged(&test, &[String::from(RULES_PACKAGE)])),
            false => None,
        },
        Step::Project => {
            let names = members_in(zones, &[checker::zone::Zone::Spec, checker::zone::Zone::App]);
            match names.is_empty() {
                true => None,
                false => Some(packaged(&test, &names)),
            }
        }
    }
}

const USAGE: &str = "usage: gate [--scope <name>]...\n       a scope is a word that appears in the path of every file you own, usually a context name";
patterns_macros::because!(
    USAGE,
    "the whole command line on two lines, printed for any argument the gate does not understand, so a mistyped flag is refused rather than silently running unscoped"
);

const BAD_ARGUMENT: i32 = 2;
patterns_macros::because!(
    BAD_ARGUMENT,
    "the exit status for a command line the gate could not read or a cargo it could not run, kept apart from a failed step so a script can tell the two apart"
);

const UNVERIFIED: i32 = 3;
patterns_macros::because!(
    UNVERIFIED,
    "the exit status when steps failed and none of them named the scope: the tree is broken by somebody else, but a step that failed before reaching your files did not test them, so the run cannot be reported as a pass"
);

fn mine(text: &str, scopes: &[String]) -> Vec<String> {
    let wanted: Vec<String> = scopes
        .iter()
        .map(|s| s.replace(checker::config::BSLASH_ALT, "/"))
        .collect();
    text.lines()
        .filter(|l| {
            let line = l.replace(checker::config::BSLASH_ALT, "/");
            wanted.iter().any(|w| line.contains(w))
        })
        .map(|l| l.trim().to_string())
        .collect()
}

fn refuse(why: &str) -> ! {
    eprintln!("{}\n{}", why, USAGE);
    std::process::exit(BAD_ARGUMENT)
}

fn scopes_from(args: &[String]) -> Vec<String> {
    let mut scopes: Vec<String> = Vec::new();
    let mut at = 0usize;
    while at < args.len() {
        let a = &args[at];
        if a != "--scope" {
            refuse(&format!("unknown argument {}", a));
        }
        let value = match args.get(at + 1) {
            Some(v) if !v.is_empty() && !v.starts_with('-') => v.clone(),
            _ => refuse("--scope needs a name after it"),
        };
        scopes.push(value);
        at += 2;
    }
    scopes
}

const RULES_PACKAGE: &str = "premise_checker";
patterns_macros::because!(
    RULES_PACKAGE,
    "the crate whose test programs the gate runs itself; cargo builds a crate's programs and then runs them one after another, so the longest of them sets the step and every other core waits"
);

fn through_cargo(cargo: &str, args: &[String]) -> (bool, String) {
    let out = match Command::new(cargo).args(args).output() {
        Ok(o) => o,
        Err(e) => refuse(&format!("{} could not be run: {}", cargo, e)),
    };
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), text)
}

fn all_together(programs: &[std::path::PathBuf]) -> (bool, String) {
    let mut done: Vec<(bool, String)> = Vec::new();
    std::thread::scope(|lane| {
        let mut running = Vec::new();
        for one in programs {
            running.push(lane.spawn(move || match Command::new(one).output() {
                Ok(o) => (
                    o.status.success(),
                    format!(
                        "{}{}",
                        String::from_utf8_lossy(&o.stdout),
                        String::from_utf8_lossy(&o.stderr)
                    ),
                ),
                Err(e) => (false, format!("{} could not be run: {}", one.display(), e)),
            }));
        }
        for one in running {
            done.push(one.join().expect("a lane finishes"));
        }
    });
    let passed = done.iter().all(|(ok, _)| *ok);
    let text = done
        .iter()
        .filter(|(ok, _)| !ok)
        .map(|(_, said)| said.clone())
        .collect::<String>();
    (passed, text)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let scopes = scopes_from(&args);
    let scope = match scopes.is_empty() {
        true => None,
        false => Some(scopes.join(", ")),
    };
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
    let mut failed = 0usize;
    let mut ours = 0usize;
    let mut laws_failed = false;
    let zones = checker::config::Zones::load(&std::env::current_dir().expect("cwd"));
    for (name, step) in STEPS {
        if *name == RULES_STEP && laws_failed {
            println!("skip  {} (the four laws failed, and the rule tests refuse a tree that is not clean)", name);
            continue;
        }
        let args = match args_of(*step, &zones) {
            Some(a) => a,
            None => {
                println!("skip  {} (no crate of this workspace is in that step's zone)", name);
                continue;
            }
        };
        let (passed, said) = match *name == LAWS_STEP {
            true => {
                let root = std::env::current_dir().expect("cwd");
                let found = checker::report(&root);
                let lines: Vec<String> = found.diags.iter().map(|d| d.to_string()).collect();
                let mut lines: Vec<String> = lines;
                match checker::refresh_census(&root) {
                    Ok(Some((was, now))) => println!("census moved: [{}] is now [{}]; {} is rewritten and the change is in your diff", was, now, checker::CENSUS_FILE),
                    Ok(None) => {}
                    Err(e) => lines.push(e),
                }
                (lines.is_empty(), lines.iter().map(|l| format!("{}
", l)).collect::<String>())
            }
            false => match *name == RULES_STEP {
                true => {
                    let root = std::env::current_dir().expect("cwd");
                    let programs = checker::cargo::test_programs(&root, RULES_PACKAGE);
                    match programs.is_empty() {
                        true => through_cargo(&cargo, &args),
                        false => all_together(&programs),
                    }
                }
                false => through_cargo(&cargo, &args),
            },
        };
        if passed {
            println!("pass  {}", name);
            continue;
        }
        failed += 1;
        if *name == LAWS_STEP {
            laws_failed = said
                .lines()
                .filter_map(checker::diag::code_of)
                .any(|code| !checker::ENVIRONMENT.contains(&code));
        }
        let scoped = mine(&said, &scopes);
        match &scope {
            None => {
                println!("FAIL  {}", name);
                print!("{}", said);
            }
            Some(s) if scoped.is_empty() => {
                println!("FAIL  {} (no line names {}, so this is not yours)", name, s);
            }
            Some(s) => {
                ours += 1;
                println!("FAIL  {} ({} lines name {})", name, scoped.len(), s);
                for l in &scoped {
                    println!("      {}", l);
                }
            }
        }
    }
    if failed == 0 {
        println!("gate clean, {} steps", STEPS.len());
        return;
    }
    match &scope {
        Some(s) if ours == 0 => {
            println!(
                "{} of {} gate steps failed and none of them names {}; the tree is broken by somebody else, and a step that failed before reaching your files has not verified them",
                failed,
                STEPS.len(),
                s
            );
            std::process::exit(UNVERIFIED);
        }
        _ => {
            println!("{} of {} gate steps failed", failed, STEPS.len());
            std::process::exit(1);
        }
    }
}
