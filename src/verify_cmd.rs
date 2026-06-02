// AlphaWord Blocks — verifier command entry point
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

use crate::types::*;
use crate::util::{self, regularize_name, DEFAULT_TICKER_INTERVAL};
use crate::verify::{build_index_from_masks, verify_all};
use anyhow::{bail, Context, Result};
use std::fs::{self, File};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

// CHANGED: ensure we can reach runtime alphabet/tokenizer
use crate::alphabet;

#[derive(Debug, Default)]
struct VerifyConf {
    cubes_path: Option<PathBuf>,
    word_paths: Vec<PathBuf>,
    sentence_paths: Vec<PathBuf>,
}

fn parse_conf_file<P: AsRef<Path>>(p: P) -> Result<VerifyConf> {
    let p = p.as_ref();
    let f = File::open(p).with_context(|| format!("opening conf {:?}", p))?;
    let rdr = BufReader::new(f);
    let mut conf = VerifyConf::default();

    for (lineno, line) in rdr.lines().enumerate() {
        let mut l = line?;
        let original = l.clone();
        if let Some(i) = l.find('#') { l.truncate(i); }
        let l = l.trim();
        if l.is_empty() { continue; }

        let parts: Vec<&str> = if let Some(i) = l.find('=') {
            vec![&l[..i], &l[i+1..]]
        } else if let Some(i) = l.find(':') {
            vec![&l[..i], &l[i+1..]]
        } else {
            let mut it = l.splitn(2, char::is_whitespace);
            if let (Some(a), Some(b)) = (it.next(), it.next()) {
                vec![a, b]
            } else {
                bail!("{}:{}: expected 'key=value' (got {:?})", p.display(), lineno+1, original);
            }
        };

        let key = parts[0].trim().to_ascii_lowercase();
        let val = parts[1].trim();
        if val.is_empty() { continue; }

        match key.as_str() {
            "cubes" | "cubes_file" | "cubes.txt" => {
                conf.cubes_path = Some(PathBuf::from(val));
            }
            "word" | "words" | "word_file" => {
                for v in val.split(',') {
                    let v = v.trim();
                    if !v.is_empty() { conf.word_paths.push(PathBuf::from(v)); }
                }
            }
            "sentence" | "sentences" | "sentence_file" => {
                for v in val.split(',') {
                    let v = v.trim();
                    if !v.is_empty() { conf.sentence_paths.push(PathBuf::from(v)); }
                }
            }
            other => eprintln!("[conf] WARN: unknown key {:?} at {}:{}", other, p.display(), lineno+1),
        }
    }

    if conf.cubes_path.is_none() {
        bail!("config missing 'cubes=' line");
    }
    Ok(conf)
}

/* ========================= Cube parsers ========================= */

// CHANGED: masks are now Vec<u64> sized to runtime alphabet
type Masks = Vec<u64>;

/// Utility: ensure alphabet is already built by the caller (main.rs does this),
/// and return alphabet size.
#[inline]
fn ensure_alphabet_ready() -> Result<usize> {
    if !alphabet::is_built() {
        bail!("runtime alphabet is not built yet — build it in main before running verify.");
    }
    Ok(alphabet::n_letters())
}

/// Convert a sequence of *face tokens* into compiled chars, enforcing that
/// each face maps to **exactly one** compiled char (so one cube face = one letter).
fn compile_face_token(token: &str) -> Result<char> {
    let compiled = alphabet::tokenize_to_compiled(token);
    let mut it = compiled.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => Ok(c),
        (Some(_first), Some(_more)) => {
            bail!("cube face token {:?} expands to multiple letters after tokenization; each face must be a single token", token)
        }
        (None, _) => {
            bail!("cube face token {:?} does not map to any alphabet symbol after tokenization", token)
        }
    }
}

