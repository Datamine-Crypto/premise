#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diag {
    pub code: &'static str,
    pub file: String,
    pub line: usize,
    pub note: String,
}

impl Diag {
    pub fn new(code: &'static str, file: &str, line: usize, note: impl Into<String>) -> Diag {
        Diag {
            code,
            file: file.replace(std::path::MAIN_SEPARATOR, "/"),
            line,
            note: note.into(),
        }
    }
}

impl std::fmt::Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}:{} {}", self.code, self.file, self.line, self.note)
    }
}

pub fn code_of(line: &str) -> Option<&str> {
    let head = line.split_whitespace().next()?;
    match head.starts_with("E-") {
        true => Some(head),
        false => None,
    }
}
