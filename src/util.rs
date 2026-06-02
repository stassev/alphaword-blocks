// AlphaWord Blocks — small shared utilities (corpus reading, helpers)
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

use unicode_segmentation::UnicodeSegmentation;
use unicode_normalization::UnicodeNormalization;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Unified, public normalization entry point used by other modules.
/// NFC (+ optional Hangul Jamo) → runtime TOKEN_REWRITES → max-munch → compiled chars.
#[inline]
pub fn regularize_name(s: &str) -> String {
    crate::alphabet::tokenize_to_compiled(s)
}

/// For warnings only: check whether a single scalar is either in the alphabet already,
/// or can be rewritten+tokenized into something non-empty.
#[inline]
fn in_alphabet_or_rewritable(ch: char) -> bool {
    let s = ch.to_string();
    let compiled = crate::alphabet::tokenize_to_compiled(&s);
    !compiled.is_empty()
}

/* -------------------- file reading & normalization -------------------- */

/// Compare a raw input string with its normalized+rewritten+tokenized→expanded form,
/// using NFC on both sides, for INFO logging.
#[inline]
fn eq_unicode_compare_raw_vs_expanded(raw: &str, compiled: &str) -> bool {
    let expanded = crate::alphabet::expand_compiled_to_tokens(compiled);
    let raw_nfc: String = raw.nfc().collect();
    let exp_nfc: String = expanded.nfc().collect();
    raw_nfc == exp_nfc
}

/// Reader (with guardrails) that’s ALPHABET-aware:
/// - trim
/// - replace separators with '\n' (skip any separator that is in ALPHABET compiled chars)
/// - split on '\n'
/// - warn on scalars not in ALPHABET and not rewritable
/// - normalize via the unified pipeline (NFC → rewrites → max-munch) to compiled chars
/// - drop segments that normalize to empty (INFO)
/// - INFO-log if normalized tokenized form differs (NFC compare vs token expansion)
pub fn read_words<P: AsRef<Path>>(path: P) -> Vec<String> {
    let p = path.as_ref();
    if !p.exists() {
        eprintln!("[read_words] ERROR: file not found: {:?}", p);
        return Vec::new();
    }
    let f = match File::open(p) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[read_words] ERROR: could not open {:?}: {}", p, e);
            return Vec::new();
        }
    };
    let rdr = BufReader::new(f);

    let mut words: Vec<String> = Vec::new();

    for line in rdr.lines() {
        if let Ok(mut l) = line {
            l = l.trim().to_string();
            if l.is_empty() {
                continue;
            }
            // Build separators and drop any that clash with compiled alphabet chars
            let mut seps = vec!["!","?",",",".","_", "/", ";", "+", "|", ":", "\\", " ", "-", "–", "—", "‒", "⸺", "⸻"];
            seps.sort();
            seps.dedup();

            for sep in seps {
                // If any separator character is actually one of the compiled alphabet chars, skip it.
                let mut clash = false;
                for sch in sep.chars() {
                    if crate::alphabet::alphabet_chars().iter().any(|&a| a == sch) {
                        clash = true; break;
                    }
                }
                if !clash {
                    l = l.replace(sep, "\n");
                }
            }

            for sw in l.split('\n') {
                if sw.is_empty() {
                    continue;
                }

                // Warn about characters that cannot be handled by the pipeline at all.
                let mut bad: Vec<char> = Vec::new();
                for ch in sw.chars() {
                    if ch.is_whitespace() { continue; }
                    if !in_alphabet_or_rewritable(ch) {
                        bad.push(ch);
                    }
                }
                if !bad.is_empty() {
                    eprintln!(
                        "[read_words] WARN: {:?} contains non-ALPHABET chars {:?} (file: {:?})",
                        sw, bad, p
                    );
                }

                // Unified pipeline: normalize + rewrites + max-munch → compiled chars
                let compiled = crate::alphabet::tokenize_to_compiled(sw);
                if compiled.is_empty() {
                    eprintln!(
                        "[read_words] INFO: dropped {:?} after normalization (no ALPHABET content)",
                        sw
                    );
                    continue;
                }

                // INFO if expanded token string differs from raw (NFC compare)
                if !eq_unicode_compare_raw_vs_expanded(sw, &compiled) {
                    let expanded = crate::alphabet::expand_compiled_to_tokens(&compiled);
                    eprintln!("[read_words] INFO: normalized {:?} -> {:?}", sw, expanded);
                }

                words.push(compiled);
            }
        }
    }
    if words.is_empty() {
        eprintln!("[read_words] ERROR: {:?} yielded 0 words after normalization.", p);
    }
    words
}

/* --------------------  Noise heuristics (unchanged)  -------------------- */







///////////////////////////
/// Consecutive repeats of the **same grapheme** of length ≥ min_len.
#[inline]
fn has_grapheme_run(word: &str, min_len: usize) -> bool {
    let mut prev: Option<&str> = None;
    let mut run = 0usize;
    for g in UnicodeSegmentation::graphemes(word, true) {
        if Some(g) == prev {
            run += 1;
            if run >= min_len { return true; }
        } else {
            run = 1;
            prev = Some(g);
        }
    }
    false
}