/// Parse a Java-style cubes file like:
///   const CUBES = [['i','j','o','t','u','z'], ['d','g','i','t','u','w'], ...];
/// or allowing string tokens (e.g., "ch") inside the inner arrays.
/// Returns letter→mask (bit j set if cube j contains that compiled letter).
fn parse_cubes_jsconst<P: AsRef<Path>>(p: P) -> anyhow::Result<Masks> {
    use anyhow::bail;

    let n_letters = ensure_alphabet_ready()?;
    let s = fs::read_to_string(&p)
        .with_context(|| format!("reading {:?}", p.as_ref()))?;

    // Simple hand-rolled parser for: [[ 'x', "y", ... ], [ ... ], ... ]
    let mut masks = vec![0u64; n_letters];

    let mut depth = 0usize;
    let mut i = 0usize;
    let bytes = s.as_bytes();

    // Find the first '[' to start the outer array
    while i < bytes.len() && s.as_bytes()[i] != b'[' { i += 1; }
    if i == bytes.len() { bail!("no '[' found in {:?}", p.as_ref()); }

    // State for inner arrays
    let mut cube_index = 0usize;
    let mut in_string = false;
    let mut quote_char = '\0';
    let mut token = String::new();
    let mut tokens_for_cube: Vec<String> = Vec::new();

    while i < bytes.len() {
        let ch = s[i..].chars().next().unwrap();
        let ch_len = ch.len_utf8();

        if in_string {
            if ch == quote_char {
                // End of token
                tokens_for_cube.push(token.clone());
                token.clear();
                in_string = false;
                i += ch_len;
                continue;
            } else if ch == '\\' {
                // rudimentary escape support: skip backslash, take next char as-is if present
                i += ch_len;
                if i < bytes.len() {
                    let ch2 = s[i..].chars().next().unwrap();
                    token.push(ch2);
                    i += ch2.len_utf8();
                }
                continue;
            } else {
                token.push(ch);
                i += ch_len;
                continue;
            }
        }

        match ch {
            '\'' | '\"' if depth >= 2 => {
                in_string = true;
                quote_char = ch;
                token.clear();
            }
            '[' => {
                depth += 1;
                if depth == 3 {
                    // unexpected: nested deeper than inner arrays
                    bail!("unexpected nested '[' depth in {:?}", p.as_ref());
                }
            }
            ']' => {
                if depth == 2 {
                    // end of one cube
                    // compile its tokens into compiled chars and set masks
                    for t in &tokens_for_cube {
                        let c = compile_face_token(t)?;
                        if let Some(li) = crate::alphabet::char_index(c) {
                            masks[li] |= 1u64 << cube_index;
                        } else {
                            eprintln!("[cubes] WARN: face token {:?} (compiled {:?}) not in ALPHABET; ignored (cube {}).", t, c, cube_index);
                        }
                    }
                    tokens_for_cube.clear();
                    cube_index += 1;
                }
                if depth == 0 { break; }
                depth = depth.saturating_sub(1);
            }
            _ => { /* commas/whitespace/idents ignored */ }
        }
        i += ch_len;
    }

    if cube_index == 0 {
        bail!("no inner arrays found; expected Java-style const CUBES = [[...], ...];");
    }
    if cube_index < N_CUBES {
        eprintln!("[cubes] WARN: only {} cubes found (expected {}); continuing with fewer.", cube_index, N_CUBES);
    }
    if cube_index > N_CUBES {
        eprintln!("[cubes] INFO: {} cubes found; using first {} (N_CUBES).", cube_index, N_CUBES);
    }

    Ok(masks)
}

