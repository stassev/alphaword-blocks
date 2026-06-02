// AlphaWord Blocks — spellability verification
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

// src/verify.rs
use crate::types::*;
use crate::util::now_sec;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Build a `CubeIndex` from raw letter masks (0-based cube bits).
/// Any bits set at positions ≥ N_CUBES are IGNORED (with a warning),
/// so caller bugs or stale data don’t explode correctness.
///
/// NOTE: Accepts a runtime-sized slice of masks.
#[inline]
pub fn build_index_from_masks(masks: &[u64]) -> CubeIndex {
    // The whole codebase uses u64 bitmasks for cube-sets, so N_CUBES must fit in 64.
    assert!(
        N_CUBES <= 64,
        "verify/build_index_from_masks: N_CUBES={} exceeds 64; bitmask-based index requires ≤64",
        N_CUBES
    );

    let n = n_letters();
    assert_eq!(
        masks.len(),
        n,
        "build_index_from_masks: masks.len()={} must equal n_letters()={}",
        masks.len(),
        n
    );

    // Dynamic, runtime-sized index
    let mut out = CubeIndex {
        letter_masks: vec![0u64; n],
        cube_lists: (0..n).map(|_| Vec::<usize>::new()).collect(),
    };

    // Mask out any stray bits ≥ N_CUBES, and warn if we dropped any.
    let mask_limit: u64 = if N_CUBES == 64 { u64::MAX } else { (1u64 << N_CUBES) - 1 };

    for i in 0..n {
        let raw = masks[i];
        let clamped = raw & mask_limit;
        if raw != clamped {
            eprintln!(
                "[build_index_from_masks] WARN: letter {} ('{}') mask had bits beyond N_CUBES ({}); \
                 ignoring extras. raw={:#018x} → clamped={:#018x}",
                i, index_char(i), N_CUBES, raw, clamped
            );
        }
        out.letter_masks[i] = clamped;

        // Expand bitset → sorted cube list.
        let mut lst = Vec::<usize>::new();
        let mut m = clamped;
        while m != 0 {
            let tz = m.trailing_zeros() as usize; // 0..N_CUBES-1
            debug_assert!(tz < N_CUBES);
            lst.push(tz);
            m &= m - 1; // clear lowest set bit
        }
        // Already monotone increasing due to trailing_zeros walk; sort for belt-and-suspenders.
        lst.sort_unstable();
        out.cube_lists[i] = lst;
    }

    out
}

/// Exact spellability test for a word with current cubes.
/// Unicode-safe: relies on `char_index` (which respects the runtime ALPHABET & lowercasing policy).
/// Uses backtracking with a u64 used-cube mask (requires N_CUBES ≤ 64) and a small symmetry
/// break for runs of identical letters to reduce duplicate search.
#[inline]
pub fn can_spell_word(word: &str, idx: &CubeIndex) -> bool {
    assert!(
        N_CUBES <= 64,
        "verify/can_spell_word: N_CUBES={} exceeds 64; bitmask-based search requires ≤64",
        N_CUBES
    );

    let n = n_letters();
    let mut counts = vec![0i32; n];
    let mut total = 0usize;
    for ch in word.chars() {
        if let Some(i) = char_index(ch) {
            counts[i] += 1;
            total += 1;
        } else {
            // Character not present (or not mappable) to the current ALPHABET.
            return false;
        }
    }
    if total > N_CUBES {
        // Pigeonhole: more letters than cubes.
        return false;
    }

    // Expand multiset into repeated indices and sort by ascending candidate cube count (fail-fast).
    let mut letters: Vec<usize> = Vec::with_capacity(total);
    for i in 0..n {
        for _ in 0..counts[i] {
            letters.push(i);
        }
    }
    // Deterministic tie-breaker: (candidate_count, letter_index).
    letters.sort_unstable_by_key(|&i| (idx.cube_lists[i].len(), i));

    // Early rejection: if any letter has no hosting cube.
    if letters.iter().any(|&i| idx.cube_lists[i].is_empty()) {
        return false;
    }

    #[inline]
    fn dfs(
        pos: usize,
        letters: &[usize],
        idx: &CubeIndex,
        used_mask: u64,
        min_cube_for_run: usize, // symmetry break for runs of identical letters
    ) -> bool {
        if pos == letters.len() {
            return true;
        }
        let letter = letters[pos];
        let cubes_for_letter = &idx.cube_lists[letter];

        for &cube in cubes_for_letter {
            if cube < min_cube_for_run {
                continue;
            }
            let bit = 1u64 << cube;
            if (used_mask & bit) != 0 {
                continue;
            }

            // If next is the same letter, enforce strictly increasing cube index within the run
            // (breaks permutations among identical letters).
            let next_min = if pos + 1 < letters.len() && letters[pos + 1] == letter {
                cube + 1
            } else {
                0
            };

            if dfs(pos + 1, letters, idx, used_mask | bit, next_min) {
                return true;
            }
        }
        false
    }

    dfs(0, &letters, idx, 0, 0)
}

/// Exhaustive verification across all words. Returns (ok, list_of_fails).
/// Stable, deterministic ordering of failures (original input order).
pub fn verify_all(
    words: &[String],
    idx: &CubeIndex,
    progress_every: usize,
    verbose: bool,
    ticker_interval: f64,
) -> (bool, Vec<String>) {
    let total = words.len();
    let last_tick = std::sync::Mutex::new(now_sec());
    let processed = AtomicUsize::new(0);

    let fails: Vec<(usize, String)> = words
        .par_iter()
        .enumerate()
        .filter_map(|(i, w)| {
            let ok = can_spell_word(w, idx);
            let done = processed.fetch_add(1, Ordering::Relaxed) + 1;

            if verbose && progress_every > 0 && done % progress_every == 0 {
                let mut lt = last_tick.lock().unwrap();
                let now = now_sec();
                if now - *lt >= ticker_interval {
                    let pct = 100.0 * (done as f64) / (total.max(1) as f64);
                    println!("Verify: {}/{} ({:.1}%)", done, total, pct);
                    *lt = now;
                }
            }

            if ok {
                None
            } else {
                Some((i, w.clone()))
            }
        })
        .collect();

    // Stable order by original index.
    let mut fails_sorted = fails;
    fails_sorted.sort_unstable_by_key(|x| x.0);
    let out: Vec<String> = fails_sorted.into_iter().map(|x| x.1).collect();

    (out.is_empty(), out)
}
