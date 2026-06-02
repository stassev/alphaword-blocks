// AlphaWord Blocks — bipartite matching (Hall's theorem) for the optimizer
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

use std::collections::VecDeque;

/// Map a character to its index in ALPHABET.
#[inline]
fn li(c: char) -> Option<usize> {
    crate::types::char_index(c)
}

/// Return the ALPHABET character for a given letter index.
#[inline]
fn ci(i: usize) -> char {
    crate::types::index_char(i)
}

/// Compute a Hall-violating certificate for `word` on the *current* cubes given by `letter_masks`.
/// - `letter_masks[ℓ]` has bit j set iff cube j has letter ℓ (0 <= j < N_CUBES).
/// - Returns: Some((multiset_string, deficit)) if the word is *not* fully matchable; None if matchable.
///
/// Notes:
/// * This is fully ALPHABET-driven (Unicode-safe). No ASCII assumptions.
/// * If `word` contains chars outside ALPHABET, we log a WARN and skip those chars.
/// * We assume `letter_masks` uses only the lowest `N_CUBES` bits of each u64 mask.
///   If `N_CUBES > 64`, masks are insufficient; we warn once here.
pub fn hall_certificate_for_word(
    word: &str,
    letter_masks: &[u64], // dynamic: must equal n_letters()
) -> Option<(String, usize)> {
    const MAX_MASK_BITS: usize = 64;
    let n_right = crate::types::N_CUBES;

    assert_eq!(
        letter_masks.len(),
        crate::types::n_letters(),
        "[hall] letter_masks.len() must equal n_letters()"
    );

    if n_right > MAX_MASK_BITS {
        eprintln!(
            "[hall] WARN: N_CUBES = {} exceeds {}; letter_masks are u64, higher bits will be truncated.",
            n_right, MAX_MASK_BITS
        );
    }

    // Left side: one node per letter *instance* in the word.
    let mut left_letters: Vec<usize> = Vec::new();
    for ch in word.chars() {
        match li(ch) {
            Some(idx) => left_letters.push(idx),
            None => {
                // Guardrail: notify if a char isn't in ALPHABET (or couldn't be mapped)
                if !ch.is_whitespace() {
                    eprintln!(
                        "[hall] WARN: skipping char {:?} not in ALPHABET while analyzing {:?}",
                        ch, word
                    );
                }
            }
        }
    }
    let n_left = left_letters.len();
    if n_left == 0 {
        // Nothing to match; treat as trivially matchable
        return None;
    }

    // Build adjacency: for each left u, the list of right v (cubes) that contain that letter.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n_left];
    for (u, &ell) in left_letters.iter().enumerate() {
        let mut m = letter_masks[ell];
        while m != 0 {
            let v = m.trailing_zeros() as usize;
            if v < n_right {
                adj[u].push(v);
            }
            // Clear lowest set bit
            m &= m - 1;
        }
    }

    // Hopcroft–Karp maximum matching.
    let (matching, pair_u, pair_v) = hopcroft_karp(n_left, n_right, &adj);

    if matching == n_left {
        // Fully matchable -> no Hall deficit.
        return None;
    }

    // Extract a Hall witness S = reachable_left via alternating reachability.
    // (Start from unmatched lefts, follow UNmatched edges L->R, then matched edges R->L.)
    let (reachable_left, _reachable_right) =
        alternating_reachability(n_left, n_right, &adj, &pair_u, &pair_v);

    let mut cert_chars: Vec<char> = Vec::new();
    for u in 0..n_left {
        if reachable_left[u] {
            let ell = left_letters[u];
            cert_chars.push(ci(ell));
        }
    }

    // Safety fallback (should be unnecessary if matching < n_left): include unmatched lefts
    if cert_chars.is_empty() {
        for u in 0..n_left {
            if pair_u[u].is_none() {
                let ell = left_letters[u];
                cert_chars.push(ci(ell));
            }
        }
    }

    let deficit = n_left - matching; // |S| - |N(S)| equals this value
    Some((cert_chars.into_iter().collect(), deficit))
}

fn hopcroft_karp(
    n_left: usize,
    n_right: usize,
    adj: &Vec<Vec<usize>>,
) -> (usize, Vec<Option<usize>>, Vec<Option<usize>>) {
    let mut pair_u: Vec<Option<usize>> = vec![None; n_left];
    let mut pair_v: Vec<Option<usize>> = vec![None; n_right];
    let mut dist: Vec<i32> = vec![0; n_left];

    let mut matching = 0usize;

    while bfs_layering(n_left, adj, &pair_u, &pair_v, &mut dist) {
        for u in 0..n_left {
            if pair_u[u].is_none() && dfs_augment(u, adj, &mut dist, &mut pair_u, &mut pair_v) {
                matching += 1;
            }
        }
    }
    (matching, pair_u, pair_v)
}

fn bfs_layering(
    n_left: usize,
    adj: &Vec<Vec<usize>>,
    pair_u: &Vec<Option<usize>>,
    pair_v: &Vec<Option<usize>>,
    dist: &mut [i32],
) -> bool {
    let mut q = VecDeque::new();
    for u in 0..n_left {
        if pair_u[u].is_none() {
            dist[u] = 0;
            q.push_back(u);
        } else {
            dist[u] = -1;
        }
    }
    let mut found_free_right = false;
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if let Some(u2) = pair_v[v] {
                if dist[u2] == -1 {
                    dist[u2] = dist[u] + 1;
                    q.push_back(u2);
                }
            } else {
                // A free right is reachable in this layering.
                found_free_right = true;
            }
        }
    }
    found_free_right
}

fn dfs_augment(
    u: usize,
    adj: &Vec<Vec<usize>>,
    dist: &mut [i32],
    pair_u: &mut [Option<usize>],
    pair_v: &mut [Option<usize>],
) -> bool {
    for &v in &adj[u] {
        if let Some(u2) = pair_v[v] {
            if dist[u2] == dist[u] + 1 && dfs_augment(u2, adj, dist, pair_u, pair_v) {
                pair_v[v] = Some(u);
                pair_u[u] = Some(v);
                return true;
            }
        } else {
            pair_v[v] = Some(u);
            pair_u[u] = Some(v);
            return true;
        }
    }
    dist[u] = -1;
    false
}

fn alternating_reachability(
    n_left: usize,
    n_right: usize,
    adj: &Vec<Vec<usize>>,
    pair_u: &Vec<Option<usize>>,
    pair_v: &Vec<Option<usize>>,
) -> (Vec<bool>, Vec<bool>) {
    let mut reachable_left = vec![false; n_left];
    let mut reachable_right = vec![false; n_right];
    let mut q = VecDeque::new();

    // Start from *unmatched* left vertices
    for u in 0..n_left {
        if pair_u[u].is_none() {
            reachable_left[u] = true;
            q.push_back((true, u)); // (is_left, index)
        }
    }

    while let Some((is_left, idx)) = q.pop_front() {
        if is_left {
            // From left, traverse *unmatched* edges to right
            for &v in &adj[idx] {
                if pair_v[v] != Some(idx) && !reachable_right[v] {
                    reachable_right[v] = true;
                    q.push_back((false, v));
                }
            }
        } else {
            // From right, traverse *matched* edge back to left
            if let Some(u2) = pair_v[idx] {
                if !reachable_left[u2] {
                    reachable_left[u2] = true;
                    q.push_back((true, u2));
                }
            }
        }
    }
    (reachable_left, reachable_right)
}
