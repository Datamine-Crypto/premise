pub const BSLASH: char = 92u8 as char;
const CHAR_LITERAL_WIDTH: usize = 3;
patterns_macros::because!(
    CHAR_LITERAL_WIDTH,
    "what a quoted single character spans, opening tick, glyph, closing tick"
);

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn skip_plain(ch: &[char], mut i: usize, mut line: usize) -> (usize, usize) {
    i += 1;
    while i < ch.len() {
        if ch[i] == BSLASH {
            if i + 1 < ch.len() && ch[i + 1] == '\n' {
                line += 1;
            }
            i += 2;
            continue;
        }
        if ch[i] == '\n' {
            line += 1;
        }
        if ch[i] == '"' {
            return (i + 1, line);
        }
        i += 1;
    }
    (i, line)
}

fn skip_raw(ch: &[char], start: usize, mut line: usize) -> Option<(usize, usize)> {
    let mut i = start;
    let mut hashes = 0usize;
    while i < ch.len() && ch[i] == '#' {
        hashes += 1;
        i += 1;
    }
    if i >= ch.len() || ch[i] != '"' {
        return None;
    }
    i += 1;
    while i < ch.len() {
        if ch[i] == '\n' {
            line += 1;
            i += 1;
            continue;
        }
        if ch[i] == '"' {
            let mut k = 0usize;
            while k < hashes && i + 1 + k < ch.len() && ch[i + 1 + k] == '#' {
                k += 1;
            }
            if k == hashes {
                return Some((i + 1 + hashes, line));
            }
        }
        i += 1;
    }
    Some((i, line))
}

fn skip_quote_atom(ch: &[char], i: usize, line: usize) -> (usize, usize) {
    let n = ch.len();
    if i + 1 < n && ch[i + 1] == BSLASH {
        let mut j = i + 2;
        while j < n && ch[j] != '\'' {
            j += 1;
        }
        return (j + 1, line);
    }
    if i + 2 < n && ch[i + 2] == '\'' {
        return (i + CHAR_LITERAL_WIDTH, line);
    }
    (i + 1, line)
}

pub fn find(src: &str) -> Vec<usize> {
    let ch: Vec<char> = src.chars().collect();
    let n = ch.len();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut prev_ident = false;
    while i < n {
        let c = ch[i];
        if c == '\n' {
            line += 1;
            i += 1;
            prev_ident = false;
            continue;
        }
        if c == '/' && i + 1 < n && ch[i + 1] == '/' {
            out.push(line);
            while i < n && ch[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < n && ch[i + 1] == '*' {
            out.push(line);
            i += 2;
            let mut depth = 1usize;
            while i < n && depth > 0 {
                if ch[i] == '\n' {
                    line += 1;
                    i += 1;
                    continue;
                }
                if ch[i] == '/' && i + 1 < n && ch[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                    continue;
                }
                if ch[i] == '*' && i + 1 < n && ch[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                    continue;
                }
                i += 1;
            }
            prev_ident = false;
            continue;
        }
        if !prev_ident && c == 'b' && i + 1 < n && ch[i + 1] == '"' {
            let (ni, nl) = skip_plain(&ch, i + 1, line);
            i = ni;
            line = nl;
            prev_ident = false;
            continue;
        }
        if !prev_ident && (c == 'r' || (c == 'b' && i + 1 < n && ch[i + 1] == 'r')) {
            let start = if c == 'r' { i + 1 } else { i + 2 };
            if let Some((ni, nl)) = skip_raw(&ch, start, line) {
                i = ni;
                line = nl;
                prev_ident = false;
                continue;
            }
        }
        if c == '"' {
            let (ni, nl) = skip_plain(&ch, i, line);
            i = ni;
            line = nl;
            prev_ident = false;
            continue;
        }
        if c == '\'' {
            let (ni, nl) = skip_quote_atom(&ch, i, line);
            i = ni;
            line = nl;
            prev_ident = false;
            continue;
        }
        prev_ident = is_ident_char(c);
        i += 1;
    }
    out
}
