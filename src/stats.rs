// AlphaWord Blocks — corpus statistics
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

// src/stats.rs
use crate::types::*;
use hashbrown::HashMap;
use rayon::prelude::*;
use smallvec::SmallVec;
use crate::util::now_sec;
use std::sync::{atomic::{AtomicUsize, Ordering}, Mutex};

/* ---------------- Seeded “random” helpers (deterministic) ---------------- */

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

// Per-letter seeded tiebreak for resolving equal frequencies inside a single word.
#[inline]
fn tbletter(letter_idx: usize, tie_seed: u64) -> u64 {
    splitmix64(tie_seed ^ (letter_idx as u64).wrapping_mul(0xA0761D6478BD642F))
}

/* ---------- KGE seeded ordering helpers (deterministic; seed passed in) ---------- */

// CHANGED: slice-based (runtime alphabet size)
#[inline]
fn kge_word_signature_from_counts(cnt: &[u8]) -> u64 {
    // Deterministic per-word signature from its distinct letters + multiplicities.
    // Purely order-agnostic: only (letter index, count) pairs matter.
    let mut h = 0xA24B_AED4_963E_E407u64;
    for (i, &c) in cnt.iter().enumerate() {
        if c != 0 {
            h = splitmix64(
                h ^ ((i as u64).wrapping_mul(0xD6E8_FEB8_6659_FD93))
                  ^ ((c as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9))
            );
        }
    }
    h
}

