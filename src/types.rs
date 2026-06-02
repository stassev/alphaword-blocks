// AlphaWord Blocks — core crate types, knobs, and shared helpers
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

//! Core crate types, knobs, and small helpers that other modules import.
//!
//! This file is now **alphabet-size agnostic**. The effective alphabet is
//! compiled at runtime in `alphabet.rs` and exposed via lightweight accessors
//! here, so the rest of the code can ask for `n_letters()`, map chars to
//! indices, etc., without baking any fixed `[T; N_LETTERS]` sizes.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RepairMode {
    Balanced,
    HighFirst,
    Elastic,
}

impl RepairMode {
    pub fn normalize<S: AsRef<str>>(s: S) -> Self {
        match s.as_ref().to_lowercase().as_str() {
            "balanced" | "balance" => RepairMode::Balanced,
            "high" | "high_first" | "highfirst" => RepairMode::HighFirst,
            "elastic" | "soft" | "soft_guard" | "softguard" => RepairMode::Elastic,
            _ => RepairMode::Balanced,
        }
    }
}

pub fn repair_mode_help() -> &'static str {
    "Available repair modes:
  • balanced   — Protect R1 strictly; optimize R2/R3; gentle high-k pressure (1 high-k step/sweep).
  • high_first — Stronger focus on high-k deficits (3 steps/sweep); still protects R1 strictly.
  • elastic    — Allows small temporary regressions: can dip R1 by 1 and slightly harm R2/R3 during a high-k step.
                 High-k executed before singles each sweep so singles fix the debt quickly.\n"
}

/* ===================== Central knobs (single source of truth) ===================== */

pub const DECOMPOSE_HANGUL: bool = true;

/* ===================== Runtime alphabet (compiled from token list) ===================== */
/*
   We compile the effective ALPHABET at runtime in `alphabet.rs` from an ordered
   INPUT_ALPHABET (tokens). Each token is normalized according to the flags above,
   then mapped to a unique single `char` (PUA for multi-letter tokens). The rest
   of the system continues to operate on chars.

   Access via the helpers below; do not cache the result at init-time unless
   you've already called `alphabet::build_alphabet_*()`.
*/

#[inline]
pub fn n_letters() -> usize {
    crate::alphabet::n_letters()
}

// Map a character (compiled symbol) to its letter index in the runtime ALPHABET.
#[inline]
pub fn char_index(c: char) -> Option<usize> {
    crate::alphabet::char_index(c)
}

// Reverse: from letter index to its ALPHABET character (Unicode-safe).
#[inline]
pub fn index_char(i: usize) -> char {
    crate::alphabet::alphabet_chars()[i]
}

/* ===================== Cube geometry & sizing ===================== */

pub const FACES_PER_CUBE: usize = 6;
pub const N_CUBES: usize = 26; // bitset-based search/index assumes ≤64
pub const TOTAL_SLOTS: usize = FACES_PER_CUBE * N_CUBES;

/* ===================== Limits & schedules (kept, now dynamic) ===================== */

pub const MAX_K: usize = 20;

// `topk_for` computes directly from `n_letters()`.
// Preserves the old DEFAULT_TOPK_* behavior without compile-time sizes.
pub fn topk_for(k: usize) -> usize {
    let n = n_letters();
    match k {
        2 => n,
        3 => n,
        4 => n.min(13), 5 => n.min(11), 6 => n.min(8),
        7 => n.min(7),  8 => n.min(6),
        9 => n.min(5), 10 => n.min(5), 11 => n.min(4),
        12 => n.min(4), 13 => n.min(4),
        _ => n.min(std::cmp::max(3, 14_i32.saturating_sub(k as i32) as usize)),
    }
}

/* ---- KGE seeding knobs (k≥4 subsets from failing words) ---- */
pub const KGE_MAXK: usize = 26;

#[inline]
pub fn kge_topk_for(k: usize) -> usize {
    let n = n_letters();
    match k {
        4 | 5 | 6 | 7 => n,               // consider all letters by default
        _ => k,          // ~half for higher k
    }
}

/* ===================== Core index and helpers ===================== */

// NOTE: This used to be fixed-size arrays over N_LETTERS. Since ALPHABET is
// runtime-compiled now, we use Vec to allow arbitrary alphabet sizes.
#[derive(Clone)]
pub struct CubeIndex {
    // letter_masks[ℓ] has bit j set iff cube j has letter ℓ (0 <= j < N_CUBES).
    // NOTE: masks are u64; if N_CUBES > 64, higher cubes cannot be represented.
    pub letter_masks: Vec<u64>,
    // Precomputed sorted cube lists per letter for backtracking.
    pub cube_lists: Vec<Vec<usize>>,
}

impl Default for CubeIndex {
    fn default() -> Self {
        let n = n_letters();
        Self {
            letter_masks: vec![0u64; n],
            cube_lists: (0..n).map(|_| Vec::new()).collect(),
        }
    }
}

/// UNION coverage: #cubes that contain at least one letter in `key`.
#[inline]
pub fn coverage_for_tuple(idx: &CubeIndex, key: &[usize]) -> i32 {
    let mut mask = 0u64;
    for &j in key {
        mask |= idx.letter_masks[j];
    }
    mask.count_ones() as i32
}

/// Canonical ordering for 3 indices (used for map keys, etc.)
#[inline]
pub fn ord3(mut a: usize, mut b: usize, mut c: usize) -> (usize, usize, usize) {
    if a > b { std::mem::swap(&mut a, &mut b); }
    if b > c { std::mem::swap(&mut b, &mut c); }
    if a > b { std::mem::swap(&mut a, &mut b); }
    (a, b, c)
}
