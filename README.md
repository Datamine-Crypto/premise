# Premise

[![gate](https://github.com/Datamine-Crypto/premise/actions/workflows/gate.yml/badge.svg)](https://github.com/Datamine-Crypto/premise/actions/workflows/gate.yml)
[![crates.io](https://img.shields.io/crates/v/premise.svg)](https://crates.io/crates/premise)
[![docs.rs](https://img.shields.io/docsrs/premise)](https://docs.rs/premise)
[![MSRV](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://blog.rust-lang.org/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A Rust discipline for codebases that AI agents write.** Documentation is abolished: every fact
lives in code exactly once, every value carries the reason somebody chose it, and a checker fails
the build the moment either one slips. The four worked contexts in this repository were each built
by an agent that had only this file.

## ❓ The problem

Documentation was designed for a reader who reads it once, remembers it, and cross checks it against
the code when something smells wrong. An agent does none of those three things. It starts every
session cold, it takes the prose as ground truth, and it acts on it in seconds.

That turns three slow problems into fast ones.

**Stale prose becomes a wrong implementation.** A comment that was true two years ago used to cost a
human ten minutes of confusion. It now costs a feature built on a fact that stopped being true, and
it costs it before anybody reads the diff.

**Every session leaves more prose behind to go stale.** Ask for a feature and you get the feature, a
README section, doc comments and a design note. Each one is a fresh copy of something that already
lives in code, and nothing compares a copy to its original. The rate of drift is now a function of
how fast you ship.

**Nothing records why a value is what it is.** An agent writes `const RETRY_LIMIT: u32 = 3;` because
three is a reasonable number, and on the page that is indistinguishable from a three somebody
measured. Six months later nobody can tell the two apart, and the value becomes a number no one
dares to change.

The third is the worst of them, because a plausible invented reason is worse than no reason at all.

## 💡 The solution

Prose carries two things with opposite properties. Split them, and handle each the way its nature
allows.

**A fact has an identity a compiler can test.** So a fact lives in code and nowhere else, stated
once. `spec/` states them, `app/` names them, `patterns/` holds logic that names none of them.
Stating one twice fails the build, and so does writing a bare `3` outside the spec. There is no
second copy for an agent to read stale, because there is no second copy.

**A reason has no identity function.** Two sentences meaning the same thing are different strings,
nothing can detect the duplicate, and no tool can tell you a reason is true. So a reason stays
English, and it is attached to its item as code:

```rust
pub struct WearTest;
source!(WearTest, "the hinge fatigue test run on the entry door before fit-out");

pub const OPEN_LIMIT: u32 = 3;
because!(OPEN_LIMIT, WearTest, "three openings is where the test showed hinge fatigue");
```

Delete that `because!` and the build fails. Write a second one for the same item and the build
fails. Write `3` anywhere outside the spec and the build fails. What no check does is read the
sentence, and that is deliberate: reading it is the one job left that cannot be automated, so the
reasons stay short, sit on the value they explain, and are the only English in the tree.

When an agent has nobody to ask, it writes `provisional!` rather than inventing a decision nobody
made. The value still compiles. It is now marked as a guess, counted in a tracked census, and
visible in git history as a guess instead of dressed up as a measurement.

One limit, stated up front. Nobody has yet shown that a reader orients faster in a Premise codebase
than in a documented one. That is the thesis, it needs a real team on a real system over months, and
until somebody runs it the claim stays a claim.

**This file is the whole of the prose.** There is no second document, no wiki and no doc comment,
because a second copy is the thing this project exists to remove. Read it in order the first time.
Everything after "Run the gate" is reference you will come back to.

## 📦 The crates

| crate | library name | what it holds |
|---|---|---|
| [`premise`](https://crates.io/crates/premise) | `patterns` | the core: sequences, tables, records, money, days, text |
| [`premise_macros`](https://crates.io/crates/premise_macros) | `patterns_macros` | `because!`, `source!`, `provisional!`, `#[derive(Record)]` |
| [`premise_web3`](https://crates.io/crates/premise_web3) | `premise_web3` | addresses with their checksum, keccak, ABI decoding of event logs, pool arithmetic |
| [`premise_cloudflare`](https://crates.io/crates/premise_cloudflare) | `premise_cloudflare` | records in D1, gzip objects in R2, the HTTP edge helpers a Worker binds its policy to |
| `premise_checker` | `checker` | the four laws, as a library and a command |
| `premise_gate` | | the gate, plus the `catalog` and `orient` commands |

You depend on one:

```toml
[dependencies]
premise = "0.1"
```

`premise_macros` arrives transitively. Add `premise_web3` or `premise_cloudflare` only if you want a
chain or a Worker. `spec/` and `app/` are worked examples in this repository and are not published.

The two tools are not on the registry at all. They embed this file with `include_str!`, which reads
from above the crate directory, so a packaged tarball would ship without the manual the checker
explains itself from. Take them from the repository instead:

```sh
cargo install --git https://github.com/Datamine-Crypto/premise premise_gate
```

Publish the four libraries in dependency order: `premise_macros`, then `premise`, then the other
two. `cargo package` refuses a dependent until its dependency is on the registry, so the order is
not optional.

## ⚖️ The four laws

1. **All logic lives in a pattern.** `patterns/` holds generic parameterized logic that
   names no project noun and carries no inline literal, only named constants. `app/` calls
   patterns and nothing else. `spec/` states facts and holds no free functions.
2. **Every value comes from the spec.** `app/` names a spec constant, never `100`. The only
   literals allowed outside `spec/` are `0`, `1` and booleans. A named `const` in `app/`
   is the same violation as an inline one: the value still lives outside the spec. `patterns/`
   is the exception, because it may not depend on `spec/`, so an algorithm constant such as a
   PRNG multiplier is named there and nowhere else.
3. **Zero comments.** No `//`, `/* */`, `///` or `#[doc]`, anywhere. Explanation is a
   `because!` call, which is code.
4. **One program.** Every tool sees the same files. The set rustc compiles equals the set on
   disk, unconditionally and on every machine, because the checker reads the
   disk while only rustc reads the program. When they disagree, every law above is void rather
   than weakened: the facts are stated twice, once to rustc and once to everyone else, and the
   copies drift. The checker asks cargo which files those are, so it never guesses.

## 🗂️ Zones

| zone | holds | may contain |
|---|---|---|
| `patterns/` | generic logic, portable to any project | all control flow, no literals, no project nouns |
| `spec/` | facts: consts, types, enums, trait impls, reducers | no free functions, no logic outside const derivation and total matches inside trait impls, no literal in a function body other than `0`, `1`, booleans and the strings a `Named` returns; a value is a const with a reason and the body names the const |
| `app/` | bindings that wire patterns to spec values | calls to patterns only, no literals, no control flow, no macro but `vec!` and the reason macros, and the arguments of a `vec!` are checked like any other expression. The duplication rules do NOT run here, and that is deliberate: see below |
| `tools/` | the toolchain | only logic placement is waived, since a parser cannot be bindings, along with lexical literals (characters, 0, 1, 2), reasons on character constants, and `include_str!`, which is how the checker carries this manual. Everything else applies: a tuned number must be a named constant carrying a `because!`, comments are banned, duplicate logic and duplicate names are detected |
| `tests/` | a crate's own test target, in any zone | everything: literals, loops, closures. Every law is off but the comment ban, because a test states the expected value and stating it is the whole point |

`premise.zones` maps a zone to a CRATE NAME, not a directory. The checker reads the workspace
members and each member manifest, so a zone is an identity rather than a path prefix. A directory
cannot be renamed out of its laws, `tests/` means a crate's own test target and not any directory
called tests, and `E-ZONE-DEPENDS` reads resolved dependency names, so a `package = ` rename does
not hide a dependency. A source file under no member is `E-ZONE-UNKNOWN`. With no workspace
manifest present, the checker falls back to directory prefixes so a bare folder can still be
checked; the rule that a crate root forbids `unknown_lints` reads the crate's manifest to find
that root, so it is silent there.

A project that takes the library from outside its workspace adds one line per library crate,
`library = <path to the crate directory>`, such as `library = ../premise/patterns`. The checker
reads that crate's manifest for its crate name, zones the package as `patterns`, learns the names
it exports so `app/` may call them and `spec/` may name them, and lets a path dependency resolve
into it without `E-OUTSIDE-WORKSPACE`. It does not scan the library's files as the project's own,
and the gate does not run its tests: the library answers to its own gate in its own workspace. A
path in the code may then start from the library's crate name or from the crate name of any
member the `patterns` zone holds, and a project's own pattern crate carries a name of its own,
since the library already took `patterns`.

## 📚 Before writing any logic, read the library

```
cargo run -q --bin catalog
```

Prints every pattern and trait with its signature, and the name of every macro `patterns/` exports,
read from `patterns/` itself so it can never be out of date. A trait's marker parameter is printed
with it, so `Facet<F>` reads as what it is. **Search it before writing a new pattern.** A new
pattern is justified only when nothing there fits. This is the entire point of the folder. The
reason macros are listed by name only; this manual is their reference.

Anything that touches the world outside the program lives here too, which is why the library owns a
writer of image files rather than the binding that wanted one. A binding may call `patterns/` and
nothing else, so a file, a clock or a device that a binding reaches for has nowhere else to be, and
putting it here means the decision is made once, with a reason, and every project that needs it
finds it in the catalog instead of writing its own. `capture` is the worked example: it turns a
frame of colours into the bytes of a file any desktop opens, and the padding, the row order and the
byte order the format wants each carry the reason they are what they are.

The toolchain does not use the library, and that is deliberate rather than an oversight. `tools/`
depends on the reason macros and nothing else, so a library that does not compile still leaves a
gate that runs and a checker that says why. A toolchain built on the thing it governs would answer
a broken library with a build error instead of a diagnostic, which is the one moment the tools are
for. The cost is that a lane split or a distance is written twice, once here and once there.

## ✅ Run the gate

```
cargo run -q -p premise_gate
```

On a tree with more than one agent in it, add `--scope` and a word that appears in the path of every
file you own. A feature spans `spec/src/contexts/<name>/`, `app/src/<name>.rs` and
`app/tests/<name>.rs`, so the word is the context name, not a directory:

```
cargo run -q -p premise_gate -- --scope lockers
```

Scope is a substring match over each diagnostic line, so a directory path would keep the spec hits
and drop your own bindings and tests. Repeat `--scope` to own more than one name.

Check the commit, not the working tree. `git clone` your repo into an empty directory, check out the
commit you are about to ship, and run the gate there. That is the only test that proves the artifact
is deployable rather than proving your own copy is, and the two came apart twice here: once because
three contexts were untracked while the tree that compiled them was not the tree in the commit, and
once because git rewrote line endings under a checkout, so a file differed from itself on a tree
nobody had touched. Neither was visible from inside the tree. `.gitattributes` pins the line
endings; nothing but a clone catches the first.

Every failing step is then reported with the lines that name your scope, or told you it names none
and the breakage is somebody else's. The exit status says which: `1` when a failure names your
scope, `3` when failures exist and none does. The second is not a pass: a compiler step that failed
on somebody else's file never reached yours, so your work is unverified until the tree is green.
Only a gate with every step passing exits `0`. Without `--scope` the gate prints everything and
exits `1` on any failure. An argument the gate does not know, or a `--scope` with no name after it,
is refused with status `2` rather than run unscoped.

The compiler step excludes the gate binary itself, since it is the process doing the excluding and
cargo cannot relink a binary it is executing. It compiles every target of every other crate, benches
and examples included, so a file cargo would otherwise never read cannot hold a line the law zone
of its crate lets through.

Five steps: the compiler, the four laws, the standard library tests, the rule tests, and the
project's own tests. All five must pass. A failing step is fixed, never weakened. When the four laws
fail on a real law, the rule tests are skipped rather than run, because they refuse a tree that is
not clean and would only echo the same lines.

## 🐌 When the gate gets slow

A project built with Premise reported its gate at over five minutes and found almost none of the
difference was its own code. Every cost was in the toolchain or in how a test asked cargo to do
something. What that project measured is written below, because the same costs appear in any tree
that grows, and because the cheapest of them are the ones nobody thinks to look for.

**Measure first, and measure three times.** Every wrong turn in that report came from acting on a
guess or on one reading. Compiling the workspace was never the problem there and is not the problem
here: it was 2.3 seconds at the start and 2.3 seconds at the end. The spread between two runs of the
same command was larger than several of the differences chased.

```
cargo run -q -p premise_gate
cargo test -q -p premise
cargo test -q -p spec -p app
cargo test -q -p premise_checker
cargo test -q -p premise_checker --test rules
```

Narrow inside a program with `--test <name> -- --list`, then run one test by its whole name with
`--test <name> <whole name> -- --exact`. A filter that matches nothing prints a pass in about a
tenth of a second, so read the test count, never the timer alone.

**A test that shells out to cargo must be told where to build.** The default is a fresh directory,
which is a cold build of every dependency, thrown away. Set the cargo target directory environment
variable to a path that is kept between runs. In that project seven fixtures each held their own
build of the library, at about 280 MB apiece, to find out whether a macro refuses to be written
twice.

**A nested build must not share a directory with the tree it is testing** when it generates crates
whose names collide with real ones. Two generated crates named `spec` in one directory link the
wrong one, and the failure appears only when two programs run together. One directory per test
program.

**Before a test copies something real, ask what it reads from it.** The rules about reaching into
the library work on the crate root of a path, not on what the crate holds, so a test about import
forms needs the library's name and not its body. Stand a small thing in and let the assertion catch
you if you were wrong: a rule that ever needs the real thing stops firing and says so.

**A sweep that scans a whole tree per case gets slower as its users' projects grow.** Give it lanes,
one tree per lane, and gather the failures rather than stopping at the first, so the message reads
the same twice.

**Count the processes you start.** They do not parallelise away and they do not appear in a profiler
that watches only your own code. The checker asks cargo which files are the program, and that
question costs about 30 ms every time it is asked.

What the toolchain does about all this, so a project need not: the checker parses each file once per
report rather than once per pass, skips the question to cargo when the root has no manifest, and the
gate runs a crate's test programs itself, all at once, rather than letting cargo run them one after
another. That last one is guarded, because a step that runs a list can silently run a short list: a
test asserts every test file of the crate appears in the listing the gate uses, and a crate whose
programs cannot be listed falls back to letting cargo run them.

What is left, and what it would take: the checker still asks cargo about the workspace on every run,
which dominates a suite that checks many small trees. Reading the manifest directly would answer
faster and is what the fallback already does, but the fallback is a second opinion about which files
are the program, and law four says the answer must be the compiler's. The scan is single threaded
because a parsed file is neither `Send` nor `Sync`, so lanes cannot share one, and the accumulators
would have to stop holding parsed items before that could change. Shape comparison grows with the
number of patterns, so it is the phase most likely to bite a project several times this size.

**Look for the same work being done twice before reaching for structure.** Every win in that report
was a repeat removed, or work that need not have been done at all. Not one of them needed a new
crate, a profile setting or a cargo config file, and that project ended with none of the three. A
tree with no build configuration is not a tree that has not been tuned yet. It is the normal case,
and reaching for configuration first is how a day goes missing.

### Would more crates help?

Asked directly by that project, and answered with measurements rather than intuition. Its numbers
are worth keeping, because the intuition is strong and wrong in two of the three cases.

**One crate per pattern module: no.** About 25 crates for 9,430 lines. Cargo already builds the
graph in parallel, and this graph is mostly linear, since nearly everything depends on the same few
modules. Every crate boundary costs a fixed amount in metadata and linking, and 25 boundaries would
very likely cost more than they save when compiling everything is already a couple of seconds.

**Splitting the library from a project's own patterns for speed: no.** The hope is that a crate
which never changes need not be rebuilt or rescanned. Cargo runs a crate's test suite again whether
or not the crate changed, so that hope does not pay. What actually cost that project seven seconds
was one test copying the library and scanning it forty times, and the fix was to stand a two line
crate in. The library's size was the problem. Which crate it lived in was not.

**Splitting the library from a project's own patterns so several projects share one version: yes.**
That is what the `library =` line is for. The project keeps its own `patterns` zone crate for the
logic only it needs, and the shared library is a dependency with a version. Nothing about speed
changes; what changes is that a fix to a shared pattern lands in every project by a version bump
rather than by a copy.

**Splitting pure logic from a shell that owns a window or a device: yes, but not for speed.** When
`patterns/` holds the arithmetic and the audio device together, a fixture that wants one macro pulls
in the whole media stack, and an edit to a signal function relinks it. That is worth separating for
the shape of the thing, and it cuts what the nested builds cost cold. It is not where the time is.

### The wrong turns that project took

Each cost a measurement to find and would cost another to find again.

- Assumed compilation was slow. It was 2.3 seconds. An hour went into linkers and debug info before
  the first measurement killed the idea.
- Assumed one shared build directory would fix two tests that built into temporary directories. It
  moved a five minute suite by nineteen seconds.
- Assumed the main build directory was safe to share. It is not, once a generated crate has the same
  name as a real one, and the failure appears only when two programs run together.
- Assumed lane counts needed tuning once the sweeps ran in parallel. Tested at a quarter and a sixth
  of the cores: no difference beyond the noise, because the contention was not in the processor.
- Read one gate timing as a regression. It was an outlier.

## 🧮 Derive, do not restate

A value computed from other values is written as a derivation, never typed out again.

```rust
pub const MAX_HP: u32 = 100;
because!(MAX_HP, "one session of the playtested length at the measured attrition rate");

pub const HP_PER_WAVE: u32 = 5;
because!(HP_PER_WAVE, "the attrition per wave the playtest runs measured");

pub const WAVE_CAP: u32 = MAX_HP / HP_PER_WAVE;
```

`WAVE_CAP` needs no `because!`. The expression is its own reason, rustc computes it, and it cannot
disagree with its inputs. A derivation is a fact about a relationship, and the relationship is what
gets stated, so the answer is never written down anywhere.

A const initialised by a literal is a **choice** and needs a reason. A const initialised by an
expression is a **consequence** and does not. The same test applies to a trait impl: an impl whose
associated consts are literals is a choice, so `decided!(Plan, Slots, "...")` is required, one per
impl. `because!` attaches to any item, a const, struct, enum, trait or impl target, so the
relational facts the manual tells you to prefer can carry their reason.

## 🔌 Getting a value into a pattern

Patterns take `fn` pointers, never closures, and a `fn` pointer cannot capture. So this is a dead
end, and no catalog entry can rescue it:

```rust
any(&state.loans, |loan| loan.title == *title)   // E-LOGIC-OUTSIDE-PATTERN, closure
```

The captured value has to arrive some other way, and the way is a trait on the element type. Put the
varying part behind a trait, implement it in `spec/`, and the pattern stays generic while the value
travels in the argument:

```rust
// patterns/
pub trait Keyed { type Key: PartialEq; fn key(&self) -> Self::Key; }
pub fn holds<T: Keyed>(items: &[T], key: T::Key) -> bool

// spec/
impl Keyed for Loan { type Key = u32; fn key(&self) -> u32 { self.title } }

// app/
holds(&state.loans, *title)
```

An empty collection is written `vec![]`, not `Vec::new()`, because a constructor call is not a
pattern call and would report `a call to a non-pattern`. `is_empty` is the predicate that asks
whether one is empty, not the constructor that makes one.

This is the general technique, and `Named` is its degenerate case: nothing varies at runtime, so the
trait carries no argument, and the text is still reached through a trait rather than a closure.
`Keyed`, `Advance` and `Ranked` in `patterns::seq` are the full form: `Keyed` carries the value it
searches for as `key: T::Key`, `Advance` carries the amount it advances by as `by: T::By`, and
`Ranked` carries the bound it compares against as `bound: T::Rank`. A trait in this library that has
nowhere to put the varying value is a bug in the trait, not a limit of the technique. That is a rule
for adding traits, not a claim about the ones here: check it when you write the next one, because it
is the kind of property that quietly stops being true the moment someone adds a trait without asking
the question.

### The recipe

When a closure is rejected, the varying value has to travel as an argument, and the element has to
expose whatever the pattern needs to compare or transform. Three steps, every time:

1. Name what VARIES at runtime. That becomes a parameter on the pattern.
2. Name what the pattern needs FROM the element. That becomes an associated type on a trait,
   and if the element must RECEIVE the varying value rather than just expose something to
   compare against, a method that takes it. Bound the associated type with whatever the
   pattern needs to do with it: `PartialEq` to match, `PartialOrd` to order, `Add` to total.
3. Write the pattern generic over the trait, taking the varying value as an argument.

Worked on two of them. `holds(items, key)`: the key varies, the element must expose a comparable
key, so `Keyed { type Key; fn key(&self) }` and `holds<T: Keyed>(&[T], T::Key)`. `count_above(items,
bound)`: the bound varies, the element must expose an ordered rank, so `Ranked { type Rank; fn
rank(&self) }`.

Apply it to a shape with no route yet and you get the trait without asking anyone. A predicate over
two fields: nothing varies except the pair you compare against, the element must expose both, so the
associated type is a tuple. A `map` whose output depends on a runtime input: the input varies and
the element must accept it, so the trait carries a method taking the input, which is exactly
`Advance` with a different name. An element compared against another element: nothing varies at
runtime, so no trait is needed and the pattern takes two slices.

A fourth shape, derived from these three steps by someone who had not read how `Keyed` was built:
totalling a charge at a rate that arrives at runtime, seeded with a carried balance. Step one gives
two varying values, the rate and the start. Step two needs the element to RECEIVE the rate rather
than expose something, so it is a method, and the amount must add, so it is bound by `Add`.

```rust
pub trait Charged {
    type Rate: Copy;
    type Amount: core::ops::Add<Output = Self::Amount> + Copy;
    fn charge(&self, rate: Self::Rate) -> Self::Amount;
}
pub fn total_charge<T: Charged>(items: &[T], rate: T::Rate, start: T::Amount) -> T::Amount
```

That is not in `patterns::seq`, deliberately. It is what the recipe produces when you follow it, and
the point of writing it here is that you can produce it too rather than wait for it to be shipped.
Expect to use it early: every projection trait in the library relates one projection to at most one
argument, so the first domain rule with two conditions, an equality on one field and an ordering on
another, has no combinator and cannot have one, because combining two predicates needs a closure.
The recipe is not only the escape hatch for closures; it is the ordinary way a second condition gets
expressed, and for most domains that arrives by the second or third feature.

The library already holds traits of this shape, `Keyed`, `Advance`, `Ranked`, `Facet`, `Shift` and
`Valued`, and when you add another, look at them together. `Keyed` and `Ranked` are both "project
the element onto something comparable, then relate it to the argument". At that many instances the
projection is worth naming once instead of per relation, and pinning one associated type per trait
starts to bite: a value that needs two different keys cannot have them.

That bite has a way out that `patterns::seq` already takes, and you will reach it sooner than you
expect. Coherence allows one `Keyed` impl per type, so the moment one value needs two projections
you are stuck. Parameterise the trait by a marker type, which `patterns::seq` already does:

```rust
pub trait Facet<F> {
    type Value: PartialEq;
    fn facet(&self) -> Self::Value;
}
```

Then a `Hold` can be projected by member, by title and readiness, and by all three, because
`Facet<ByMember>`, `Facet<ByQueue>` and `Facet<ByClaim>` are different impls. The markers are empty
structs in `spec/`, and they are vocabulary: `ByMember` names a way of looking at a hold, so it
belongs beside the hold. `count_facet`, `any_facet` and `first_facet` take the projection the same
way `count_above` takes a rank.

The library also holds the traits a domain's values implement so a pattern reads them without naming
the domain: `Kinded` and `Shape` for an event's word kinds and layout, `Built` for a decoded event
from its words, `Fresh` for a working set that reports what it touched, `Earner` for a row a dollar
walk prices, `Charted` for a chart a table of readings draws, `Worded` for a run of a title with its
ink, `Texted` and `Shown` for a value rendered as text around its number or shown under a marker,
and `Fielded` for a value moving to a store or a wire as fields, which `Record` derives from the
fields a struct already names. Each is a reading of a spec type, so the spec implements it and the
pattern never imports the type.

The same bite reaches `Advance`, which mutates a value by a step, and `Shift<M>` is the marker
parameterised form of it. Which one you want is a modelling question and both are in this tree.
`Valued<M>` is the third marker trait, for a lookup rather than a projection or a mutation.

If the ways a value moves are a closed vocabulary the domain already names, one `Advance` with a
step enum is right, and `seasonticket` does that: `Step { Hold, Release, Move }` is what can happen
to a ticket, and the enum is the vocabulary rather than a bag. If they are unrelated ways to change
the same value, `Shift<M>` keeps them apart, and `fines` does that: a member is charged by
`Shift<ByCharge>` and settled by `Shift<BySettle>`, which have nothing to do with each other beyond
sharing a subject. The failure to avoid is merging unrelated mutations into one enum because
coherence forced you to, which is what happens when `Shift` is not in reach.

`Ranked` is an ordering and nothing else. `fines` spends `Loan`'s on the due day, which is what lets
`overdue` and `current_loans` partition the shelf between them. A table that maps a vocabulary to a
value, a grade to its fine cap, a tier to its stand-down, a size to its rate, is not an ordering,
and it is `Valued<M>`: `impl Valued<ByCap> for Grade`, read with `value_of::<ByCap, Grade>(&grade)`.
The marker says which table, so one enum may carry several. The moment a value needs a second
ordering, write `Ordered<M>` in the same mould as `Facet<F>` and give it a reason saying why both
are there.

The traits above are the routes that exist today, and that list is OPEN, not complete. A predicate
over two fields and a predicate comparing an element to another element have the same shape and
neither has been built yet. If you hit one, the fix is a new projection trait in the same mould.
When you hit a closure, ask what varies, and give that a trait.

## 🧱 Contexts

A context is a bounded area with a closed vocabulary. Nest them under `contexts/`.

```
spec/src/contexts/<name>/vocabulary.rs    the closed vocabulary, enums only
spec/src/contexts/<name>/state.rs         the state shape
spec/src/contexts/<name>/reducer.rs       impl Context, the transition table
```

The vocabulary is enums, never strings. A duplicate variant is a compile error and a match over it
is checked exhaustive, which a string map never is.

```rust
pub enum Command { Start, Advance, Clear }
pub enum Event { Started, Advanced, Cleared }
pub enum Fault { AlreadyRunning, NotRunning }
```

State is derived from events and never stored twice.

```rust
impl Context for Wave {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State { ... }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command { ... }
    }

    fn apply(state: State, event: &Event) -> State {
        match event { ... }
    }
}
```

A context has two tables. `decide` turns a command into an event or a fault and is where rules are
enforced. `apply` turns an event into new state and never fails, because an event is something that
already happened. Drive them with `patterns::handle` and `patterns::handle_all`, which stops at the
first fault and returns it with the index of the command that raised it, because a half-applied
batch is a state no replay can reach and a caller can only retry when it knows where the batch
broke; rebuild state with `patterns::replay`, or catch a snapshot up with `patterns::replay_from`.

A vocabulary carries its own text by implementing `patterns::Named`, which is a total match over its
own variants and needs no exemption. `Display` does not fit, because writing to a formatter is a
method call. There is no query type. Reading state is a field access, and any computation over it is
a pattern call from `app/`, so a query needs no new concept.

`match` in `spec/` is allowed inside a TRAIT IMPL, when the scrutinee is a place expression and
every arm is a variant pattern with no wildcard arm and no guard. A `..` inside a struct variant
pattern is fine; it is the arm `_ =>` that is refused. That is a transition table rather than a
branch, and rustc proves it total. The rule is about the SHAPE of the match, not the name of the
trait, which is why `Named` works as well as `Context`. Free functions and inherent impls are barred
from `spec/` by `E-SPEC-FN`, so a match has nowhere else to live. Replay with
`patterns::replay::<Wave>(&events)`.

Import a context by its vocabulary module rather than by each name, which keeps two contexts in one
file readable:

```rust
use spec::contexts::seats::vocabulary as seats;
use spec::contexts::invoicing::vocabulary as invoicing;
```

One context reaches another only through its `vocabulary` module. Touching another context's `state`
or `reducer` is `E-CONTEXT-LEAK`. The rule covers the whole of `spec/`, so a bridge module outside
`contexts/` cannot launder a reach, and relative `super::` paths are resolved before the check.

A fact two contexts share goes in `contexts/shared/`, which every context may reach. Without it the
isolation rule would force the same enum to be declared once per context, which is the duplication
this system exists to prevent, reintroduced by the mechanism meant to prevent it.

## 🚪 One context, end to end

Everything above in one piece. A door that can be locked, with a counter, in eight files. Four carry
the content and four carry the wiring, and the wiring is not filler: a file no `mod` declares is
`E-ORPHAN-FILE`, because rustc never compiles it while the checker reads it and reports on it.

```rust
// spec/src/lib.rs
pub mod contexts;
```

```rust
// spec/src/contexts.rs
pub mod door;
```

```rust
// spec/src/contexts/door/mod.rs
pub mod reducer;
pub mod state;
pub mod vocabulary;
```

```rust
// spec/src/contexts/door/vocabulary.rs
use patterns::{because, source};

pub struct WearTest;
source!(WearTest, "the hinge fatigue test run on the entry door before fit-out");

pub struct Door;
because!(Door, WearTest, "the entry door, the only fixture the test covered");

pub enum Command { Open, Lock }
pub enum Event { Opened, Locked }
#[derive(Clone)]
pub enum Fault { Worn, AlreadyLocked }

pub const OPEN_LIMIT: u32 = 3;
because!(OPEN_LIMIT, WearTest, "three openings is where the test showed hinge fatigue");
```

```rust
// spec/src/contexts/door/state.rs
pub struct State {
    pub opened: u32,
    pub locked: bool,
}
```

```rust
// spec/src/contexts/door/reducer.rs
use crate::contexts::door::state::State;
use crate::contexts::door::vocabulary::{Command, Door, Event, Fault, OPEN_LIMIT};
use patterns::{either, is_at_least, raise_by, Context};

impl Context for Door {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State { opened: 0, locked: false }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Open => either(
                is_at_least(state.opened, OPEN_LIMIT),
                Err(Fault::Worn),
                Ok(Event::Opened),
            ),
            Command::Lock => either(state.locked, Err(Fault::AlreadyLocked), Ok(Event::Locked)),
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Opened => State { opened: raise_by(state.opened, 1), locked: state.locked },
            Event::Locked => State { opened: state.opened, locked: true },
        }
    }
}
```

```rust
// app/src/lib.rs
pub mod door;
```

```rust
// app/src/door.rs
use patterns::{handle, is_at_least, replay};
use spec::contexts::door::state::State;
use spec::contexts::door::vocabulary::{Command, Door, Event, Fault, OPEN_LIMIT};

pub fn current(events: &[Event]) -> State {
    replay::<Door>(events)
}

pub fn step(state: State, command: &Command) -> Result<State, Fault> {
    handle::<Door>(state, command)
}

pub fn worn(state: &State) -> bool {
    is_at_least(state.opened, OPEN_LIMIT)
}
```

Those eight files are the complete Rust and nothing else. A workspace needs manifests, an
`premise.zones` and a first gate run as well; `README.md` owns that, and you should read it first if
this is a new workspace.

Every line of that passes the gate, and rustc compiles it. The fixture
`tools/checker/tests/fixtures/door` is these eight files, the rule tests assert the checker is
silent on it, and a test in `tools/checker/tests/manual.rs` asserts the blocks above equal the
fixture byte for byte and that the fixture builds. Earlier drafts did not pass. One used `match
state.locked { true =>, false => }`, the bool-match trap listed under "Edges worth knowing before
you hit them", and left `Door` without a reason. Another showed only the four content files, which
the checker of the day called clean while rustc compiled none of them, because no `mod` declared
them. That draft is why `E-ORPHAN-FILE` exists.

Five things to notice, because each is a law doing its job:

`OPEN_LIMIT` lives in `spec/` and carries a reason, because it is a choice. `decide` reads it, so
the limit is a rule the context enforces and not a number a caller may ignore, and `app/` names it
without ever writing `3`. Both arms of `decide` can fault, so a command is a request and never an
instruction. The reducer's two matches are exhaustive over closed enums, which is why they are
permitted in `spec/` at all. And `app/` contains no `if`, no operator and no literal: every line is
a call to a pattern, fed from the spec.

## ➕ Adding a context to a workspace that already has one

The worked example builds the first context. The second is different, and this is what you need that
the first one did not teach.

**Read the neighbours before you name anything.** `E-DUP-VOCABULARY` compares your enums against
every other context's, and two two-variant enums sharing one variant name is a hit. `catalog` will
not tell you this; the other `vocabulary.rs` files will.

**A plain spec constant is compared across the whole spec, not within your context.** Two contexts
may each own a `Plan` or a `Status`, because a type is scoped by its context. Two contexts may not
both declare `TERM_DAYS`, whatever its visibility, and may not both declare a `source!` named
`TariffReview`: one review is one item, and if they really are two reviews, name them apart. Prefix
a constant with your context's name when the word is generic: `LOCKER_TERM_DAYS`, not `TERM_DAYS`.

**Reasons are scoped the same way as the thing they explain.** Two contexts may each give their own
`Status` its own `because!`, which is the point: a shared vocabulary that both contexts genuinely
need belongs in `contexts/shared/`, and a same-named enum that means two different things belongs to
each of them separately, with a reason each saying why they are not the same thing.

**Your bindings will look like the neighbour's and that is fine.** `current`, `step` and `steps`
wrap `replay`, `handle` and `handle_all` for your context type, and every context has them. The
duplication rules do not run in `app/` precisely so that this is allowed.

**Facts you share go in `contexts/shared/vocabulary.rs`, and nothing else does.** State and reducers
belong to a named context. A derivation may reach across, so with the first two constants in the
shared vocabulary and the third in a context of its own, this is lawful and normal:

```rust
pub const POST_DAYS: u32 = 2;
because!(POST_DAYS, "the days the carrier takes to deliver a parcel across the county");

pub const GRACE_DAYS: u32 = 3;
because!(GRACE_DAYS, "the slack the desk allows a member after delivery before a return is late");

pub const RETURN_WINDOW_DAYS: u32 = POST_DAYS + GRACE_DAYS;
```

## 📖 When the door is too small

The door above teaches the shape. It does not teach the idiom, because it has one collection of
nothing and one field of state, and a real context has neither. Two cold readers each said the same
thing: they read whole neighbouring contexts to learn how the pieces actually go together, and that
was the largest single cost of their first feature.

So read them. Four contexts ship in `spec/src/contexts/`, every one of them built by an agent who
had only this manual, and all four are verified by the gate on every run, which means they cannot
drift from what the tools accept the way a longer example in this file would.

| context | read it for |
|---|---|
| `holdqueue/` | a queue with order that matters, three `Facet` projections on one type, and `Advance` moving a hold between waiting and notified |
| `lockers/` | the widest state: five projections, two collections in one `State`, a salvage log written when a rental is reclaimed, and the `Rent` arm as a lazy guard table through a `Guards` trait in `state.rs` |
| `seasonticket/` | `Advance` with a step enum, `Step { Hold, Release, Move }`, when the ways a value moves are a closed vocabulary the domain names |
| `fines/` | a per-unit rate that changed on a date: `supersedes!` with the era chosen in `decide`, a cap read as `Valued<ByCap>` and a waiver as a cap of zero, a `decide` arm written as a `refuse_when` guard table, and `Shift<ByCharge>` beside `Shift<BySettle>`, when two mutations share a subject and nothing else |

Read the one closest to your problem before you design. The bindings in `app/` are the shorter read
and show what a context looks like from outside; the `state.rs` files are where the projections and
mutations are, and they are what you will copy.

## ⚡ Edges worth knowing before you hit them

**The match rule is syntactic.** A capitalised bare identifier in a pattern counts as a variant,
because syn cannot tell `None` from a binding without name resolution, and requiring `Option::None`
was a trap that cost an adopter a trip into the checker's source. The cost is that a capitalised
BINDING also passes: `match self.stage { Open => ..., Closed => ... }` over a `u32` draws no logic
diagnostic. rustc reports it as an unreachable pattern, which is a warning by default, so `spec/`
and `app/` carry `#![deny(unreachable_patterns)]` and the build fails rather than the program. The
boundary is still spelling, not meaning.

**A bool is not a vocabulary.** `match state.locked { true => .., false => .. }` is refused: `true`
is a literal pattern, not a variant, so the match is a branch and not a table. Use
`either(state.locked, yes, no)`.

**A match scrutinee is a place, not a call.** For example, `match found(&state.loans, *loan) { .. }`
fires `E-LOGIC-OUTSIDE-PATTERN` even when every arm is a variant. Bind the call first, `let out =
found(..);`, and match `out`. The message says so.

**A capacity is a `usize`.** `size` and `is_full` measure a collection in `usize`, so a spec
constant that bounds one, such as a number of bays, is declared `usize` and not `u32`, and the
comparison is, for example, `is_full(&state.rentals, LOCKER_BAYS)` rather than a count against a
cast.

**`either` is eager, and reads inside out.** `either(cond, yes, no)` is a call, so both arms are
built before it runs, and a chain of them in a `decide` arm evaluates every fault it might return.
That is fine when the arms are values and wrong when one is expensive or would panic. Use
`either_lazily(cond, yes, no)`, which takes two `fn` pointers and builds only the branch it returns.
Note also that a nested chain checks the OUTERMOST condition first, so a `decide` arm written this
way reads inside out; that is the cost of having no `if` and it is worth knowing before you write
four of them.

**A `let` binding is not logic.** Only `let ... else` is. Bind freely in a reducer.

**Arithmetic never panics and never wraps.** `raise_by` and `reduce_by` saturate, `mul` saturates,
and `div` and `rem` return `Option` because a zero divisor is a fault the caller names, not a crash.
All of them go through the `Floor` trait, which every integer type implements and which carries
`ZERO` and `ONE` so a pattern never conjures either. There is no wrapping form to pick by accident.
A money total that saturates is wrong, and it is wrong loudly at the type's ceiling rather than
silently at zero.

**A guard chain reads top to bottom with `refuse_when`.** A `decide` arm with several ways to fault
is a table, `refuse_when(&[(cond, Fault::A), (cond, Fault::B)], event)`, and the first true guard
wins in the order written. Nested `either` calls express the same thing inside out; `fines` shows
the table form. The table is eager, like `either`: every guard and the ok value are built before the
call. When a later guard is only safe once an earlier one has passed, use `refuse_when_lazily(state,
command, &[(<State as Guards>::taken, Fault::AlreadyRented), ..], event)`, whose guards are
`fn(&State, &Command) -> bool` and run only until the first refusal. A guard cannot be a free
function, since `spec/` holds none, and cannot be a closure; it is a method on a trait the context
declares in `state.rs`, `pub trait Guards { fn taken(&self, command: &Command) -> bool; }`,
implemented for `State` with a pattern call in its body, and named in the table by its qualified
path. Three things to know before you write one. A guard receives the whole `Command`, not the arm's
payload, so a guard that reads the command is a total match over `Command` whose other arms return
`false`; that is the price of a `fn` pointer with no capture. The same method may be called inline,
`<State as Guards>::taken(state, command)`, so a predicate two arms share is written once. The `ok`
value is still built before the call, as with `refuse_when`; only the guards are lazy. A table
passed straight to the call needs no annotation, because the parameter type coerces both fn items; a
table bound with `let` first does, `let guards: &[Guard<State, Command, Fault>] = &[..]`, and
`lockers` shows that form; `tests/fixtures/lazyguard` holds both forms and the rule tests build it.
A guard that reads no field may take `_command` and then applies wherever it is named; match on the
command only when the guard should hold for some arms and not others. Two limits: the fault type
must be `Clone`, because the table lends its faults; and a lookup that may be absent is not a guard,
so `let who = found(..); match who { Some(one) => refuse_when(..), None => Err(Fault::NotEnrolled)
}` keeps one `match` outside the table, as `fines` does.

**Queries copy by default.** Every query that returns items returns clones. Only `found`, `first`,
`keep` and `first_facet` have `_ref` forms, `found_ref`, `first_ref`, `keep_ref` and
`first_facet_ref`, which borrow instead, for a state large enough that the copy would show; the
ordering, queue and ledger patterns clone and have no borrowing twin yet.

## 🔍 What the compiler proves

Reach for these before adding a check. Half the work is already done.

| failure | error |
|---|---|
| a second reason for one item, held by `compile_fail/dup_because` and `raw_because` | E0428, name defined twice |
| a hand written impl of a blanket derived trait, `HasMax` or `HasMin` on a `Bounded` type, held by `compile_fail/conflict_impl` | E0119, conflicting implementations |
| a lint name nobody defines, prose wearing an attribute, held by `compile_fail/lint_prose` | unknown lint, forbidden in `spec/` and `app/`, which no inner `allow` can lift |
| a duplicate enum variant or name | E0428 |
| a non exhaustive match over a vocabulary | E0004 |

Prefer a trait impl to a const when a fact is relational. Coherence then does the uniqueness
checking at no cost. Read such facts as `<Hp as Bounded>::LO`.

## 🚦 The checks

When a check fires and the fix is not obvious, ask it:

```
cargo run -q -p premise_checker -- --explain E-ORPHAN-FILE
```

That prints the row from the table below and every passage of this manual that names the code, so
you never have to search this file for a code you just saw, and you get the reasoning rather than
only the rule. The failing run tells you the command.

| code | fix |
|---|---|
| `E-COMMENT` | the comment is a fact, so make it a `because!`, or it is noise, so delete it. A string carried by an attribute that is not `cfg`, `cfg_attr`, `derive`, `test`, `path`, `doc`, `should_panic` or `ignore`, a `deprecated` note, a `must_use` message or an `allow` reason, is a comment by another route. A lint name that is prose is the compiler's to refuse, so every `spec/` and `app/` crate root with a manifest carries `#![forbid(unknown_lints)]` on its first line and the checker refuses a root without it; an attribute that names `unknown_lints` anywhere else would lift that, and a lint under `clippy::`, `rustdoc::` or `rustc::` is a name the compiler never reads, so both are refused in those zones. A string bound to `_` or to a name starting with `_`, as a `const _: &str` or a `let _note = "..."`, is read by nothing and refused as a comment, as is a string local discarded into `_` or into a name starting with `_`, or any `let` in `spec/` whose initialiser carries a string, and so is a `#` line in any `Cargo.toml` under the workspace, or a `readme`, `keywords`, `categories` or `metadata` entry there, in the package, workspace or inheritance table, or a `description` that does not read as a reason: a registry demands one, so it is held to the checks a `because!` passes, enough words, words the crate name does not already say, no fact and no leaning on a document. An identifier of more than eight words in `spec/`, `app/` or `patterns/` is a sentence wearing a name, and a declared name that starts or ends with `the`, `which`, `nobody` or `it` is a fragment of one; the list holds the words no name in any domain edges, so `is_after`, `days_before` and `and_gate` pass and a fragment seamed on such a word passes with them, which is the bound the Known limits describe |
| `E-ASSEMBLED-NUMBER` | a value built from literal arithmetic is still a magic number. `2 * 2 * 2 + 1` is a spelling of 9, not a derivation, and `UNIT + UNIT + UNIT` is a spelling of a multiple; write the factor as a named constant. Arithmetic under any operator, shifts included, whose leaves are all constants valued zero or one is the same spelling, and so is a unit shifted by any named constant, since `ONE << N` is a power spelled as a derivation; the values are read across the whole spec, through a constant that only renames another too. A product or a difference of named constants that are not units is a relation and passes. A constant that only renames another, `const B: u32 = A;`, is resolved first, so `A / B` cancels as `A / A` does |
| `E-INLINE-LITERAL` | move the value to `spec/`, add `because!`, use the path. Every spelling counts: `0x64`, `1_000`, `0.5` and `1e9` are literals, `u32::MAX` is a value from nowhere in any spelling, a literal inside a `spec/` function body is a value the body should name as a const, a literal inside a `const _` assertion or inside a compound const that assertion names, whatever block, tuple or array wraps the arithmetic, restates a value the assertion should name, a spec type alias to a primitive or std type is resolved to its target so `Word::MAX` is judged as `u32::MAX`, a string in `spec/` anywhere but the whole value of a match arm or a body in a trait impl that renders a named thing is a fact in prose, a byte string is a number in every zone with the literal law, and a SHOUTY name reached through a glob from anywhere but the spec or the library is a value from nowhere until the file imports it from the spec by name |
| `E-LOGIC-OUTSIDE-PATTERN` | control flow, an operator, a method call, a cast, an index, a closure, an `async` fn or block, an assignment, a macro other than `vec!` and the reason macros, or a call to anything that is not a pattern. A call is a pattern only when its name is a public function the `patterns` crate exports, and a call through a qualified path, `<T as Trait>::f(..)`, is allowed only inside `spec/` for a trait the context declares, never from a binding; an import, module, struct, enum, trait or type named `patterns` shadows the library, so every call through it is a call to anything, and only `::patterns` reaches the crate. Run `catalog`, find one that fits, call it. Write a new pattern only if none does. For a CLOSURE, see "Getting a value into a pattern" above: the catalog cannot help you, because the problem is capture rather than a missing entry. A macro such as `matches!` is refused because its body carries patterns and guards no check reads. A local item named `patterns`, `patterns_macros`, `std`, `core` or `alloc` is refused under this code too, since every path through such a name would reach anything |
| `E-SPEC-FN` | `spec/` holds no free functions, no inherent methods, no trait default bodies and no `cfg(test)` helpers. Move the body to a pattern |
| `E-PATTERN-IMPURE` | the pattern names a project crate. Make that a parameter |
| `E-CASE-LEAK` | two bindings in one `app/` file whose bodies are identical except at a spec constant. A rule with several cases is one binding taking the case; enumerating them one binding apiece writes the values out again where `app/` may not write values, and the arity of the rule has leaked out of the spec. A `let` that only renames a constant is inlined before the comparison. The message names the constants rather than an edit distance |
| `E-DUP-PATTERN` | two patterns share a shape, whatever they are called. A pattern in a project that shares its shape with one in a `library =` crate is the same fault, reported against the library's file, since the library already holds it. A body is flattened before it is compared: a block around one expression is opened, `let x = e; x` becomes `e`, and so do `return e;` and `(e)`, and an `either` whose condition is a literal or whose two arms are the same becomes the arm it always takes, so a wrapper disguises nothing. Delete one, repoint its callers |
| `E-NEAR-PATTERN` | two composed patterns differ by a few tokens. The message names exactly which. Make that difference a parameter and delete one, or, if both must exist, make each reason name the other pattern and say what separates them; a pair is excused only when both reasons do. A method that implements a trait is excused, because the trait dictates its shape |
| `E-NO-BECAUSE` | ask for the reason and never invent one. If the value is derivable, derive it and the rule stops applying; a derivation is one whose every leaf names something, so `15 * ONE` and `u32::MAX` are choices, as are a literal wrapped in a block, a tuple field, a method call, an enum discriminant or a blanket impl. A trait impl with literal consts is its own choice: a reason on the type alone or on the trait alone does not cover it, so every concrete impl writes `decided!(Type, Trait, "...")`, one per impl; a reason on the trait covers only a blanket impl; and an inherent impl carrying literal consts is refused outright, since no reason can cover its consts one by one, so make them plain consts or a trait impl. A spec type alias whose target is a primitive or std type, `type Word = u32;`, is a choice of range and unit and carries a `because!` of its own. An array, tuple or struct const holding more than one literal, or an enum with more than one literal discriminant, or a trait or impl with more than one literal const, is several facts under one sentence, so it is built from named consts, each with its own reason. If there is nobody to ask, say so with `provisional!(ITEM, [SOURCE,] "what would settle this")`, which the census counts |
| `E-DUP-REASON` | one item carries two reasons, in different modules of the same crate. One item, one reason, wherever they are written; a `decided!` pair is an item, so two files deciding one impl is the same fault |
| `E-ORPHAN-REASON` | a reason names an item that is not declared in the file, or one that is only imported there, or a `decided!` is written in a file other than the one holding the impl it covers. Fix the name, or move the reason beside the declaration |
| `E-WEAK-REASON` | the reason restates the name or is too short. Say why this value and not another |
| `E-FACT-IN-REASON` | a number written into a reason's prose, or into any string in `spec/` or `patterns/`, a `Named` text included, where a digit in a word counts too, and in any script; in `patterns/` the primitive type names, `u32` and its kin, are exempt, since a library message may name a type, and in `spec/` they are not, since a rendered text names no Rust type. That value now lives outside the spec, where nothing checks it and nothing keeps the two copies equal: change the constant and the sentence still says the old figure. Move it into a `const`, or into a `source!` item if it names the study, contract or release the reason rests on, and cite that instead |
| `E-UNDECLARED-SOURCE` | a reason cites an identifier the file does not declare, or its prose names a review, survey, study, contract, agreement, regulation, minutes or audit while citing nothing. What a reason leans on is a fact like any other, so two reasons leaning on one study should lean on one item, not two sentences that will drift. Declare it with `source!` and cite it. Prose that names a source by another word is not caught; that is the limit of a word list |
| `E-DEAD-SOURCE` | a `source!` that no reason anywhere in the tree cites, so a source in `contexts/shared/` cited from two contexts is alive; or a struct, enum, trait or type alias in `spec/`, at any depth, that carries a `because!` rather than a `source!` and is named nowhere but where it is declared, since a fact nothing uses is a comment wearing a reason. One mention is the declaration itself, so an item a sibling names in the same file is alive. A source exists so that several reasons lean on one item; one nothing leans on is a citation somebody forgot, or a thing already declared under another name. Cite it or merge it |
| `E-PATTERN-NO-REASON` | a public function in `patterns/` with no `because!`. A pattern is a decision and decisions carry reasons, and `E-NEAR-PATTERN` excuses a pair only when each reason names the other, so a pattern without one cannot be excused by an adopter who collides with it |
| `E-UNTRACEABLE` | a `because!` on a constant or static in `patterns/` cites no `source!` item. A pattern constant answers to something outside this repo, and that thing is an item, not a phrase. Visibility is irrelevant: a private algorithm constant answers to the paper it came from exactly as a public one does |
| `E-CONTEXT-LEAK` | reach the other context through its `vocabulary` only. A context is exactly `mod.rs`, `vocabulary.rs`, `state.rs` and `reducer.rs`; a fifth file is refused at any depth under the context, and any own item other than the vocabulary counts as state. A `use ... as` alias is expanded before the check; a vocabulary that re-exports, aliases or names its own `state` or `reducer` in any item that is not private, impls included, is a leak from the other side; and a module under a vocabulary, inline or as a `vocabulary/` directory, is refused, because anything in it would be reachable as vocabulary. A `pub use` of a glob, or of the context module itself, from a vocabulary is a leak, since it hands over every module the context holds; a bare `state::` or `reducer::` path in a file that glob-imports another vocabulary is judged as the module it names; `self::` resolves to the file's own module, not the crate; the `contexts` module holds `mod` items and nothing else, since anything else there is a bridge; and a glob of `contexts` itself is refused. A marker is vocabulary, but `<Marker as Context>::State`, or `replay::<Marker>` and the other three drivers, reach the marker's context through the library rather than the module, and are refused for another context's marker, through a local `type` alias too; a public alias to another context's item, a glob import of another context's vocabulary, and a type parameter bounded by `Context` are refused, since each is a route to the same projection. Behind all of those, the marker itself, the type an `impl Context` is written for, is named nowhere in `spec/` outside its own context: not as an alias target, a type argument, an impl target, a re-export, a `shared` item or a `decided!` subject, so no wrapper carries that context's state anywhere. Beneath that again, `spec/` names types by path only: a projection `<T as Trait>::Item` stands on a type the context owns or on one of shared's, a projection through a type parameter, `T::Item`, and a projection of a projection are refused, and `handle`, `handle_all`, `replay` and `replay_from` from the library are not called in `spec/` at all, since app drives contexts; a local that happens to share a name is a local. A `use ... as` rename is expanded to a fixed point, and a path that resolves to no context and to no crate root is an unresolved name, which no context owns; a name brought in by a glob of `shared` is shared's. A marker name is one fact, so two contexts whose markers share a name are `E-DUP-NAME`. A path through a vocabulary is judged as a module only when the segment after `vocabulary` is lowercase, so `other::Event::Variant` through a renamed import is the intended style |
| `E-PARSE` | the file is not valid Rust, or is not UTF-8 text. Fix the syntax; nothing else can be checked until it parses |
| `E-SPLICE` | an `include!`, an `include_str!`, an `include_bytes!`, an `env!` or a `#[path]` attribute pulls a file or a build-time value into this zone, so a fact enters that no law reads |
| `E-MACRO-DEF` | a `macro_rules!` in `spec/` or `app/` hides its body from every check. Define macros in `patterns/` |
| `E-ZONE-DEPENDS` | `patterns/` or a `tools/` crate depends on `spec/` or `app/`, which only `app/` may; or `spec/` or `app/` depends on a crate no zone names, so anything it re-exports, aliases or wraps enters from outside every law |
| `E-ZONE-MISSING` | `premise.zones` names a crate that is not a workspace member, or a `library =` path with no crate manifest under it, usually a typo that silently disables a zone |
| `E-ZONE-UNKNOWN` | a source file belongs to no zone, so no law applies to it. Add its crate to the workspace members and to `premise.zones` |
| `E-ZONE-KEY` | a line in `premise.zones` whose key is not a zone, or that has no `=` at all, since the file holds zone lines and nothing else. A crate named under two zone keys is the same fault, since the first line would silently decide its laws. The line does nothing, so the crate it names is under no law. The zones are `patterns`, `spec`, `app` and `tools`; `tests` is derived from a crate's own test target and never keyed |
| `E-DEFAULT-MEMBERS` | `default-members` in the workspace manifest. It takes a crate out of every gate step, type checking included, while leaving it on disk for the checker to read |
| `E-CRATE-NESTED` | a workspace member whose directory sits inside another member's. A file under it answers to whichever crate has the longer path rather than to the one rustc compiles it into, so zones stop meaning anything. Move the crate out |
| `E-MODULE-CASE` | `mod door;` resolving to `Door.rs`. Windows and macOS ignore filename case, so this compiles here and fails on Linux CI. Rename the file to match the `mod` |
| `E-TARGETS-UNRESOLVED` | `cargo metadata` did not answer, so the checker fell back to reading the manifest itself. That fallback is a second opinion about which files are the program, and it has been wrong before, so the run is unchecked rather than clean |
| `E-OUTSIDE-LAW` | a public re-export in `spec/` or `app/` from a crate no zone names. The re-exported item becomes spec vocabulary that no law governs and no check reads, whether it came from a path dependency, a registry crate or a vendored one |
| `E-OUTSIDE-WORKSPACE` | a path dependency that resolves outside the workspace root and into no `library =` crate. No zone reaches it, so its code answers to no law while the crate that depends on it re-exports the result |
| `E-OUTSIDE-SRC` | a governed crate reaching outside `src/`: an example, a bench or a build script that `cargo build` never compiles, or a lib or bin target whose resolved path leaves `src/`, which lands the crate's real code in another zone's laws or in none. The path is resolved, not matched as text, so `..` buys nothing. Logic belongs in a `tools/` crate |
| `E-CONDITIONAL` | a `#[cfg]`, `#[cfg_attr]` or `cfg!` anywhere, other than `cfg(test)`. It makes the set of facts depend on a build flag, so rustc compiles one program while the checker reads another. `cfg(test)` is allowed because the gate runs `cargo test`. Decide once, in code |
| `E-ORPHAN-FILE` | a `.rs` file under a crate that no `mod` declares. rustc never compiles it, so it is not part of the program, yet the checker reads it and reports on it. Declare it or delete it |
| `E-DUP-NAME` | two spec facts share a name, whatever their visibility. A const, a static or a `source!` is compared across the whole spec, because it is one fact for the whole program; an associated const is compared within its context, because its type already qualifies it, and also against every plain const, so `pub const Plan_MAX` beside `impl Slots for Plan { const MAX }` is the same collision. One name, one fact; the module system is not a namespace for facts |
| `E-DUP-VOCABULARY` | two contexts state the same vocabulary, or one has drifted a variant from another, whatever the enum's visibility; a spec file outside `contexts/` counts as one more context. The diagnostic is filed on both sides, so whichever context you own sees it. Move it to `contexts/shared`, or state a `because!` on the enum saying why they are genuinely different |
| `E-SHARED-SCOPE` | `contexts/shared/` is a directory holding `vocabulary.rs` and nothing else, and that file holds data-less enums, unit structs, consts, reasons and impls of library traits whose bodies are a single total match; a struct or enum with fields, a type alias, a local trait, an `impl Context` under any name, or a method body with logic is state or a rule, and those belong to a named context |
| `E-CARGO-CONFIG` | a `.cargo/config.toml` or `.cargo/config` under the workspace that sets a runner, a compiler, a wrapper, flags, an environment, an alias, a patch or a source. Every one changes what cargo runs or compiles when the gate calls it, so a red tree could print green. Delete the file; the gate is the only cargo configuration a governed workspace has, and it runs the four laws in-process so a config cannot silence them. A `rust-toolchain.toml`, or the extensionless `rust-toolchain` rustup prefers, that sets `path` is the same fault, since it selects a compiler shipped inside the repository. The rule reads the workspace only; a config above the root or in the cargo home is outside what the checker can see |
| `E-IDENT-SCRIPT` | an identifier in a governed zone with a character outside ASCII. A name that looks like another name is the duplicate the name rules exist to catch, and no check can compare what it cannot read. Spell it in ASCII |

## 🧾 A relation is not a reason

Three things a reason can say about another constant, and only one of them belongs in prose.

**A definition.** "MAX_HP divided by WAVE_CAP" when the value is exactly that. Write the derivation
instead: rustc computes it, it cannot disagree with its inputs, and `E-NO-BECAUSE` stops applying
because a consequence needs no reason.

**A constraint.** "must divide MAX_HP evenly so no wave ends in a stub" restricts the value without
determining it, so a derivation would be wrong. Write the constraint as code:

```rust
const _: () = assert!(MAX_HP % SLOTS == 0);
```

That is compile-time checked, which prose is not, and it is a better outcome than either.

**A divergence.** "was MAX_HP divided by WAVE_CAP until the rebalance" is a value that deliberately
is not the expression. Do NOT turn it into a derivation; that changes the value. Record it with
`rejected!(ITEM, "MAX_HP / WAVE_CAP", "changed at the rebalance")`, which moves the divergence out
of prose into a structured fact.

**A value nobody has decided yet.** `E-NO-BECAUSE` says to ask rather than invent, and if you are
working with nobody to ask, that instruction has no legal answer. Inventing a reason is worse than
leaving one out, because a fabricated `because!` reads as settled policy that somebody decided.

```rust
pub const COLLECT_DAYS: u32 = 7;
provisional!(COLLECT_DAYS, "chosen as one visit cycle, but no branch has confirmed its cycle; the desk supervisor would settle it");
```

`provisional!` satisfies the reason rule and says plainly that the value is yours rather than
anyone's. The census counts them, so a spec carrying undecided values reports how many, and they
cannot quietly become policy by sitting there. Say what would settle it, not what you guessed. It
takes a citation the same way `because!` does, `provisional!(ITEM, TariffReview, "...")`, for the
common case where the document that would settle it is already a `source!`; without that slot the
source would sit uncited, which `E-DEAD-SOURCE` refuses.

**A supersession.** A rule that changed on a date, where the old value is still owed on old records,
is not a rejection: both values are live and both are true, in different periods. Two constants,
each with its own reason, and the relation between them stated in code:

```rust
pub struct ConcessionAgreement;
source!(ConcessionAgreement, "the concession agreement that first fixed the minute rate");

pub struct TariffReview;
source!(TariffReview, "the tariff review that reset the rate against operating cost");

pub const LEGACY_PENCE_PER_MINUTE: u32 = 15;
because!(LEGACY_PENCE_PER_MINUTE, ConcessionAgreement, "the rate it fixed at the outset");

pub const TARIFF_DAY: u32 = 90;
because!(TARIFF_DAY, TariffReview, "the day of the operating calendar it named for the new rate to run from");

pub const PENCE_PER_MINUTE: u32 = 22;
because!(PENCE_PER_MINUTE, TariffReview, "the rate it set against operating cost");
supersedes!(PENCE_PER_MINUTE, LEGACY_PENCE_PER_MINUTE, TARIFF_DAY, "the review that reset it");
```

`supersedes!` names every item it mentions, so rustc refuses to compile if any is renamed or
deleted, and the ordering is stated once beside the values rather than only implied by whichever
`decide` arm selects on the era. Nothing ties the two together mechanically; the record is what a
reader checks the arm against. The third slot is the constant that says WHEN, and it is required:
`E-FACT-IN-REASON` forbids writing the day into the prose, so without a slot for it the rule would
move the number out of the sentence and into nothing. Name the constant and the sentence can say
what the change was for. Do NOT reach for `rejected!` here: that records a value nobody chose, and
an old tariff you still owe money at is a value someone very much chose.

That is guidance for you, and nothing checks it. Checks that tried, by parsing the arithmetic out of
a reason and evaluating it, were deleted after being measured. What they taught is that the English
in a reason is the one place in Premise where meaning is not mechanically checkable, which is why
the reasons are there at all.

`E-WEAK-REASON` and `E-UNTRACEABLE` raise a floor, nothing more. They count words and check that a
reason cites something. `because!(CAP, "zorble frobnicate quux gribble")` passes both. A green gate
means no reason was left blank or written as a restatement of its own name; it does not mean the
reasons are true, and treating the check as proof of that is worse than not having it. Reading the
reasons is still a human job.

### The six reason macros

Each takes item names, commas and string literals, nothing else, and refuses anything else at
compile time with a message naming the slot and the shape. A path is refused: bring the item into
scope and name it. A number or a char where a sentence belongs is refused. A missing or extra slot
is refused. Raw identifiers work. A single trailing comma is allowed.

| macro | shape |
|---|---|
| `because!` | `because!(ITEM, [CITE, ...], "reason")` |
| `source!` | `source!(ITEM, "what it is")` |
| `provisional!` | `provisional!(ITEM, [CITE, ...], "what would settle it")` |
| `rejected!` | `rejected!(ITEM, "alternative", "cost")` |
| `supersedes!` | `supersedes!(NEW, OLD, WHEN, "why")` |
| `decided!` | `decided!(TYPE, TRAIT, "reason")`, the reason for one concrete impl whose associated consts are literals; one per impl, so two impls on one type carry two, and a marker-parameterised trait is written with its marker, `decided!(Plan, Cap<ByCap>, "...")`, so each marker impl carries its own |

Every one expands to a `use` of every item it names, so rustc refuses a reason, a rejection or a
supersession that names something that does not exist, and a rename breaks every reason that leaned
on the old name. Three of the six crates in `tools/checker/tests/compile_fail/` test the macros:
`malformed` holds a malformed call for every refusal across the six macros and asserts each message,
`raw_supersedes` holds well formed raw-identifier calls that must build, and `ghost_rejected` names
a ghost item and must not. The other three hold the compiler's own refusals, listed under "What the
compiler proves".

## 🚫 A reason is not a place to put facts

The reasons were the last hole in law 2, and it took someone reading one to see it:

```rust
pub struct Warehouse;
because!(Warehouse, "the offsite store the 2025 relocation created");
```

That sentence carries three facts. `2025` is a number living outside the spec, which
`E-INLINE-LITERAL` bans everywhere else and `because!` let straight back in. "The relocation" is an
event, and if two constants both came from it the event is written twice and the copies will drift.
"The offsite store" is an entity the vocabulary should already name. Prose in a reason is a second,
unchecked model of the same domain the spec is modelling in types.

So a reason has two parts, and only one of them is prose. What it rests on is an item:

```rust
pub struct Relocation2025;
source!(Relocation2025, "the move of stock offsite, and the sites it left behind");

pub struct Warehouse;
because!(Warehouse, Relocation2025, "the store that move created");
```

`E-FACT-IN-REASON` refuses a number in the prose, in `rejected!` and `supersedes!` as much as in
`because!`. `E-UNDECLARED-SOURCE` refuses a citation to something nothing declares, and a reason
whose prose names a review or a study while citing nothing. `E-UNTRACEABLE` refuses a pattern
constant that cites no `source!` item; citing a type is not citing a source. `E-DEAD-SOURCE` refuses
a source nothing cites. Two constants from one study now cite one item, and renaming or deleting the
study breaks every reason that leaned on it, because `source!` and `because!` both expand to a use
of the name.

A `source!` cited exactly once is a smell. Either the reason did not need a citation, or the thing
it cites is one you have already declared under another name and the two should be merged. The value
of a citation is that several reasons lean on one item; a citation with one citer is two lines of
ceremony that buy nothing.

`E-FACT-IN-REASON` fires in two kinds. Most often the prose names a dated source in words, a survey,
a review, a policy, a contract, an act, a study, which is the case `source!` exists for. The rest
restate the constant's own value: `COLLECT_DAYS: u32 = 7` carried the reason "7 days is one whole
visit cycle", so the seven was written twice and changing the constant would leave the sentence
saying the old figure.

The rule looks for a word that begins with a digit, so it under-catches: "three weeks" and "sixty
slots to each aisle bay" both pass, and both are facts, while `u32` and `E0428` pass because they
are names. Spelled numbers are deliberately not scanned, because "one queue", "a second pass" and
"half the shelf" are ordinary English rather than values, and a rule that fires on those is turned
off within a week. Silence on a real fact costs a stale sentence; noise on a determiner costs the
whole check. Expect it to fire often on an existing codebase, including on prose written by authors
who had read this manual.

Where this bottoms out, stated rather than hidden. A `source!` still holds a sentence, and that
sentence still names things. "The move of stock offsite" names stock and sites. The discipline is
that a noun the spec models is written as the item, and a noun the spec does not model is why the
sentence exists at all. You cannot define the outside world in Rust, and pretending otherwise would
put the drift somewhere harder to see. What you can do is make sure the same outside thing is
referred to once, by name, everywhere it matters, and that is what E-FACT-IN-REASON,
E-UNDECLARED-SOURCE and E-UNTRACEABLE buy.

## 👯 The near duplicate trap

This section is near the end on purpose. It is a maintainer's concern, about a library rotting over
years, and two agents building their first feature here each spent their largest single block of
planning on it before discovering it never fired for them. It is long because the rule is measured,
not because it is likely. Write the pattern you need; if two collide you will be told, with both
names, the edit distance and the differing tokens, and the fix takes a minute.

**Why the duplication rules stop at `app/`.** They do not run in the binding zone, and this is a
decision with evidence behind it rather than an oversight. A binding is a single pattern call by
construction, because that is what the zone is for, so two bindings that each wrap one call are not
duplication: they are the layer doing its job. The rule's remedy, "parameterise the difference and
delete one", has no meaning there. Parameterise `sold { size(&state.tickets) }` against `depth {
size(&state.queue) }` and you get `size`. Parameterise `current { replay::<Ticket>(e) }` against
`replay::<Queue>(e)` and you get `replay`.

This was tried. The rules were switched on in `app/` for one commit, and two adopters working
independently hit the same wall within the hour: one deleted six bindings and reached past `app/`
into `patterns::` with a turbofish from its tests, the other abandoned `current`/`step`/`steps` for
a single fused call. Both abandoned the shape this manual demonstrates, to satisfy a rule the manual
demonstrates breaking. A gate that fires on its own worked example is worse than a gate that misses
something, so it was reverted.

`E-CASE-LEAK` notices the thing this rule was reaching for and could not reach. The discriminator
came from an adopter who watched the reverted rule fire on their own work: every false positive
differed at a type, which is how contexts stay apart and is legitimate, while the real defect
differed at a spec constant, which `app/` is forbidden from writing. So two bindings identical
except at a constant are the leak, and two identical except at a type are the layer doing its job.

It compares only bindings in one file, which is one context's binding surface. Across files the same
shape is two contexts capping their own money by their own cap, and telling those two to share one
binding would push the choice of cap into the caller. That was checked before the rule shipped, by
writing the parallel pair and confirming silence; the pair is `tests/fixtures/parallelcap` and it
asserts silence still.

What `E-NEAR-PATTERN` and `E-DUP-PATTERN` actually see, because you should not have to read
`shape.rs` to predict it. `E-CASE-LEAK` above uses the same profile but its own thresholds: two
bindings of any length, equal token for token except at the constants. Two functions are compared as
token sequences with every bound identifier alpha-renamed to its position, so parameter and local
names never count as a difference, and the function's own name is replaced by a fixed one, so two
identical bodies under two names are one shape. What survives verbatim is any identifier after a `.`
or `::`, which is to say the methods and patterns it calls. Everything else, including turbofish
TYPE arguments, is normalised away: to the comparison, `count_facet::<BySize, Rental>(&s.rentals,
x)` and `count_facet::<ByContents, Salvage>(&s.log, y)` differ only in the field name. The threshold
is `MAX_DRIFT`, two token edits, and a pair under `MIN_SHAPE_TOKENS`, twelve tokens, is never
compared at all. Callee sets must overlap, so two functions calling nothing in common are never a
pair.

The consequence worth stating plainly: a marker type in a turbofish is invisible here, so two
projections that differ only by their marker will read as one edit apart. That is a real limit of
comparing shape rather than resolved types, and it is why this rule does not run in `app/`.

`E-NEAR-PATTERN` has been measured at scale, because its failure mode is the gate blocking correct
work, which is the failure that gets a tool switched off. Against a generated library of 240
patterns and 28,680 pairs: patterns that differ by an operator never collide, at any size. Every
collision came from one family, patterns differing only by the name of a function they call, and
those are the pairs the rule exists to find. So the risk is not scale, it is shape. A pair like
`first` and `last`, distinct patterns that differ by one callee name over the same body, will be
flagged and you will have to answer for it; a pair like `add` and `sub` never will, because `+` and
`-` are punctuation and the rule requires every difference to be an identifier. Two tests in
`tools/checker/tests/scale.rs` hold both halves of that: the operator families must stay clean, and
the callee family must still fire.

The way a pattern library rots is not exact copies, which `E-DUP-PATTERN` already stops. It is a
pile of one token variants: `settle_up` and `settle_down`, then `settle_up_capped`, until nobody can
tell which to call.

`E-NEAR-PATTERN` catches this. Two patterns that **compose other patterns** and differ by at most
`MAX_DRIFT` tokens are one pattern with a missing parameter. The diagnostic names the differing
tokens, so the fix is mechanical: promote that token to an argument, delete the second pattern,
repoint its callers.

```
E-NEAR-PATTERN patterns/src/settle.rs:14 settle_down is 1 edit from settle_up at patterns/src/settle.rs:7 (@reduce_by not @raise_by). Parameterise the difference
```

Here the difference is which function is applied, so the fix is to take it as a parameter: `fn
settle<T>(v: T, floor: T, roof: T, step: fn(T, T) -> T) -> T`.

When both really must exist, say so on both: a pair is excused only when each reason names the other
pattern by name, so the excuse is a decision two reasons record rather than a coincidence of two
reasons existing. Every pattern carries a reason, so a rule that excused any reasoned pair would
never fire; `tests/fixtures/nearexcused` and `nearunexcused` hold the two halves.

Patterns that call no other pattern are **primitives**, the axioms of the library, and are exempt.
`clamp_upper` and `clamp_lower` differ only by `>` and `<`, and forcing them to merge would be
wrong: `<` and `>` are irreducibly different. A method call on a trait is a primitive too. Only
composition is policed.

## ⚠️ Known limits

Every hole the tools admit, in one place, each pointing at the section that owns it, so nobody has
to discover one by falling into it.

- **No check reads what a reason means.** See "A relation is not a reason".
- **The match rule is spelling, not meaning.** See "Edges worth knowing before you hit them".
- **`E-FACT-IN-REASON` sees digits, not numbers.** See "A reason is not a place to put facts".
- **`E-UNDECLARED-SOURCE` sees a word list.** See the `E-UNDECLARED-SOURCE` row of the checks
  table; policy, standard and release are not on the list because they are ordinary English as
  often as they are documents.
- **Shape comparison is blind to marker types.** See "The near duplicate trap".
- **Queries copy.** See "Edges worth knowing before you hit them".
- **A file that names something `patterns` loses the short spelling.** Every call through the
  library must then be written `::patterns::name`, and the diagnostic says so.
- **Only `vec!` has its arguments read.** It is the one macro a binding may call, and its
  arguments are parsed as expressions; a macro added to that allowance later would need the same
  treatment or its body would be invisible again.
- **The smuggle suite is a list.** `tools/checker/tests/smuggle.rs` injects every bypass
  spelling anyone has found into a copy of the live spec and app on every gate run and asserts
  each is caught. A spelling nobody has found yet is not on the list; when one is found, it goes
  on the list before the checker is fixed, so the fix is measured.
- **The checker asks cargo which files are the program on every run.** That answer costs about
  30 ms, it is the largest cost left in a suite that checks many small trees, and law four is why it
  is not replaced by reading the manifest. See "When the gate gets slow".
- **This file is a build input.** `tools/checker` embeds it with `include_str!`, so the checker will
  not build in a tree without `README.md` beside `patterns/`. That is how `--explain` answers
  without a second copy of the check table. Vendor the two together.
- **The thesis is unproven.** See "Measuring the thesis".

## 📐 Measuring the thesis

The claim is that a reader orients faster here than in a documented codebase. `orient` is the
instrument:

```
cargo run -q --bin orient
cargo run -q --bin orient -- --quiz
```

It generates orientation questions from the spec itself, "why is this constant the value it is",
"which cases must a match over this vocabulary cover", "which context declares this", so the set can
never be stale and every answer is one lookup away if the thesis holds. `--quiz` times each answer
and appends one line to `premise.orient`, which is tracked like the census, so the cost of
orientation is in git history rather than in anyone's memory. Ask the same questions of a documented
codebase in the same domain and the comparison is a table rather than an argument. Until somebody
does that with a real team, the thesis stays a claim.

## 📋 Rules for working here

- Run `catalog` before writing logic. Search before writing:
  `cargo run -q --bin catalog -- count above` narrows to entries matching every word given, across
  the signature and the module name.
- Prefer a derivation to a stated value. Prefer a trait impl to a const.
- Never invent a reason. A fabricated `because!` reads as a decision nobody made. With nobody to
  ask, use `provisional!`.
- Never weaken a check to make a run pass.
- Put the check next to the claim. A statement in this manual that a tool could verify gets a test
  in `tools/checker/tests/manual.rs`, so the manual cannot drift from the tools either.
- Idempotent. A second run on a clean workspace changes nothing.

## 📊 The census

`premise.census` is one tracked line with five counts: pattern functions, governed lines, escaped
lines, test lines and provisional values. The checker and the gate both compute it on every run
through one shared routine, print one line after the diagnostics if it has moved, and say nothing
otherwise. A census that cannot be written is reported as such and fails the laws step, rather than
reported as moved.

Escaped lines are the ones in `tools/` crates you wrote, excluding the ones the toolchain ships.
Test lines are the other quiet exit: `tests/` is a zone with every law off, so a function that will
not fit the rules can move there instead of into a pattern. It is not a fault on its own, and a
healthy project has plenty; a jump in it beside no jump in the governed count is the signal. Logic
placement is waived in `tools/`, which makes it the pressure valve: an agent that hits
`E-LOGIC-OUTSIDE-PATTERN` at the end of a long task can write the projection trait, or it can move
the code into a `tools/`-shaped crate and be done. The second is legal and it is how this design
fails quietly. A rising escaped count is that happening, and it is visible in git history rather
than in anyone's memory.

A rising pattern count is the other signal, and it is not a fault: it means the library grew, and
the rule is to search before writing. When the count moves, run `catalog` before writing another
pattern. That nudge exists because a catalog reading goes stale while you work, and the near
duplicate you are about to write is one somebody added since you last looked.

The checker updates the file itself, so there is no step to remember and no shared line for two
agents to fight over. The change lands in your diff, which is where somebody notices it. The point
is not the number, it is that a movement in it is visible in git history rather than in nobody's
memory.

## 🤖 Handing a feature to an agent

```
Read <workspace>/README.md. That is the manual and it is meant to be sufficient.

Then read the context in spec/src/contexts/ closest to your problem before you design anything.
The manual names what each one demonstrates.

Build: <the feature, in plain English, including the awkward parts>

Make the modelling judgement calls yourself, the way the manual tells you to.
Verify with `cargo run -q -p premise_gate`. It must pass.

Two things you will hit:
- You have no user to ask. Where the manual says never to invent a reason, use
  provisional!(ITEM, [SOURCE,] "what would settle this"), citing a source! in the second slot when
  one exists, rather than writing a reason that reads as a decision somebody made.
- If other agents are in the tree, run `cargo run -q -p premise_gate -- --scope <your context name>`
  so a red gate somebody else caused does not look like yours. It still is not a pass: exit status 3
  means the tree is broken elsewhere and your work is unverified until it is green.
```

Two things change what you get, and both are counterintuitive.

**Include the awkward clauses deliberately.** "The rate changed on a date, with the old rate still
owed on older records" exercises `supersedes!` and produces the sharpest modelling. "Two tiers with
different limits" forces a relational fact rather than two loose constants. Smoothing those out gets
you a smoother spec that teaches the agent less.

**Do not say how to model it.** Every agent told to make its own judgement calls modelled better
than a specification would have: one collapsed two commands into one because they are the same event
for that queue, another turned "higher for staff" into a compile time assertion instead of prose.

Expect roughly 70% understanding and 30% building on a first feature, inverting on the second. That
is the entry cost, reported independently by two cold readers.

## 🔧 Notes for the installer

Copy `patterns/`, `patterns_macros/` and `tools/` into an empty directory, with this file beside
them. Add a workspace `Cargo.toml` listing those crates plus `spec` and `app`, and a `premise.zones`
mapping each crate name to its zone. Then `cargo run -q -p premise_gate`.

Verified end to end: that plus one constant with a reason gives `gate clean, 5 steps`. One thing
bites on that first run and it is the gate working. A first reason often restates the constant's own
name, which `E-WEAK-REASON` catches: a free minutes constant explained as "the free period it fixes"
spends both its words on the name and says nothing, and the check is right to refuse it.

`patterns/` depends on `patterns_macros` and nothing else, and `patterns_macros` depends on nothing
at all, so the pair drops into any Rust workspace unchanged. The checker uses `syn`, `proc-macro2`,
`quote`, `toml` and `serde_json`, which stay inside `tools/` and never reach the code you write.

Give every crate a zone in `premise.zones`, keyed by crate name. A crate in no zone is
`E-ZONE-UNKNOWN`, which is deliberate: a file no law governs is worse than a file that breaks one.

## 🌍 If a fact has to leave Rust

It does not, today. Everything is Rust, and `app/` reads a constant such as
`spec::contexts::fines::vocabulary::LOAN_DAYS` directly, so there is nothing to keep in sync and
nothing to drift.

A cross language generator existed and was removed. It emitted TypeScript from the spec and verified
its own output, and it worked, but it was the largest and most defect prone component in the tree:
every naming collision, every duplicate export, every silent mistranslation came from there. If a
fact ever has to reach a consumer that is not Rust, compile the spec to wasm rather than restating
it in another language. A second copy that is generated is better than one that is written; no
second copy at all is better than either.

## 📄 License

MIT. See [LICENSE](LICENSE).

On Windows, `python` resolves to a Microsoft Store stub that exits without running anything. `sed`,
`perl` and `awk` work through the bundled Git Bash. Nothing in the toolchain needs python; this is
only worth knowing if you are scripting around it.
