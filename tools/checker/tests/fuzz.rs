

const SEED: u64 = 0x5eed_1a17;
const RUNS: usize = 150;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[(self.next() % xs.len() as u64) as usize]
    }
}

const DIRS: &[&str] = &["spec", "app", "patterns", "tools/probe", "crates/spec", "a/b/c"];
const NAMES: &[&str] = &["spec", "app", "patterns", "probe", "r#spec", "SPEC", "spec-2", ""];
const ZONES: &[&str] = &["spec", "app", "patterns", "tools", "gen", "tests", "nonsense", ""];
const LEAVES: &[&str] = &["lib.rs", "mod.rs", "main.rs", "a.rs", "..", ".", "x/y.rs", ""];
const SEPS: &[&str] = &["/", "\\", "//", "/./", "/../"];

fn body(rng: &mut Rng) -> String {
    let shapes = [
        "pub const A: u32 = 1;",
        "pub enum E { X, Y }",
        "pub fn f() {}",
        "pub mod ghost;",
        "#[cfg(test)] pub mod t;",
        "pub use other::Thing;",
        "impl T for S { const M: u32 = 2; }",
        "pub static S: u32 = 3;",
        "",
        "this is not rust",
    ];
    rng.pick(&shapes).to_string()
}

struct Case {
    dir: String,
    name: String,
    zone: String,
    leaf: String,
    sep: String,
    lib: String,
    ghost: String,
}

#[test]
fn zone_and_target_resolution_never_panics_on_junk() {
    let mut rng = Rng(SEED);
    let base = std::env::temp_dir().join(format!("premise_fuzz_{}", std::process::id()));
    let cases: Vec<Case> = (0..RUNS)
        .map(|_| Case {
            dir: rng.pick(DIRS).to_string(),
            name: rng.pick(NAMES).to_string(),
            zone: rng.pick(ZONES).to_string(),
            leaf: rng.pick(LEAVES).to_string(),
            sep: rng.pick(SEPS).to_string(),
            lib: body(&mut rng),
            ghost: body(&mut rng),
        })
        .collect();
    let lanes = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(cases.len())
        .max(1);
    let each = cases.len().div_ceil(lanes);
    let mut panicked: Vec<String> = Vec::new();
    let ground = &base;
    std::thread::scope(|lane| {
        let mut running = Vec::new();
        for (at, part) in cases.chunks(each).enumerate() {
            running.push(lane.spawn(move || {
                let mut broke: Vec<String> = Vec::new();
                for (which, case) in part.iter().enumerate() {
                    let run = at * each + which;
                    let dir = ground.join(format!("r{}", run));
                    let _ = std::fs::remove_dir_all(&dir);
                    std::fs::create_dir_all(&dir).expect("a scratch directory can be made");
                    let root_ok = std::fs::write(
                        dir.join("Cargo.toml"),
                        format!("[workspace]\nmembers = [\"{}\"]\n", case.dir),
                    );
                    let zones_ok = std::fs::write(
                        dir.join("premise.zones"),
                        format!("{} = {}\n", case.zone, case.name),
                    );
                    root_ok.expect("the workspace manifest can be written");
                    zones_ok.expect("the zones file can be written");
                    let home = dir.join(&case.dir);
                    let src = home.join("src");
                    let _ = std::fs::create_dir_all(&src);
                    let _ = std::fs::write(
                        home.join("Cargo.toml"),
                        format!(
                            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\npath = \"src{}{}\"\n",
                            case.name, case.sep, case.leaf
                        ),
                    );
                    let _ = std::fs::write(src.join("lib.rs"), &case.lib);
                    let _ = std::fs::write(src.join("ghost.rs"), &case.ghost);
                    let found = std::panic::catch_unwind(|| checker::run(&dir));
                    if found.is_err() {
                        broke.push(format!(
                            "run {} panicked: dir {:?} name {:?} zone {:?} lib src{}{}",
                            run, case.dir, case.name, case.zone, case.sep, case.leaf
                        ));
                    }
                    let _ = std::fs::remove_dir_all(&dir);
                }
                broke
            }));
        }
        for one in running {
            panicked.extend(one.join().expect("a lane finishes"));
        }
    });
    panicked.sort();
    let _ = std::fs::remove_dir_all(&base);
    assert!(panicked.is_empty(), "{}", panicked.join("\n"));
}