/// Check for a k-grapheme block repeated `reps` times consecutively anywhere.
#[inline]
fn has_repeated_block_g(word: &str, k: usize, reps: usize) -> bool {
    let g: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    let n = g.len();
    if k == 0 || reps < 2 || n < k * reps { return false; }
    let limit = n.saturating_sub(k * reps);     // <-- fixed
    for i in 0..=limit {
        let block = &g[i..i + k];
        let mut ok = true;
        for r in 1..reps {
            let start = i + r * k;
            // start + k <= n is guaranteed by the limit above
            if &g[start..start + k] != block {
                ok = false;
                break;
            }
        }
        if ok { return true; }
    }
    false
}

/// Run of a specific **grapheme/token** (can be multi-cluster), repeated ≥ min_len.
#[inline]
fn has_run_of_grapheme(word: &str, token: &str, min_len: usize) -> bool {
    let tg: Vec<&str> = UnicodeSegmentation::graphemes(token, true).collect();
    if tg.is_empty() { return false; }

    let g: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    let mut i = 0usize;
    let mut run = 0usize;
    while i + tg.len() <= g.len() {
        if &g[i..i + tg.len()] == tg.as_slice() {
            run += 1;
            if run >= min_len { return true; }
            i += tg.len();
        } else {
            i += 1;
            run = 0;
        }
    }
    false
}

/// ^(token in {tokens}){min_reps,} — anchored at start, **grapheme-aware**.
/// Tokens may be of mixed grapheme lengths; we match greedily in input order.
#[inline]
fn starts_with_token_run(word: &str, tokens: &[&str], min_reps: usize) -> bool {
    let g: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    if g.is_empty() { return false; }
    let toks: Vec<Vec<&str>> = tokens.iter()
        .map(|t| UnicodeSegmentation::graphemes(*t, true).collect())
        .collect();

    let mut i = 0usize;
    let mut reps = 0usize;
    loop {
        let mut matched = 0usize;
        for t in &toks {
            let m = t.len();
            if m > 0 && i + m <= g.len() && &g[i..i + m] == &t[..] {
                matched = m;
                break;
            }
        }
        if matched == 0 { break; }
        reps += 1;
        i += matched;
    }
    reps >= min_reps
}

/// Chunk repeated at least 3× to make the whole word, chunk length ≥ 2 **graphemes**.
#[inline]
fn is_chunk_tripled(word: &str) -> bool {
    let g: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    let n = g.len();
    if n < 6 { return false; } // shortest is 2 * 3
    for period in 2..=n / 3 {
        if n % period != 0 { continue; }
        let reps = n / period;
        if reps < 3 { continue; }
        let chunk = &g[..period];
        let mut ok = true;
        for r in 1..reps {
            let start = r * period;
            if &g[start..start + period] != chunk {
                ok = false;
                break;
            }
        }
        if ok { return true; }
    }
    false
}

/// Vowel run of length ≥ min_len (Latin ASCII + Cyrillic + Greek; char-based).
#[inline]
fn has_vowel_run_chars(word: &str, min_len: usize) -> bool {
    let vowels_lat = ['a','e','i','o','u','y','A','E','I','O','U','Y'];
    let vowels_cyr = ['а','е','ё','и','о','у','ы','э','ю','я','А','Е','Ё','И','О','У','Ы','Э','Ю','Я'];
    let vowels_grk = ['α','ε','η','ι','ο','υ','ω','Α','Ε','Η','Ι','Ο','Υ','Ω'];

    let mut run = 0usize;
    for ch in word.chars() {
        if vowels_lat.contains(&ch) || vowels_cyr.contains(&ch) || vowels_grk.contains(&ch) {
            run += 1;
            if run >= min_len { return true; }
        } else { run = 0; }
    }
    false
}
/// Heuristic noise detector — now **grapheme-aware** where it matters.
pub fn looks_noisy(w: &str) -> bool {
    if w.is_empty() { return false; }

    has_grapheme_run(w, 5)
        || has_repeated_block_g(w, 2, 4)
        || has_repeated_block_g(w, 3, 3)
        || has_run_of_grapheme(w, "x", 3) // (keeps your original intent; can add "X" if you want)
        || starts_with_token_run(w, &["ch","co","oh","nh","ho"], 3)
        || has_vowel_run_chars(w, 5)
        || is_chunk_tripled(w)
}


/// Heuristic noise detector — kept *exactly* as your original byte-based logic.
//pub fn looks_noisy(w: &str) -> bool {
//    if w.is_empty() { return false; }
//    let b = w.as_bytes();
//
//    has_run(b, 5)
//    || has_repeated_block(b, 2, 4)
//    || has_repeated_block(b, 3, 3)
//    || has_run_of_char(b, b'x', 3)
//    || starts_with_token_run(w, &["ch","co","oh","nh","ho"], 3)
//    || has_vowel_run(b, 5)
//    || is_chunk_tripled(w)
//}

pub fn find_noisy_words(words: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for w in words {
        if !w.is_empty() && looks_noisy(w) {
            out.push(w.clone());
        }
    }
    out.sort();
    out.dedup();
    out
}

// --- small timing helpers ---

/// Seconds since UNIX epoch as f64.
pub fn now_sec() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// How often to print progress tickers.
pub const DEFAULT_TICKER_INTERVAL: f64 = 0.5; // seconds