#[inline]
fn kge_letter_weight(seed: u64, word_sig: u64, l: usize) -> u64 {
    // Seeded, per-letter weight for KGE pool ranking; reproducible.
    splitmix64(seed ^ word_sig ^ (l as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

/* ---------------- Pareto helpers ---------------- */

#[inline]
fn dominates_u8<const N: usize>(u: &[u8; N], v: &[u8; N]) -> bool {
    for i in 0..N {
        if u[i] < v[i] {
            return false;
        }
    }
    true
}

#[inline]
fn insert_maximal_u8<const N: usize>(vecs: &mut Vec<[u8; N]>, v: [u8; N]) -> bool {
    // If anything already dominates v, drop v.
    for u in vecs.iter() {
        if dominates_u8(u, &v) {
            return false;
        }
    }
    // Keep only entries not dominated by v, then push v.
    let mut keep = Vec::with_capacity(vecs.len() + 1);
    for u in vecs.iter() {
        if !dominates_u8(&v, u) {
            keep.push(*u);
        }
    }
    keep.push(v);
    *vecs = keep;
    true
}

#[inline]
fn dominates_slice(x: &[u8], y: &[u8]) -> bool {
    for i in 0..x.len() {
        if x[i] < y[i] {
            return false;
        }
    }
    true
}

#[inline]
fn insert_maximal_vec(vecs: &mut Vec<Vec<u8>>, v: Vec<u8>) {
    // Drop v if dominated by any existing vector.
    if vecs.iter().any(|u| dominates_slice(u, &v)) {
        return;
    }
    // Remove entries dominated by v.
    vecs.retain(|u| !dominates_slice(&v, u));
    vecs.push(v);
}

/* ---------------- Public Stats type ---------------- */

#[derive(Clone)]
pub struct Stats {
    // CHANGED: dynamic sizes (runtime alphabet)
    pub r1: Vec<i32>,
    pub r2v: HashMap<(usize, usize), Vec<[u8; 2]>>,
    pub r3v: HashMap<(usize, usize, usize), Vec<[u8; 3]>>,
    pub rhv: HashMap<Vec<usize>, Vec<Vec<u8>>>, // 4..MAX_K (sorted key)
    pub f: Vec<i64>,

    /// purely diagnostic: how many unique high-k (k≥4) keys were merged via the forced/Hall path
    pub rhv_forced_count: usize,
}

/* ---------------- Optional: external forced/Hall certs container ---------------- */

/// Provide forced (Hall) constraints to merge **after** building base stats.
/// Anything present here completely bypasses any per-word `topk_for(k)` pruning.
#[derive(Clone, Default)]
pub struct ForcedHall {
    pub r2v: HashMap<(usize, usize), Vec<[u8; 2]>>,
    pub r3v: HashMap<(usize, usize, usize), Vec<[u8; 3]>>,
    pub rhv: HashMap<Vec<usize>, Vec<Vec<u8>>>, // k>=4
}

impl ForcedHall {
    /// Add a single certificate string (multiset of letters) into the forced maps.
    /// Projects to all pairs, all triples, and the full k≥4 key.
    pub fn add_certificate(&mut self, cert: &str) {
        if cert.is_empty() { return; }

        // CHANGED: dynamic counts
        let n = n_letters();
        let mut cnt = vec![0u8; n];
        for ch in cert.chars() {
            if let Some(i) = crate::types::char_index(ch) {
                if cnt[i] != u8::MAX {
                    cnt[i] = cnt[i].saturating_add(1);
                }
            }
        }

        // Build sorted key of letters present
        let mut key: Vec<usize> = (0..n).filter(|&i| cnt[i] > 0).collect();
        if key.is_empty() { return; }
        key.sort_unstable();

        // k >= 4 projection (store exactly the multiset on that key)
        if key.len() >= 4 {
            let v: Vec<u8> = key.iter().map(|&i| cnt[i]).collect();
            insert_maximal_vec(self.rhv.entry(key.clone()).or_default(), v);
        }

        // Pair projections
        if key.len() >= 2 {
            for i in 0..key.len() {
                for j in (i + 1)..key.len() {
                    let a = key[i]; let b = key[j];
                    let (x, y) = if a < b { (a, b) } else { (b, a) };
                    let need = [cnt[x], cnt[y]];
                    insert_maximal_u8(self.r2v.entry((x, y)).or_default(), need);
                }
            }
        }

        // Triple projections
        if key.len() >= 3 {
            for i in 0..key.len() {
                for j in (i + 1)..key.len() {
                    for k in (j + 1)..key.len() {
                        let (a, b, c) = ord3(key[i], key[j], key[k]);
                        let need = [cnt[a], cnt[b], cnt[c]];
                        insert_maximal_u8(self.r3v.entry((a, b, c)).or_default(), need);
                    }
                }
            }
        }
    }

    /// Add many certificates.
    pub fn add_many<'a, I: IntoIterator<Item = &'a str>>(&mut self, it: I) {
        for s in it { self.add_certificate(s); }
    }

    /// Return (pairs, triples, high_k) counts for logging.
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.r2v.len(), self.r3v.len(), self.rhv.len())
    }
}

/* ---------------- Internal “need” evaluators (depend on current deg) ---------------- */
/* These compute the *target need* for unions (NOT coverage). Callers should compare to union coverage. */

// CHANGED: deg is a slice (runtime length)
#[inline]
fn active_union_target2(
    r2v: &HashMap<(usize, usize), Vec<[u8; 2]>>,
    a: usize,
    b: usize,
    deg: &[i32],
) -> i32 {
    if a == b {
        return 0;
    }
    let (aa, bb, swap) = if a < b { (a, b, false) } else { (b, a, true) };
    match r2v.get(&(aa, bb)) {
        None => 0,
        Some(vecs) => {
            let mut best = 0i32;
            for v in vecs {
                let (ra, rb) = if swap { (v[1], v[0]) } else { (v[0], v[1]) };
                if deg[a] >= ra as i32 && deg[b] >= rb as i32 {
                    let s = ra as i32 + rb as i32;
                    if s > best {
                        best = s;
                    }
                }
            }
            best
        }
    }
}

// CHANGED: deg is a slice (runtime length)
#[inline]
fn active_union_target3(
    r3v: &HashMap<(usize, usize, usize), Vec<[u8; 3]>>,
    a: usize,
    b: usize,
    c: usize,
    deg: &[i32],
) -> i32 {
    let (a1, a2, a3) = ord3(a, b, c);
    match r3v.get(&(a1, a2, a3)) {
        None => 0,
        Some(vecs) => {
            let mut best = 0i32;
            for v in vecs {
                let ra = if a == a1 { v[0] } else if a == a2 { v[1] } else { v[2] };
                let rb = if b == a1 { v[0] } else if b == a2 { v[1] } else { v[2] };
                let rc = if c == a1 { v[0] } else if c == a2 { v[1] } else { v[2] };
                if deg[a] >= ra as i32 && deg[b] >= rb as i32 && deg[c] >= rc as i32 {
                    let s = ra as i32 + rb as i32 + rc as i32;
                    if s > best {
                        best = s;
                    }
                }
            }
            best
        }
    }
}

// CHANGED: deg is a slice (runtime length)
#[inline]
fn active_union_target_vecs(vecs: &[Vec<u8>], key: &[usize], deg: &[i32]) -> i32 {
    let mut best = 0i32;
    'outer: for v in vecs {
        let mut s = 0i32;
        for j in 0..key.len() {
            if deg[key[j]] < v[j] as i32 {
                continue 'outer;
            }
            s += v[j] as i32;
        }
        if s > best {
            best = s;
        }
    }
    best
}

