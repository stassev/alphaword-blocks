// AlphaWord Blocks — runtime-compiled token alphabet and tokenization
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

//! Runtime-compiled token alphabet with greedy (max-munch) tokenization.
//!
//! - Tokens are configured at runtime (see glyph_config).
//! - We normalize tokens & text using **NFC only** plus optional Hangul decomposition
//!     (`types::DECOMPOSE_HANGUL`). We **do not** case-fold or strip diacritics here;
//!     those behaviors are expressed via TOKEN REWRITES computed in glyph_config.rs.
//! - Each token compiles to a unique single `char`:
//!     • single-scalar tokens reuse their scalar (e.g., 'é')
//!     • multi-scalar tokens (or collisions) get PUA chars (BMP PUA, then Plane 15/16).

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use unicode_normalization::UnicodeNormalization;

#[inline]
fn decompose_hangul() -> bool { crate::types::DECOMPOSE_HANGUL }

/* ----------------------------- Hangul decomposition ------------------------------ */

#[inline]
fn decompose_hangul_syllables(s: &str) -> String {
    if !decompose_hangul() { return s.to_string(); }
    const S_BASE: u32 = 0xAC00;
    const L_BASE: u32 = 0x1100;
    const V_BASE: u32 = 0x1161;
    const T_BASE: u32 = 0x11A7;
    const N_L: u32 = 19;
    const N_V: u32 = 21;
    const N_T: u32 = 28;
    const S_END: u32 = S_BASE + N_L * N_V * N_T - 1;

    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        let c = ch as u32;
        if (S_BASE..=S_END).contains(&c) {
            let s_index = c - S_BASE;
            let l_index = s_index / (N_V * N_T);
            let v_index = (s_index % (N_V * N_T)) / N_T;
            let t_index = s_index % N_T;

            if let Some(lc) = char::from_u32(L_BASE + l_index) { out.push(lc); }
            if let Some(vc) = char::from_u32(V_BASE + v_index) { out.push(vc); }
            if t_index != 0 {
                if let Some(tc) = char::from_u32(T_BASE + t_index) { out.push(tc); }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/* ------------------------------ Normalization policy ------------------------------ */

pub fn normalize_for_tokenization(s: &str) -> String {
    let nfc: String = s.nfc().collect::<String>();
    decompose_hangul_syllables(&nfc)
}

/* ------------------------------ Runtime token rewrites ------------------------------ */

struct RewriteRule { from: String, to: String }
struct RewriteTable { rules: Vec<RewriteRule> }

static REWRITES: OnceLock<RewriteTable> = OnceLock::new();

/// Check that `s` is fully coverable by the current tokens (without applying rewrites).
fn rhs_fully_tokenizable(s: &str, a: &Alphabet) -> bool {
    if s.is_empty() { return false; }
    let mut i = 0usize;
    while i < s.len() {
        let slice = &s[i..];
        let mut matched = false;
        for t in &a.sorted {
            if slice.starts_with(&t.token) {
                i += t.token.len();
                matched = true;
                break;
            }
        }
        if !matched {
            return false;
        }
    }
    true
}

/// Install runtime rewrites (normalized + filtered) coming **only** from glyph_config.rs.
/// Behavior:
/// - Drop rules whose LHS is an actual token (distinct glyph remains distinct).
/// - Drop rules whose RHS is not fully tokenizable by the current alphabet
///   (supports multi-token RHS like "AE").
/// - Sort longest-first for greedy max-munch stability.
pub fn install_rewrites_filtered(raw_pairs: &[(String, String)]) -> anyhow::Result<()> {
    let a = alphabet(); // must be built already

    // Which tokens exist for this run?
    let mut token_set = HashSet::<&str>::new();
    for t in &a.tokens {
        token_set.insert(t.as_str());
    }

    let mut rules: Vec<RewriteRule> = Vec::new();
    for (lhs, rhs) in raw_pairs {
        let from = normalize_for_tokenization(lhs);
        let to   = normalize_for_tokenization(rhs);
        if from.is_empty() || to.is_empty() || from == to { continue; }

        if token_set.contains(from.as_str()) {
            // Author kept the LHS as a real glyph; don’t auto-rewrite it.
            continue;
        }
        if !rhs_fully_tokenizable(&to, a) {
            eprintln!("[rewrite] INFO: dropping rule {:?} → {:?} (RHS not tokenizable)", lhs, rhs);
            continue;
        }
        rules.push(RewriteRule { from, to });
    }

    rules.sort_by(|x, y| {
		let lx = x.from.chars().count();
		let ly = y.from.chars().count();
		ly.cmp(&lx).then_with(|| x.from.cmp(&y.from))
	});


    REWRITES.set(RewriteTable { rules }).map_err(|_| anyhow::anyhow!("Rewrites already installed"))
}

/* ======================== Compiled alphabet & tokenizer ======================== */

pub struct Alphabet {
    pub(crate) tokens: Vec<String>,
    chars: Vec<char>,
    
    ch_to_tok: HashMap<char, String>,
    ch_to_index: HashMap<char, usize>,
    pub(crate) sorted: Vec<SortedToken>,
}

pub struct SortedToken {
    pub token: String,
    pub compiled: char,
    pub order: usize,
}

static ALPHABET: OnceLock<Alphabet> = OnceLock::new();

pub fn build_alphabet(input_tokens: &[String]) -> anyhow::Result<()> {
    use anyhow::bail;

    if input_tokens.is_empty() { bail!("INPUT_ALPHABET is empty"); }

    // 1) Normalize tokens (NFC + optional Hangul decomp)
    let norm: Vec<String> = input_tokens.iter().map(|t| normalize_for_tokenization(t)).collect();

    // 2) Warn if authored token ≠ canonical; we will use canonical.
    for (i, (orig, can)) in input_tokens.iter().zip(norm.iter()).enumerate() {
        if orig != can {
            eprintln!(
                "[alphabet] WARN: token #{i} {:?} is not canonical under current policy; canonical = {:?}",
                orig, can
            );
        }
    }

    // 3) Uniqueness after normalization — fatal
    {
        let mut seen = HashSet::<&str>::new();
        for t in &norm {
            if !seen.insert(t.as_str()) {
                eprintln!("[alphabet] ERROR: duplicate token after normalization: {:?}", t);
                anyhow::bail!("duplicate token");
            }
        }
    }

    // Helper: does string equal a single scalar?
    let single_char = |s: &str| -> Option<char> {
        let mut it = s.chars();
        match (it.next(), it.next()) {
            (Some(c), None) => Some(c),
            _ => None,
        }
    };

    // 4) Compile tokens to unique chars: reuse single scalars, else PUA.
    let mut chars = vec!['\0'; norm.len()];
    let mut used = HashSet::<char>::new();

    // Reuse single scalars if unique.
    for (i, t) in norm.iter().enumerate() {
        if let Some(c) = single_char(t) {
            if used.insert(c) {
                chars[i] = c;
            }
        }
    }

    // PUA allocation across BMP (Plane 0) and Supplementary (Planes 15/16) ranges.
    const PUA_RANGES: &[(u32, u32)] = &[
        (0xE000, 0xF8FF),       // BMP Private Use Area
        (0xF0000, 0xFFFFD),     // Plane 15 Private Use
        (0x100000, 0x10FFFD),   // Plane 16 Private Use
    ];
    let mut range_ix = 0usize;
    let mut cur = PUA_RANGES[0].0;

    let mut next_pua = |used: &HashSet<char>| -> anyhow::Result<char> {
        loop {
            if range_ix >= PUA_RANGES.len() {
                anyhow::bail!("Ran out of Private Use Area code points while compiling alphabet");
            }
            let (_, hi) = PUA_RANGES[range_ix];
            if cur > hi {
                range_ix += 1;
                if range_ix < PUA_RANGES.len() {
                    cur = PUA_RANGES[range_ix].0;
                }
                continue;
            }
            if let Some(cand) = char::from_u32(cur) {
                cur = cur.saturating_add(1);
                if !used.contains(&cand) {
                    return Ok(cand);
                }
            } else {
                cur = cur.saturating_add(1);
            }
        }
    };

    // Fill remaining with PUA.
    for ch in &mut chars {
        if *ch != '\0' { continue; }
        let cand = next_pua(&used)?;
        *ch = cand;
        used.insert(cand);
    }

    // 5) Build maps and tokenizer table.
    let mut tok_to_ch = HashMap::new();
    let mut ch_to_tok = HashMap::new();
    let mut ch_to_index = HashMap::new();
    for (i, (&c, t)) in chars.iter().zip(norm.iter()).enumerate() {
        tok_to_ch.insert(t.clone(), c);
        ch_to_tok.insert(c, t.clone());
        ch_to_index.insert(c, i);
    }

    let mut sorted: Vec<SortedToken> = norm.iter().enumerate().map(|(i, t)| SortedToken {
        token: t.clone(),
        compiled: chars[i],
        order: i,
    }).collect();

    // Max-munch priority: longer token first; tie -> earlier list order.
    sorted.sort_by(|a, b| {
    let la = a.token.chars().count();
    let lb = b.token.chars().count();
    lb.cmp(&la).then(a.order.cmp(&b.order))
});


    let alpha = Alphabet { tokens: norm, chars, ch_to_tok, ch_to_index, sorted };
    ALPHABET.set(alpha).map_err(|_| anyhow::anyhow!("ALPHABET already built"))?;
    Ok(())
}

/* ======================== Accessors & helpers ============================ */

#[inline] pub fn is_built() -> bool { ALPHABET.get().is_some() }
pub fn alphabet() -> &'static Alphabet {
    ALPHABET.get().expect("ALPHABET not built — call build_alphabet() first")
}
#[inline] pub fn n_letters() -> usize { alphabet().chars.len() }
#[inline] pub fn alphabet_chars() -> &'static [char] { &alphabet().chars }
#[inline] pub fn char_index(c: char) -> Option<usize> { alphabet().ch_to_index.get(&c).copied() }

pub fn token_of_compiled(c: char) -> &'static str {
    alphabet().ch_to_tok.get(&c).map(|s| s.as_str()).unwrap_or("�")
}

pub fn expand_compiled_to_tokens(compiled: &str) -> String {
    let mut out = String::new();
    for c in compiled.chars() {
        out.push_str(token_of_compiled(c));
    }
    out
}

pub fn print_alphabet_summary() {
    let a = alphabet();
    println!("===== Compiled Alphabet =====");
    println!("Tokens: {}", a.tokens.len());
    for t in a.tokens.iter() { print!("{} ", t); }
    //if a.tokens.len() > 120 { print!("…"); }
    println!();
}

/* ======================== Tokenization ============================ */

#[inline]
fn apply_rewrites(s: &str) -> String {
    let tbl = REWRITES.get();
    if tbl.is_none() { return s.to_string(); }
    let tbl = tbl.unwrap();
    if tbl.rules.is_empty() { return s.to_string(); }

    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < s.len() {
        let slice = &s[i..];
        let mut matched = false;
        for r in &tbl.rules {
            if slice.starts_with(&r.from) {
                out.push_str(&r.to);
                i += r.from.len();
                matched = true;
                break;
            }
        }
        if !matched {
            // advance by one scalar and pass-through that scalar
            let mut it = slice.char_indices();
            it.next();
            if let Some((next_off, _)) = it.next() {
                out.push_str(&slice[..next_off]);
                i += next_off;
            } else {
                out.push_str(slice);
                break;
            }
        }
    }
    out
}

/// Normalize, rewrite, then max-munch tokenize to compiled char string.
/// Non-matching scalars are skipped.
pub fn tokenize_to_compiled(text: &str) -> String {
    let a = alphabet();
    let norm = normalize_for_tokenization(text);
    let norm = apply_rewrites(&norm);
    let bytes = norm.as_bytes();
    let mut out = String::new();
    let mut i = 0usize;

    while i < bytes.len() {
        let slice = &norm[i..];
        let mut matched = false;

        for t in &a.sorted {
            if slice.starts_with(&t.token) {
                out.push(t.compiled);
                i += t.token.len();
                matched = true;
                break;
            }
        }

        if !matched {
            // advance by one scalar (skip)
            let mut it = slice.char_indices();
            it.next(); // first char at offset 0
            if let Some((next_off, _)) = it.next() { i += next_off; }
            else { i = bytes.len(); }
        }
    }
    out
}

/// Compile a face string authored as tokens (e.g., "chaz") → compiled chars.
pub fn compile_face_string(face: &str) -> Vec<char> {
    tokenize_to_compiled(face).chars().collect()
}
