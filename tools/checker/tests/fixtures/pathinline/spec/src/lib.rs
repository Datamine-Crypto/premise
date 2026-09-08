#![forbid(unknown_lints)]
pub mod outer {
    #[path = "target.rs"]
    pub mod inner;
}