/* ---------------- Public re-exports ---------------- */

// CHANGED: deg is a slice
pub fn active_union2_pub(
    r2v: &HashMap<(usize, usize), Vec<[u8; 2]>>,
    a: usize,
    b: usize,
    deg: &[i32],
) -> i32 {
    active_union_target2(r2v, a, b, deg)
}

// CHANGED: deg is a slice
pub fn active_union3_pub(
    r3v: &HashMap<(usize, usize, usize), Vec<[u8; 3]>>,
    a: usize,
    b: usize,
    c: usize,
    deg: &[i32],
) -> i32 {
    active_union_target3(r3v, a, b, c, deg)
}

// CHANGED: deg is a slice
pub fn active_union_vecs_pub(vecs: &[Vec<u8>], key: &[usize], deg: &[i32]) -> i32 {
    active_union_target_vecs(vecs, key, deg)
}

/* ---------------- KGE subset generation  ---------------- */

/// Add all distinct-letter subsets (k=4..KGE_MAXK) for a word into `forced.rhv`.
/// Each subset is stored with a multiplicity vector of all ones (the KGE “uniqueing”).
/// The selection of which distinct letters to consider is **seeded** to avoid a..z bias.
pub fn add_certificate_all_subsets_kge4(forced: &mut ForcedHall, word: &str, kge_seed: u64) {
    // CHANGED: dynamic counts
    let n = n_letters();
    let mut cnt_u8 = vec![0u8; n];
    for ch in word.chars() {
        if let Some(i) = crate::types::char_index(ch) {
            if cnt_u8[i] != u8::MAX {
                cnt_u8[i] = cnt_u8[i].saturating_add(1);
            }
        }
    }

    // Distinct letters (indices)
    let mut pool: Vec<usize> = (0..n).filter(|&i| cnt_u8[i] > 0).collect();
    if pool.len() < 4 { return; }

    // Seeded permutation to avoid deterministic alphabetic bias before truncation
    let word_sig = kge_word_signature_from_counts(&cnt_u8);
    pool.sort_unstable_by(|&l1, &l2| {
        kge_letter_weight(kge_seed, word_sig, l1)
            .cmp(&kge_letter_weight(kge_seed, word_sig, l2))
            .then_with(|| l1.cmp(&l2)) // deterministic fallback (near-impossible tie)
    });

    // For each k, apply its own topK truncation (always ≥ k), then canonicalize
    let max_k = KGE_MAXK.min(pool.len());
    let mut cur = Vec::<usize>::new();

    fn rec(
        pool: &[usize],
        start: usize,
        depth: usize,
        k: usize,
        cur: &mut [usize],
        forced: &mut ForcedHall,
    ) {
        if depth == k {
            // materialize sorted key of letter indices
            let key: Vec<usize> = cur.iter().map(|&ix| pool[ix]).collect();
            let v = vec![1u8; k]; // uniqueing
            let dst = forced.rhv.entry(key).or_insert_with(Vec::new);
            insert_maximal_vec(dst, v);
            return;
        }
        let last_start = pool.len().saturating_sub(k - depth);
        let mut i = start;
        while i <= last_start {
            cur[depth] = i;
            rec(pool, i + 1, depth + 1, k, cur, forced);
            i += 1;
        }
    }

    for k in 4..=max_k {
        let topk = std::cmp::min(pool.len(), std::cmp::max(k, crate::types::kge_topk_for(k)));
        if topk < k { continue; }

        // Take the topk letters by seeded weight, then canonicalize for map keys
        let mut pool_k = pool[..topk].to_vec();
        pool_k.sort_unstable();

        if cur.len() != k { cur.resize(k, 0); }
        rec(&pool_k, 0, 0, k, &mut cur[..k], forced);
    }
}

/* ---------------- Main stats computation (SEEDed + wrapper) ---------------- */

