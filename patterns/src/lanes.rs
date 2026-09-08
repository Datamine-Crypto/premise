use patterns_macros::because;

pub fn parted<T>(items: &[T], lanes: usize) -> Vec<&[T]> {
    if lanes == 0 || items.is_empty() {
        return Vec::new();
    }
    items.chunks(items.len().div_ceil(lanes)).collect()
}
because!(parted, "a sequence cut into at most that many pieces of near equal size, rounding the piece up so the last one is the short one rather than an extra piece beyond the count asked for; a request for no pieces gives none rather than dividing by nothing");

pub fn lanes_here() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
because!(lanes_here, "how many pieces of work this machine will really run at once, asked in one place so a caller neither hard codes a count nor has to decide what to do when the machine declines to answer, in which case one lane is the sequential run every caller already expects to be correct");

pub fn across<T: Sync, R: Send>(items: &[T], work: fn(&T) -> R) -> Vec<R> {
    let mut out: Vec<R> = Vec::with_capacity(items.len());
    std::thread::scope(|lane| {
        let mut running = Vec::new();
        for part in parted(items, lanes_here()) {
            running.push(lane.spawn(move || part.iter().map(work).collect::<Vec<R>>()));
        }
        for one in running {
            match one.join() {
                Ok(done) => out.extend(done),
                Err(panicked) => std::panic::resume_unwind(panicked),
            }
        }
    });
    out
}
because!(across, "every item worked on over the whole machine, answering in the order the items came rather than the order the lanes finished, so a caller may swap it for map and read the same answer; a lane that panics is raised again here rather than swallowed, because a silent lane is a test that passed by not running");