/// Parse a pythonic cubes file like:
///   cubes = [set("ijotuz"), set('dgituw'), ...]
/// Returns letter→mask (bit j set if cube j contains that letter).
fn parse_cubes_pythonic<P: AsRef<Path>>(p: P) -> anyhow::Result<Masks> {
    use anyhow::{bail, Context};
    use regex::Regex;
    use std::fs;

    let n_letters = ensure_alphabet_ready()?;
    let s = fs::read_to_string(&p).with_context(|| format!("reading {:?}", p.as_ref()))?;

    // No backrefs: capture both quotes and compare them in code.
    // Matches set("abc") and set('abc'); if quotes mismatch, we skip with a warning.
    let re = Regex::new(r#"set\(\s*(['"])([A-Za-z]+)(['"])\s*\)"#).unwrap();

    let mut sets: Vec<String> = Vec::new();
    for cap in re.captures_iter(&s) {
        let q1 = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let q2 = cap.get(3).map(|m| m.as_str()).unwrap_or("");
        if q1 != q2 {
            eprintln!("[cubes] WARN: mismatched quotes in {} — skipping: {}",
                      p.as_ref().display(), &cap[0]);
            continue;
        }
        sets.push(cap.get(2).unwrap().as_str().to_string());
    }

    if sets.is_empty() {
        bail!(
            "no sets found in {:?}; expected pythonic like: cubes = [set(\"abc\"), ...]",
            p.as_ref()
        );
    }
    if sets.len() < N_CUBES {
        eprintln!("[cubes] WARN: only {} sets found (expected {}); continuing with fewer.",
                  sets.len(), N_CUBES);
    }
    if sets.len() > N_CUBES {
        eprintln!("[cubes] INFO: {} sets found; using first {} (N_CUBES).",
                  sets.len(), N_CUBES);
    }

    let mut masks = vec![0u64; n_letters];
    for (j, letters) in sets.into_iter().take(N_CUBES).enumerate() {
        // CHANGED: run max-munch tokenizer on the whole face string,
        // so multi-letter tokens like "ch" or "que" are handled.
        let compiled: Vec<char> = alphabet::compile_face_string(&letters);
        if compiled.is_empty() {
            eprintln!("[cubes] WARN: cube {} has empty/invalid face string {:?}", j, letters);
        }
        for c in compiled {
            if let Some(i) = crate::alphabet::char_index(c) {
                masks[i] |= 1u64 << j;
            } else {
                eprintln!("[cubes] WARN: letter {:?} not in ALPHABET; ignored (cube {}).", c, j);
            }
        }
    }
    Ok(masks)
}

/// Try Java-style first; if it fails, fall back to pythonic.
fn parse_cubes_auto<P: AsRef<Path>>(p: P) -> anyhow::Result<Masks> {
    match parse_cubes_jsconst(&p) {
        Ok(m) => Ok(m),
        Err(e_js) => {
            // Try pythonic
            match parse_cubes_pythonic(&p) {
                Ok(m) => Ok(m),
                Err(e_py) => {
                    // Report both errors for easier debugging.
                    Err(anyhow::anyhow!("failed to parse cubes as Java-style ({:?}) or pythonic ({:?})", e_js, e_py))
                }
            }
        }
    }
}

/* ========================= Loading words/sentences ========================= */

fn load_words_one_file(p: &Path) -> Vec<String> {
    println!("[load] words: {}", p.display());
    let words = util::read_words(p);
    println!("[load] words: {} (+{})", p.display(), words.len());
    // Dedup within file (stable)
    let mut seen = std::collections::HashSet::<String>::new();
    let mut uniq = Vec::new();
    for w in words {
        if seen.insert(w.clone()) {
            uniq.push(w);
        }
    }
    uniq
}

fn load_sentence_lines_one_file(p: &Path) -> Vec<String> {
    println!("[load] sentences: {}", p.display());
    let file = match File::open(p) {
        Ok(f) => f,
        Err(e) => { eprintln!("[load] ERROR opening {}: {}", p.display(), e); return Vec::new(); }
    };
    let rdr = BufReader::new(file);
    let mut out = Vec::<String>::new();
    let mut n = 0usize;
    for line in rdr.lines() {
        if let Ok(l) = line {
            n += 1;
            let s = regularize_name(l.trim());
            if !s.is_empty() {
                out.push(s);
            }
        }
    }
    println!("[load] sentences: {} (+{}, normalized)", p.display(), n);
    // Dedup within file (stable)
    let mut seen = std::collections::HashSet::<String>::new();
    let mut uniq = Vec::new();
    for w in out {
        if seen.insert(w.clone()) {
            uniq.push(w);
        }
    }
    uniq
}

/* ========================= Orchestrator ========================= */

pub fn run_from_conf<P: AsRef<Path>>(conf_path: P) -> Result<()> {
    let conf_path = conf_path.as_ref();
    let conf = parse_conf_file(conf_path)?;
    let cubes_path = conf.cubes_path.as_ref().unwrap();

    // CHANGED: ensure runtime alphabet is built (by main) and get size
    ensure_alphabet_ready()?;

    // CHANGED: Build index from Java-style (or pythonic fallback) cubes
    let masks = parse_cubes_auto(cubes_path)?;
    let idx = build_index_from_masks(&masks);

    // Output paths
    let dir = conf_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = conf_path.file_stem().unwrap_or_default().to_string_lossy();
    let out_all = dir.join(format!("{}.all_words.txt", stem));
    let out_bad = dir.join(format!("{}.unspellable.txt", stem));

    // Aggregate for all_words.txt (global)
    let mut all_tokens: Vec<String> = Vec::new();

    // Prepare grouped-unspellables buffer
    let mut grouped = String::new();
    let mut total_unspellable = 0usize;
    let mut total_checked = 0usize;

    // Helper to run verify for one source
    let mut run_one = |label: &str, path: &Path, tokens: Vec<String>| -> Result<()> {
        if tokens.is_empty() {
            println!("[verify] {} — {} (0 tokens, skipping).", label, path.display());
            return Ok(());
        }
        println!(
            "[verify] {} — {} ({} tokens; ticker ~{:.1}s).",
            label, path.display(), tokens.len(), DEFAULT_TICKER_INTERVAL
        );
        let (_ok, unspellable) = verify_all(
            &tokens,
            &idx,
            10_000,
            false,
            DEFAULT_TICKER_INTERVAL,
        );
        total_checked += tokens.len();

        // Append to global token pool
        all_tokens.extend(tokens.into_iter());

        // Grouped output
        let base = path.file_name().unwrap_or_else(|| path.as_os_str()).to_string_lossy();
        writeln!(&mut grouped, "===={}====", base).unwrap();
        if unspellable.is_empty() {
            writeln!(&mut grouped, "(none)").unwrap();
            println!("===={}====", base);
            println!("(none)");
        } else {
            total_unspellable += unspellable.len();
            for w in &unspellable {
                writeln!(&mut grouped, "{}", w).unwrap();
                // Also print to stdout as requested
                println!("{}", w);
            }
        }
        // Blank separator
        writeln!(&mut grouped).unwrap();
        println!();
        Ok(())
    };

    // Words per file
    for p in &conf.word_paths {
        let toks = load_words_one_file(p);
        run_one("words", p, toks)?;
    }

    // Sentences per file (each line becomes one normalized token; no splitting)
    for p in &conf.sentence_paths {
        let toks = load_sentence_lines_one_file(p);
        run_one("sentences", p, toks)?;
    }

    // Write all_words.txt (dedup + sort for reproducibility)
    if all_tokens.is_empty() {
        bail!("No tokens to verify — word/sentence lists are empty.");
    }
    let mut set = std::collections::BTreeSet::<String>::new();
    for w in all_tokens { set.insert(w); }
    let mut all_sorted: Vec<String> = set.into_iter().collect();
    all_sorted.sort();
    fs::write(&out_all, all_sorted.join("\n") + "\n")
        .with_context(|| format!("writing {:?}", out_all))?;
    println!("[out] wrote {}", out_all.display());

    // Write grouped unspellables
    fs::write(&out_bad, grouped)
        .with_context(|| format!("writing {:?}", out_bad))?;
    println!("[out] wrote {}", out_bad.display());
    println!("[verify] DONE — {} / {} unspellable.", total_unspellable, total_checked);
    Ok(())
}