/// Build base stats from words with per-word `topk_for(k)` pruning *only on the base*.
/// Forced/Hall constraints are merged later via `merge_forced_hall_after_cap`.
pub fn compute_letter_stats_k_seeded(words: &[String], max_k: usize, tie_seed: u64) -> Stats {
    #[derive(Default)]
    struct Part {
        // CHANGED: dynamic sizes
        r1: Vec<i32>, // per-chunk MAX per letter
        f:  Vec<i64>,
        r2v: HashMap<(usize, usize), Vec<[u8; 2]>>,
        r3v: HashMap<(usize, usize, usize), Vec<[u8; 3]>>,
        rhv: HashMap<Vec<usize>, Vec<Vec<u8>>>,
    }

    let total = words.len();
    let processed = AtomicUsize::new(0);
    let last_tick = Mutex::new(now_sec());
    const STATS_TICKER_SECS: f64 = 3.0;

    let n = n_letters();

    // Build per-chunk partials in parallel
    let parts: Vec<Part> = words
        .par_chunks(1024)
        .map(|chunk| {
            let mut part = Part {
                r1: vec![0i32; n],
                f:  vec![0i64; n],
                r2v: HashMap::new(),
                r3v: HashMap::new(),
                rhv: HashMap::new(),
            };

            let mut counts = vec![0i32; n];
            for w in chunk {
                // per-word counts
                counts.fill(0);
                for ch in w.chars() {
                    if let Some(i) = crate::types::char_index(ch) {
                        counts[i] += 1;
                    }
                }

                // === singles: MAX per letter over all words (NOT #words containing) ===
                for i in 0..n {
                    if counts[i] > part.r1[i] {
                        part.r1[i] = counts[i];
                    }
                    part.f[i] += counts[i] as i64;
                }

                // Build ordered list of letters present in this word.
                // Sort primarily by frequency (desc), tie-break by seeded per-letter jitter, then by index.
                // CHANGED: bounded SmallVec capacity (raise if your alphabet grows beyond 64)
                let mut ord: SmallVec<[(usize, i32); 64]> = SmallVec::new();
                for i in 0..n {
                    if counts[i] > 0 {
                        ord.push((i, counts[i]));
                    }
                }
                ord.sort_unstable_by(|&(i1, c1), &(i2, c2)| {
                    c2.cmp(&c1)
                      .then_with(|| tbletter(i1, tie_seed).cmp(&tbletter(i2, tie_seed)))
                      .then_with(|| i1.cmp(&i2))
                });

                // === k = 2 ===  (align counts to sorted key)
                if max_k >= 2 {
                    let p_lim = topk_for(2).min(ord.len());
                    for a in 0..p_lim {
                        for b in (a + 1)..p_lim {
                            let (ia, ca) = (ord[a].0, ord[a].1 as u8);
                            let (ib, cb) = (ord[b].0, ord[b].1 as u8);
                            let (aa, bb, ra, rb) =
                                if ia < ib { (ia, ib, ca, cb) } else { (ib, ia, cb, ca) };
                            let vecs = part.r2v.entry((aa, bb)).or_insert_with(Vec::new);
                            insert_maximal_u8(vecs, [ra, rb]);
                        }
                    }
                }

                // === k = 3 ===  (align counts to sorted key)
                if max_k >= 3 {
                    let p_lim = topk_for(3).min(ord.len());
                    for a in 0..p_lim {
                        for b in (a + 1)..p_lim {
                            for c in (b + 1)..p_lim {
                                let mut triples = [
                                    (ord[a].0, ord[a].1 as u8),
                                    (ord[b].0, ord[b].1 as u8),
                                    (ord[c].0, ord[c].1 as u8),
                                ];
                                triples.sort_unstable_by_key(|p| p.0);
                                let key = (triples[0].0, triples[1].0, triples[2].0);
                                let need = [triples[0].1, triples[1].1, triples[2].1];
                                let vecs = part.r3v.entry(key).or_insert_with(Vec::new);
                                insert_maximal_u8(vecs, need);
                            }
                        }
                    }
                }

                // === k >= 4 ===  (align counts to sorted key)
                for k in 4..=max_k.min(MAX_K) {
                    let p_lim = topk_for(k).min(ord.len());
                    if p_lim < k {
                        break;
                    }

                    fn rec(
                        start: usize,
                        depth: usize,
                        k: usize,
                        ord: &[(usize, i32)],
                        cur: &mut [usize], // indices into ord[], len == k
                        out: &mut HashMap<Vec<usize>, Vec<Vec<u8>>>,
                    ) {
                        if depth == k {
                            let mut pairs: Vec<(usize, u8)> =
                                cur.iter().map(|&ix| (ord[ix].0, ord[ix].1 as u8)).collect();
                            pairs.sort_unstable_by_key(|p| p.0);
                            let key: Vec<usize> = pairs.iter().map(|p| p.0).collect();
                            let v: Vec<u8> = pairs.iter().map(|p| p.1).collect();
                            let vecs = out.entry(key).or_insert_with(Vec::new);
                            insert_maximal_vec(vecs, v);
                            return;
                        }
                        for i in start..=ord.len() - (k - depth) {
                            cur[depth] = i;
                            rec(i + 1, depth + 1, k, ord, cur, out);
                        }
                    }

                    let mut cur = vec![0usize; k];
                    rec(0, 0, k, &ord[..p_lim], &mut cur, &mut part.rhv);
                }
            }

            let done = processed.fetch_add(chunk.len(), Ordering::Relaxed) + chunk.len();
            if total > 0 {
                let mut lt = last_tick.lock().unwrap();
                let now = now_sec();
                if now - *lt >= STATS_TICKER_SECS {
                    let pct = 100.0 * (done as f64) / (total as f64);
                    println!("Stats+: {}/{} ({:.1}%)", done, total, pct);
                    *lt = now;
                }
            }
            part
        })
        .collect();

    // Merge partials
    let n = n_letters();
    let mut r1 = vec![0i32; n]; // global MAX per letter
    let mut f  = vec![0i64; n];
    let mut r2v: HashMap<(usize, usize), Vec<[u8; 2]>> = HashMap::with_capacity(1 << 16);
    let mut r3v: HashMap<(usize, usize, usize), Vec<[u8; 3]>> = HashMap::with_capacity(1 << 16);
    let mut rhv: HashMap<Vec<usize>, Vec<Vec<u8>>> = HashMap::with_capacity(1 << 16);

    for p in parts {
        for i in 0..n {
            if p.r1[i] > r1[i] {
                r1[i] = p.r1[i];
            }
            f[i] += p.f[i];
        }
        for (k, v) in p.r2v {
            let dst = r2v.entry(k).or_insert_with(Vec::new);
            for vv in v {
                insert_maximal_u8(dst, vv);
            }
        }
        for (k, v) in p.r3v {
            let dst = r3v.entry(k).or_insert_with(Vec::new);
            for vv in v {
                insert_maximal_u8(dst, vv);
            }
        }
        for (k, v) in p.rhv {
            let dst = rhv.entry(k).or_insert_with(Vec::new);
            for vv in v {
                insert_maximal_vec(dst, vv.clone());
            }
        }
    }

    println!("Stats+: done.");
    Stats { r1, r2v, r3v, rhv, f, rhv_forced_count: 0 }
}

