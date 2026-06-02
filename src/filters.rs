// AlphaWord Blocks — corpus word filtering and blocklist handling
// Copyright (C) 2025- Svetlin Tassev
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// Normalize blocked entries the same way corpus words are normalized.
use crate::util::regularize_name;

/// Load blocked words (one per line), ignoring blanks and lines starting with '#'.
/// Entries are normalized with `regularize_name` so they match post-regularization words.
pub fn load_blocked_words(path: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let p = Path::new(path);
    if !p.exists() {
        eprintln!("[blocked] INFO: no blocked list at {}; continuing with empty set.", path);
        return out;
    }
    match File::open(p) {
        Ok(f) => {
            for line in BufReader::new(f).lines() {
                if let Ok(s) = line {
                    let t = s.trim();
                    if t.is_empty() || t.starts_with('#') { continue; }

                    let norm = regularize_name(t);
                    if norm.is_empty() { continue; }
                    out.insert(norm);
                }
            }
        }
        Err(e) => eprintln!("[blocked] ERROR: cannot read {}: {}", path, e),
    }
    out
}

/// Remove words longer than `max_len` and any found in `data/blocked.txt`.
/// Logs **which filter** rejected a word:
///   - [filter:max_length]
///   - [filter:blocked_list]
/// Length check uses Unicode scalar count (chars) instead of bytes, which
/// matches tokenized words because each compiled token is one `char`.
pub fn remove_long_words(words: Vec<String>, max_len: usize) -> Vec<String> {
    let blocked = load_blocked_words("data/blocked.txt");
    let mut out = Vec::with_capacity(words.len());
    for w in words {
        let len_chars = w.chars().count();
        if len_chars > max_len {
            eprintln!(
                "[filter:max_length] drop {:?} — len {} > max_len {}",
                w, len_chars, max_len
            );
            continue;
        }
        if blocked.contains(&w) {
            eprintln!(
                "[filter:blocked_list] drop {:?} — present in data/blocked.txt (normalized)",
                w
            );
            continue;
        }
        out.push(w);
    }
    out
}

/// Remove Roman numerals (len ≤ 10) using a permissive regex.
/// Logs **which filter** rejected a word:
///   - [filter:roman_numeral]
pub fn remove_roman_numerals(words: Vec<String>) -> Vec<String> {
    let re = regex::Regex::new(
        r"^m{0,4}(cm|cd|d?c{0,4})(xc|xl|l?x{0,4})(ix|iv|v?i{0,4})$"
    ).unwrap();
    let mut out = Vec::with_capacity(words.len());
    for w in words {
        // Keep behavior: only drop if already short (≤10) and matches the pattern.
        if w.len() <= 10 && re.is_match(&w) {
            eprintln!(
                "[filter:roman_numeral] drop {:?} — matches Roman numeral pattern (len ≤ 10)",
                w
            );
            continue;
        }
        out.push(w);
    }
    out
}
