pub fn pick() -> u32 {
    if cfg!(windows) { 0 } else { 1 }
}