/// Merge **forced/Hall** constraints **after** the base stats have been built.
/// These additions *bypass* any per-word `topk_for(k)` pruning and are never capped.
pub fn merge_forced_hall_after_cap(stats: &mut Stats, forced: &ForcedHall) {
    // R2
    for (&k, vecs) in forced.r2v.iter() {
        let dst = stats.r2v.entry(k).or_insert_with(Vec::new);
        for &vv in vecs {
            insert_maximal_u8(dst, vv);
        }
    }
    // R3
    for (&k, vecs) in forced.r3v.iter() {
        let dst = stats.r3v.entry(k).or_insert_with(Vec::new);
        for &vv in vecs {
            insert_maximal_u8(dst, vv);
        }
    }
    // k>=4
    let mut newly_added = 0usize;
    for (key, vecs) in forced.rhv.iter() {
        let was_absent = !stats.rhv.contains_key(key);
        let dst = stats.rhv.entry(key.clone()).or_insert_with(Vec::new);
        let before = dst.len();
        for v in vecs {
            insert_maximal_vec(dst, v.clone());
        }
        if was_absent && !dst.is_empty() {
            newly_added += 1;
        } else if dst.len() > before && was_absent {
            newly_added += 1;
        }
    }
    stats.rhv_forced_count += newly_added;
}
