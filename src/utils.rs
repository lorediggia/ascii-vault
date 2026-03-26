use crate::app::AsciiItem;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// ── Cursor ──────────────────────────────────────────────────────────────────
pub fn pos_to_line_col(buf: &str, pos: usize) -> (usize, usize) {
    let pos = (0..=pos.min(buf.len()))
        .rev()
        .find(|&p| buf.is_char_boundary(p))
        .unwrap_or(0);
    let before = &buf[..pos];
    let line = before.matches('\n').count();
    let col = match before.rfind('\n') {
        Some(nl) => before[nl + 1..].chars().count(),
        None => before.chars().count(),
    };
    (line, col)
}

pub fn cursor_left(buf: &str, pos: usize) -> usize {
    if pos == 0 {
        return 0;
    }
    let mut p = pos - 1;
    while !buf.is_char_boundary(p) {
        p -= 1;
    }
    p
}

pub fn cursor_right(buf: &str, pos: usize) -> usize {
    if pos >= buf.len() {
        return buf.len();
    }
    let mut p = pos + 1;
    while p <= buf.len() && !buf.is_char_boundary(p) {
        p += 1;
    }
    p.min(buf.len())
}

pub fn line_start(buf: &str, pos: usize) -> usize {
    let before = &buf[..pos.min(buf.len())];
    before.rfind('\n').map(|i| i + 1).unwrap_or(0)
}

pub fn line_end(buf: &str, pos: usize) -> usize {
    let pos = pos.min(buf.len());
    match buf[pos..].find('\n') {
        Some(i) => pos + i,
        None => buf.len(),
    }
}

pub fn cursor_up(buf: &str, pos: usize) -> usize {
    let (line, col) = pos_to_line_col(buf, pos);
    if line == 0 {
        return 0;
    }
    let lines: Vec<&str> = buf.split('\n').collect();
    let prev = lines[line - 1];
    let target_col = col.min(prev.chars().count());
    let mut offset = 0usize;
    for (i, l) in lines.iter().enumerate() {
        if i == line - 1 {
            return offset
                + l.char_indices()
                    .nth(target_col)
                    .map(|(b, _)| b)
                    .unwrap_or(l.len());
        }
        offset += l.len() + 1;
    }
    0
}

pub fn cursor_down(buf: &str, pos: usize) -> usize {
    let (line, col) = pos_to_line_col(buf, pos);
    let lines: Vec<&str> = buf.split('\n').collect();
    if line + 1 >= lines.len() {
        return buf.len();
    }
    let next = lines[line + 1];
    let target_col = col.min(next.chars().count());
    let mut offset = 0usize;
    for (i, l) in lines.iter().enumerate() {
        if i == line + 1 {
            return offset
                + l.char_indices()
                    .nth(target_col)
                    .map(|(b, _)| b)
                    .unwrap_or(l.len());
        }
        offset += l.len() + 1;
    }
    buf.len()
}

pub fn insert_char(buf: &mut String, pos: &mut usize, c: char) {
    buf.insert(*pos, c);
    *pos += c.len_utf8();
}

pub fn backspace(buf: &mut String, pos: &mut usize) {
    if *pos == 0 {
        return;
    }
    let prev = cursor_left(buf, *pos);
    buf.remove(prev);
    *pos = prev;
}

pub fn delete_char(buf: &mut String, pos: usize) {
    if pos < buf.len() && buf.is_char_boundary(pos) {
        buf.remove(pos);
    }
}

// ── Persistence & Notes ────────────────────────────────────────────────────
pub fn load_data(path: &str) -> Vec<AsciiItem> {
    if !std::path::Path::new(path).exists() {
        return vec![];
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|d| serde_json::from_str(&d).ok())
        .unwrap_or_default()
}

pub fn save_data(items: &[AsciiItem], path: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(j) = serde_json::to_string_pretty(items) {
        let tmp_path = format!("{}.tmp", path);
        if fs::write(&tmp_path, &j).is_ok() {
            let _ = fs::rename(&tmp_path, path); 
        }
    }
}

pub fn copy_to_clipboard(text: &str) {
    if let Ok(mut child) = Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
    }
    if let Ok(mut child) = Command::new("cliphist")
        .arg("store")
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
    }
}

pub fn get_clipboard() -> String {
    Command::new("wl-paste")
        .arg("--no-newline")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

pub fn is_safe_path(path_str: &str) -> bool {
    let path = Path::new(path_str);

    if !path.starts_with("/home/") {
        return false;
    }

    if path_str.contains("..") {
        return false;
    }

    true
}
