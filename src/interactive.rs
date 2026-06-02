// AlphaWord Blocks — interactive annealing block optimizer
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

// src/interactive.rs
use crate::{types::*, util::*, filters::*, stats::*, assign::*, verify::*};
use anyhow::Result;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashSet};
use std::io::Write;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
// IMPORTANT: use UNION coverage everywhere for “have”
use crate::types::coverage_for_tuple as union_coverage_for_tuple;
// bring the helpers we just added in types.rs
use crate::types::{repair_mode_help, RepairMode};
// util ticker alias
use crate::util::DEFAULT_TICKER_INTERVAL as UTIL_TICKER_INTERVAL;
// Hall certificate extractor
use crate::hall::hall_certificate_for_word;

// ---------- CONFIG ----------
const MIN_CHAR_LENGTH: usize = 0; // <- set this as you like (0 disables seeding)

// --- local type aliases to match stats.rs map shapes ---
type R2VMap = hashbrown::HashMap<(usize, usize), Vec<[u8; 2]>>;
type R3VMap = hashbrown::HashMap<(usize, usize, usize), Vec<[u8; 3]>>;
type RHVMap = hashbrown::HashMap<Vec<usize>, Vec<Vec<u8>>>;

fn dump_words<P: AsRef<std::path::Path>>(path: P, ws: &[String]) -> std::io::Result<()> {
    let mut f = std::fs::File::create(path)?;
    for w in ws {
        writeln!(f, "{}", w)?;
    }
    Ok(())
}

/* ============================ PARALLEL BLOCKERS ============================ */

#[inline]
fn for_each_combo_early<F>(pool: &[usize], k: usize, mut cb: F) -> bool
where
    F: FnMut(&[usize]) -> bool + Send,
{
    if k == 0 || k > pool.len() {
        return false;
    }
    let mut cur = vec![0usize; k];

    fn rec<F>(pool: &[usize], start: usize, depth: usize, k: usize, cur: &mut [usize], cb: &mut F) -> bool
    where
        F: FnMut(&[usize]) -> bool + Send,
    {
        if depth == k {
            return cb(cur);
        }
        let last_start = pool.len().saturating_sub(k - depth);
        let mut i = start;
        while i <= last_start {
            cur[depth] = pool[i];
            if rec(pool, i + 1, depth + 1, k, cur, cb) {
                return true; // early exit
            }
            i += 1;
        }
        false
    }

    rec(pool, 0, 0, k, &mut cur, &mut cb)
}

#[inline]
fn word_need2_local_nalloc(
    r2v: &R2VMap,
    a: usize,
    b: usize,
    counts: &[i32],
) -> i32 {
    let (x, y) = if a < b { (a, b) } else { (b, a) };
    if let Some(vecs) = r2v.get(&(x, y)) {
        let mut best = 0i32;
        for v in vecs {
            let (ra, rb) = if a < b { (v[0] as i32, v[1] as i32) } else { (v[1] as i32, v[0] as i32) };
            if counts[a] >= ra && counts[b] >= rb {
                let s = ra + rb;
                if s > best { best = s; }
            }
        }
        best
    } else {
        0
    }
}

#[inline]
fn word_need3_local_nalloc(
    r3v: &R3VMap,
    a: usize,
    b: usize,
    c: usize,
    counts: &[i32],
) -> i32 {
    let (a1, a2, a3) = ord3(a, b, c);
    if let Some(vecs) = r3v.get(&(a1, a2, a3)) {
        let mut best = 0i32;
        for v in vecs {
            let ra = v[if a == a1 { 0 } else if a == a2 { 1 } else { 2 }] as i32;
            let rb = v[if b == a1 { 0 } else if b == a2 { 1 } else { 2 }] as i32;
            let rc = v[if c == a1 { 0 } else if c == a2 { 1 } else { 2 }] as i32;
            if counts[a] >= ra && counts[b] >= rb && counts[c] >= rc {
                let s = ra + rb + rc;
                if s > best { best = s; }
            }
        }
        best
    } else {
        0
    }
}

#[inline]
fn word_needk_local_slice(
    rhv: &RHVMap,
    key: &[usize], // borrow Vec<usize> keys as slice → no allocation
    counts: &[i32],
) -> i32 {
    if let Some(vecs) = rhv.get(key) {
        let mut best = 0i32;
        'outer: for v in vecs {
            let mut s = 0i32;
            for (j, &idx) in key.iter().enumerate() {
                let req = v[j] as i32;
                if counts[idx] < req { continue 'outer; }
                s += req;
            }
            if s > best { best = s; }
        }
        best
    } else {
        0
    }
}

/// Parallel, early-exit search for “constraint-blocking” words.
fn constraint_blockers_parallel(
    all_words: &[String],
    idx: &CubeIndex,
    stats: &Stats,
    max_k: usize,
    limit: usize,
    deg: &[i32], // from built.deg (for R1 quick test)
) -> Vec<String> {
    let n = n_letters();

    let blocked: Vec<String> = all_words
        .par_iter()
        .filter_map(|w| {
            let mut counts = vec![0i32; n];
            for ch in w.chars() {
                if let Some(i) = char_index(ch) {
                    counts[i] += 1;
                }
            }

            // Singles quick fail
            for i in 0..n {
                if counts[i] > deg[i] {
                    return Some(w.clone());
                }
            }

            let idxs: Vec<usize> = (0..n).filter(|&i| counts[i] > 0).collect();
            if idxs.is_empty() { return None; }

            // Rank once by multiplicity
            let mut ranked: Vec<(i32, usize)> = idxs.iter().map(|&i| (counts[i], i)).collect();
            ranked.sort_unstable_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
            let ranked_only: Vec<usize> = ranked.into_iter().map(|x| x.1).collect();

            for k in 2..=max_k {
                if ranked_only.len() < k { break; }
                let lim = std::cmp::max(topk_for(k), k);
                let take = ranked_only.len().min(lim);
                if take < k { break; }

                let mut p = ranked_only[..take].to_vec();
                p.sort_unstable();

                let hit = for_each_combo_early(&p, k, |key| {
                    let need = match k {
                        2 => word_need2_local_nalloc(&stats.r2v, key[0], key[1], &counts),
                        3 => word_need3_local_nalloc(&stats.r3v, key[0], key[1], key[2], &counts),
                        _ => word_needk_local_slice(&stats.rhv, key, &counts),
                    };
                    if need == 0 { return false; }
                    // *** UNION coverage for “have” ***
                    let have_union = union_coverage_for_tuple(idx, key);
                    need > have_union
                });

                if hit { return Some(w.clone()); }
            }

            None
        })
        .collect();

    let mut uniq = blocked;
    uniq.sort();
    uniq.dedup();
    if uniq.len() > limit {
        uniq.truncate(limit);
    }
    uniq
}

/* =============================== IO HELPERS =============================== */

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    let _ = std::io::stdout().flush();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn parse_indices(input: &str, max_index: usize) -> Vec<usize> {
    let s = input.trim();
    if s.is_empty() { return Vec::new(); }
    let toks = s.split(|c: char| c == ',' || c.is_whitespace());
    let mut out = Vec::new();
    for tok in toks {
        let t = tok.trim();
        if t.is_empty() { continue; }
        if let Some(hy) = t.find('-') {
            let a = t[..hy].trim().parse::<usize>().ok();
            let b = t[hy + 1..].trim().parse::<usize>().ok();
            if let (Some(mut i), Some(mut j)) = (a, b) {
                if i > j { std::mem::swap(&mut i, &mut j); }
                for k in i..=j {
                    if (1..=max_index).contains(&k) { out.push(k); }
                }
            }
        } else if let Ok(k) = t.parse::<usize>() {
            if (1..=max_index).contains(&k) { out.push(k); }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/* =========================== INTERACTIVE HELPERS ========================== */

fn interactive_trim_over_list(all: &mut Vec<String>, listed: &[String]) -> Vec<String> {
    if listed.is_empty() { return Vec::new(); }
    let mut uniq = listed.to_vec();
    uniq.sort();
    uniq.dedup();
    println!("=== Candidate words ({}) ===", uniq.len());
    for (i, w) in uniq.iter().enumerate() {
        println!("{:3}) {}", i + 1, w);
    }
    println!("Enter indices to remove (e.g. '1 3-7 12'), or press ENTER to keep all:");
    let ans = prompt("> ");
    if ans.trim().is_empty() {
        let conf = prompt("Confirm keeping ALL listed words? [y/N]: ").to_lowercase();
        if conf == "y" || conf == "yes" { return Vec::new(); }
        return interactive_trim_over_list(all, listed);
    }
    let idxs = parse_indices(&ans, uniq.len());
    if idxs.is_empty() {
        println!("No valid indices parsed. Nothing removed.");
        return Vec::new();
    }
    let toremove: Vec<String> = idxs.into_iter().map(|i| uniq[i - 1].clone()).collect();
    println!("You selected to remove {} words:", toremove.len());
    for w in &toremove { println!("  - {}", w); }
    let conf = prompt("Confirm removal? [y/N]: ").to_lowercase();
    if conf == "y" || conf == "yes" {
        let set: HashSet<String> = toremove.iter().cloned().collect();
        let filtered: Vec<String> = all.iter().filter(|w| !set.contains(*w)).cloned().collect();
        *all = filtered;
        return toremove;
    } else {
        println!("Removal canceled.");
        Vec::new()
    }
}

fn interactive_noise_trim(all_words: &mut Vec<String>) -> Vec<String> {
    let cands = find_noisy_words(all_words);
    if cands.is_empty() {
        println!("[Noisy-filter] No heuristic-noise candidates found.");
        return Vec::new();
    }
    println!("[Noisy-filter] Found {} heuristic-noise candidates.", cands.len());
    let removed = interactive_trim_over_list(all_words, &cands);
    if !removed.is_empty() {
        println!("[Noisy-filter] Removed {} words:", removed.len());
        for w in &removed { println!("  - {}", w); }
    } else {
        println!("[Noisy-filter] Nothing removed.");
    }
    removed
}

fn interactive_trim_or_repair(
    all_words: &mut Vec<String>,
    candidates: &[String],
    current_sweeps: usize,
    current_mode: RepairMode,
) -> (String, Vec<String>, usize, RepairMode) {
    if candidates.is_empty() {
        println!("No candidate words to remove (by constraints).");
        return ("none".into(), Vec::new(), current_sweeps, current_mode);
    }

    let mut uniq = candidates.to_vec();
    uniq.sort();
    uniq.dedup();
    //let show_n = uniq.len().min(200);
    println!("=== Constraint-blocking words ({}) ===",  uniq.len());
    for (i, w) in uniq.iter().enumerate() {
        println!("{:3}) {}", i + 1, w);
    }

    println!("Choose an action:");
    println!("  • Enter indices to REMOVE (e.g. '1 3-7 12')");
    println!("  • Or type 'more' to try LONGER repairs (adds +1000 sweeps)");
    println!("  • Or type '+sweeps=NNN' to set a new sweep count");
    println!("  • Or type 'retry' to rebuild FROM SCRATCH with a NEW SEED (seed += 1)");
    println!("  • Or type '+seed=NNN' (or 'seed=NNN') to set an EXACT SEED and rebuild");
    println!("  • Or type 'modes' to list repair modes");
    println!("  • Or type 'mode=<balanced|high_first|elastic>' to switch");
    println!("  • Or just press ENTER to keep all and do nothing");

    let raw = prompt("> ");
    let s_trim = raw.trim();
    if s_trim.is_empty() {
        let conf = prompt("Confirm keeping ALL candidate words and not changing sweeps? [y/N]: ").to_lowercase();
        if conf == "y" || conf == "yes" {
            return ("none".into(), Vec::new(), current_sweeps, current_mode);
        } else {
            return interactive_trim_or_repair(all_words, candidates, current_sweeps, current_mode);
        }
    }

    // --- Repair fix: interpret commands BEFORE trying to parse indices; be case-insensitive.
    let s = s_trim.to_ascii_lowercase();

    if s == "more" {
        let val = current_sweeps + 1000;
        println!("Okay, will try longer repairs: sweeps = {}", val);
        return ("sweeps".into(), Vec::new(), val, current_mode);
    }

    if let Some(rest) = s.strip_prefix("+sweeps=").or_else(|| s.strip_prefix("sweeps=")) {
        if let Ok(val) = rest.parse::<usize>() {
            println!("Okay, will set sweeps = {}", val);
            return ("sweeps".into(), Vec::new(), val, current_mode);
        }
        println!("Could not parse number after +sweeps=. No change.");
        return ("none".into(), Vec::new(), current_sweeps, current_mode);
    }

    if s == "retry" {
        println!("Okay, will bump the seed and rebuild from scratch.");
        return ("seed_bump".into(), Vec::new(), current_sweeps, current_mode);
    }

    if let Some(rest) = s.strip_prefix("seed=").or_else(|| s.strip_prefix("+seed=")) {
        if let Ok(val) = rest.parse::<i64>() {
            println!("Okay, will set seed={} and rebuild from scratch.", val);
            return ("seed_set".into(), vec![val.to_string()], current_sweeps, current_mode);
        }
        println!("Could not parse seed value. No change.");
        return ("none".into(), Vec::new(), current_sweeps, current_mode);
    }

    if s == "modes" || s == "mode" || s == "mode?" {
        println!("{}", repair_mode_help());
        let ans2 = prompt("Enter one: balanced | high_first | elastic  (ENTER to cancel) > mode=").to_lowercase();
        if ans2.trim().is_empty() {
            println!("Mode unchanged.");
            return ("none".into(), Vec::new(), current_sweeps, current_mode);
        } else {
            let new_mode = RepairMode::normalize(ans2);
            println!("Switching repair mode to {:?}.", new_mode);
            return ("mode_set".into(), Vec::new(), current_sweeps, new_mode);
        }
    }

    if let Some(rest) = s.strip_prefix("mode=").or_else(|| s.strip_prefix("+mode=")) {
        let new_mode = RepairMode::normalize(rest);
        println!("Switching repair mode to {:?}.", new_mode);
        return ("mode_set".into(), Vec::new(), current_sweeps, new_mode);
    }

    // Otherwise, treat as indices/ranges
    let idxs = parse_indices(s_trim, uniq.len());
    if idxs.is_empty() {
        println!("No valid indices parsed. Nothing removed.");
        return ("none".into(), Vec::new(), current_sweeps, current_mode);
    }
    let toremove: Vec<String> = idxs.into_iter().map(|i| uniq[i - 1].clone()).collect();
    println!("You selected to remove {} words:", toremove.len());
    for w in &toremove { println!("  - {}", w); }
    let conf = prompt("Confirm removal? [y/N]: ").to_lowercase();
    if conf == "y" || conf == "yes" {
        let set: HashSet<String> = toremove.iter().cloned().collect();
        let filtered: Vec<String> = all_words.iter().filter(|w| !set.contains(*w)).cloned().collect();
        *all_words = filtered;
        return ("remove".into(), toremove, current_sweeps, current_mode);
    } else {
        println!("Removal canceled.");
        return ("none".into(), Vec::new(), current_sweeps, current_mode);
    }
}

/* ========================= FORCED CONSTRAINTS (HALL) ========================== */

/// Add a Hall certificate string into a `ForcedHall` accumulator.
/// Returns (pairs_added, triples_added, higher_added) counts of *keys* touched.
fn add_certificate_to_forced(forced: &mut ForcedHall, cert: &str) -> (usize, usize, usize) {
    let n = n_letters();
    let mut cnt = vec![0u8; n];
    for ch in cert.chars() {
        if let Some(i) = char_index(ch) {
            if cnt[i] < u8::MAX { cnt[i] = cnt[i].saturating_add(1); }
        }
    }
    let mut key: Vec<usize> = (0..n).filter(|&i| cnt[i] > 0).collect();
    if key.is_empty() { return (0, 0, 0); }
    key.sort_unstable();

    // k >= 4 vector
    let vec_u8: Vec<u8> = key.iter().map(|&i| cnt[i]).collect();
    let was_absent_high = !forced.rhv.contains_key(&key);
    forced.rhv.entry(key.clone()).or_default().push(vec_u8);

    // project to all pairs
    let mut pair_keys_added = 0usize;
    if key.len() >= 2 {
        for i in 0..key.len() {
            for j in (i+1)..key.len() {
                let a = key[i]; let b = key[j];
                let (x, y) = if a < b { (a, b) } else { (b, a) };
                let first = !forced.r2v.contains_key(&(x, y));
                forced.r2v.entry((x,y)).or_default().push([cnt[x], cnt[y]]);
                if first { pair_keys_added += 1; }
            }
        }
    }

    // project to all triples
    let mut triple_keys_added = 0usize;
    if key.len() >= 3 {
        for i in 0..key.len() {
            for j in (i+1)..key.len() {
                for k in (j+1)..key.len() {
                    let (a,b,c) = ord3(key[i], key[j], key[k]);
                    let first = !forced.r3v.contains_key(&(a,b,c));
                    let v = [cnt[a], cnt[b], cnt[c]];
                    forced.r3v.entry((a,b,c)).or_default().push(v);
                    if first { triple_keys_added += 1; }
                }
            }
        }
    }

    (pair_keys_added, triple_keys_added, if was_absent_high && key.len() >= 4 { 1 } else { 0 })
}

/* =============================== MAIN DRIVER ============================== */

pub fn interactive_main(seed0: i64, sweeps0: usize, max_k: usize) -> Result<()> {
    // NOTE: The runtime alphabet & rewrites are built in main.rs before calling here.

    println!("=== Loading corpora ===");
    let mut all_words: Vec<String> = Vec::new();
    // Bundled public-domain / permissively-licensed English word lists.
    // See DATA_SOURCES.md for provenance and licenses. Additional corpora
    // (Tatoeba, Wiktionary, Wikipedia, dwyl/english-words, SCOWL) can be
    // fetched with the scripts in download_scripts/ and added here once the
    // resulting files are present in data/.
    for f in [
        "webster.txt", "enable1.txt", "wordnet.txt", "nltk_words.txt",
    ] {
        all_words.extend(read_words(format!("data/{}", f)));
    }
    


    println!("=== Filtering ===");
    
    all_words = remove_long_words(all_words, n_letters());


    // Drop pure Roman numerals (same “batch filter” style as other helpers)
    let before_roman = all_words.len();
    all_words = remove_roman_numerals(all_words);
    let after_roman = all_words.len();
    println!(
        "[Roman-filter] Removed {} entries that were Roman numerals.",
        before_roman.saturating_sub(after_roman)
    );

    // 1) Drop empties
    all_words.retain(|w| !w.is_empty());
    // 2) Deduplicate
    let mut uniq = HashSet::with_capacity(all_words.len());
    all_words.retain(|w| uniq.insert(w.clone()));
    // 3) Canonical lexicographic order
    all_words.sort_unstable();
    // 4) Deterministic shuffle to avoid lexicographic bias (seeded)
    {
        let mut rng = ChaCha20Rng::seed_from_u64(seed0 as u64);
        all_words.shuffle(&mut rng);
    }

    println!("=== Sample (first/last) ===");
    for m in 0..all_words.len().min(10) { println!("{}", all_words[m]); }
    let l = all_words.len();
    for m in 0..(l.saturating_sub(1)).min(10) { println!("{}", all_words[l - 1 - m]); }
    if let Err(e) = dump_words("rs_checked_words.txt", &all_words) {
        eprintln!("WARN: couldn't write rs_checked_words.txt: {e}");
    } else {
        println!("Wrote rs_checked_words.txt ({} words)", all_words.len());
    }

    // Noise pass
    let mut discarded: Vec<String> = Vec::new();
    let noise_removed = interactive_noise_trim(&mut all_words);
    discarded.extend(noise_removed);

    // Iterative loop
    let mut sweeps = sweeps0;
    let mut curseed = seed0;
    let mut cur_mode = RepairMode::Balanced;

    // Cache base stats so we don’t recompute unless corpus changed
    let mut base_stats_cache: Option<Stats> = None;
    let mut stats_dirty = true;

    // === NEW: persistent "forced" constraints (no top-K pruning) ===
    let mut forced = ForcedHall::default();

    // === Warm-start slot (scoped to this run) ===
    let mut warm_start: Option<Built> = None;

    // one-time seeding guards
    let mut seeded_fullword_constraints = false; //TURN OFF SET TO TRUE
    let mut seeded_kge_subsets = false;
let mut base_constraints_reported = false;
    loop {
        println!();

        // Compute/reuse BASE stats on the corpus ONLY (no replication; no overlay)
        let base_stats: Stats = if stats_dirty || base_stats_cache.is_none() {
            println!("=== Computing BASE corpus stats (R1..R{} vec) ===", max_k);
            let s = compute_letter_stats_k_seeded(&all_words, max_k, seed0 as u64);
            base_stats_cache = Some(s);
            stats_dirty = false;
            base_stats_cache.as_ref().unwrap().clone()
        } else {
            println!("=== Reusing cached BASE corpus stats (R1..R{}) ===", max_k);
            base_stats_cache.as_ref().unwrap().clone()
        };
        
      if !base_constraints_reported{
// === ONE-SHOT CONSTRAINT REPORT using base_stats (R2 & R3) ===
// Insert this immediately after `let base_stats: Stats = ...;`
// === ONE-SHOT CONSTRAINT REPORT using base_stats (R2 & R3) ===
// Insert this immediately after `let base_stats: Stats = ...;`
{
    use std::collections::{HashMap, HashSet};

    println!("[base-constraint-report] using base_stats (one-shot)");

    // Defensive: check that base_stats actually has r2v / r3v maps
    // (If your Stats struct uses different field names, update these)
    let has_r2 = true;
    let has_r3 = true;

    if !has_r2 && !has_r3 {
        println!("[base-constraint-report] no R2/R3 data available in base_stats -> skipping");
    } else {
        let n = n_letters();

        // Precompute per-word counts for letters (same mapping used elsewhere)
        let mut word_counts: Vec<Vec<usize>> = Vec::with_capacity(all_words.len());
        for w in &all_words {
            let mut cnt = vec![0usize; n];
            for ch in w.chars() {
                if let Some(i) = char_index(ch) {
                    cnt[i] += 1;
                }
            }
            word_counts.push(cnt);
        }

        // --- R1: best per single letter (unchanged) ---
        let mut max_counts = vec![0usize; n];
        let mut best_single: Vec<HashSet<String>> = vec![HashSet::new(); n];

        for (w, cnt) in all_words.iter().zip(word_counts.iter()) {
            for i in 0..n {
                let c = cnt[i];
                if c == 0 { continue; }
                if c > max_counts[i] {
                    max_counts[i] = c;
                    best_single[i].clear();
                    best_single[i].insert(w.clone());
                } else if c == max_counts[i] {
                    best_single[i].insert(w.clone());
                }
            }
        }

        println!("=== R1 (single-letter) : letter => max count => words ===");
        for i in 0..n {
            if max_counts[i] == 0 { continue; }
            let ch = index_char(i);
            let mut bs: Vec<_> = best_single[i].iter().cloned().collect::<Vec<String>>();
            bs.sort();
            let joined = bs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
            println!("'{}' : {} => {}", ch, max_counts[i], joined);
        }

        // --- R2: read multiplicity vectors from base_stats.r2v and keep only maximal (non-subsumed) ones ---
        let mut r2_constraints: HashMap<(usize,usize), Vec<((usize,usize), HashSet<String>)>> = HashMap::new();
        if {
            !base_stats.r2v.is_empty()
        } {
            for (&(i,j), vecs) in base_stats.r2v.iter() {
                // collect distinct multiplicities present in base_stats.r2v for this (i,j)
                let mut seen: Vec<(usize,usize)> = Vec::new();
                for v in vecs {
                    if v.len() >= 2 {
                        let ri = v[0] as usize;
                        let rj = v[1] as usize;
                        if !seen.contains(&(ri, rj)) {
                            seen.push((ri, rj));
                        }
                    }
                }
                if seen.is_empty() { continue; }

                // compute maximal multiplicities (drop any m that is subsumed by another m')
                let mut maximal: Vec<(usize,usize)> = Vec::new();
                'outer_pair: for &m in &seen {
                    for &n in &seen {
                        if n != m && n.0 >= m.0 && n.1 >= m.1 {
                            // n subsumes m, skip m
                            continue 'outer_pair;
                        }
                    }
                    maximal.push(m);
                }

                // deterministic ordering: (sum desc, ri desc, rj desc)
                maximal.sort_by(|a,b| {
                    let sa = a.0 + a.1;
                    let sb = b.0 + b.1;
                    sb.cmp(&sa).then(b.0.cmp(&a.0)).then(b.1.cmp(&a.1))
                });

                let mut vec_constraints = Vec::with_capacity(maximal.len());
                for (ri, rj) in maximal {
                    let mut satisfying: HashSet<String> = HashSet::new();
                    for (w, cnt) in all_words.iter().zip(word_counts.iter()) {
                        if cnt[i] >= ri && cnt[j] >= rj {
                            satisfying.insert(w.clone());
                        }
                    }
                    vec_constraints.push(((ri, rj), satisfying));
                }
                if !vec_constraints.is_empty() {
                    r2_constraints.insert((i,j), vec_constraints);
                }
            }
        } else {
            println!("[base-constraint-report] base_stats.r2v appears empty");
        }

        println!("=== R2 (base_stats pair constraints - maximal only) ===");
        let mut pair_keys: Vec<_> = r2_constraints.keys().cloned().collect();
        pair_keys.sort_by_key(|&(i,j)| i * 100 + j);
        for (i, j) in pair_keys {
            let a = index_char(i);
            let b = index_char(j);
            println!("Pair '{}{}' constraints (format: (req_i,req_j) => sum => #words):", a, b);
            let entries = r2_constraints.get(&(i,j)).unwrap();
            for ((ri, rj), words_set) in entries {
                let sum = ri + rj;
                let mut ws: Vec<_> = words_set.iter().cloned().collect::<Vec<String>>();
                ws.sort();
                let joined = ws.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("  ({},{}) => sum={} => {} word(s): {}", ri, rj, sum, ws.len(), joined);
            }
        }

        // --- R3: read multiplicity vectors from base_stats.r3v and keep only maximal ones ---
        let mut r3_constraints: HashMap<(usize,usize,usize), Vec<((usize,usize,usize), HashSet<String>)>> = HashMap::new();
        if {
            !base_stats.r3v.is_empty()
        } {
            for (&(i,j,k), vecs) in base_stats.r3v.iter() {
                // collect distinct multiplicities
                let mut seen: Vec<(usize,usize,usize)> = Vec::new();
                for v in vecs {
                    if v.len() >= 3 {
                        let ri = v[0] as usize;
                        let rj = v[1] as usize;
                        let rk = v[2] as usize;
                        if !seen.contains(&(ri, rj, rk)) {
                            seen.push((ri, rj, rk));
                        }
                    }
                }
                if seen.is_empty() { continue; }

                // compute maximal (non-subsumed) multiplicities
                let mut maximal: Vec<(usize,usize,usize)> = Vec::new();
                'outer_trip: for &m in &seen {
                    for &n in &seen {
                        if n != m && n.0 >= m.0 && n.1 >= m.1 && n.2 >= m.2 {
                            // n subsumes m, skip m
                            continue 'outer_trip;
                        }
                    }
                    maximal.push(m);
                }

                // deterministic ordering: (sum desc, ri desc, rj desc, rk desc)
                maximal.sort_by(|a,b| {
                    let sa = a.0 + a.1 + a.2;
                    let sb = b.0 + b.1 + b.2;
                    sb.cmp(&sa)
                        .then(b.0.cmp(&a.0))
                        .then(b.1.cmp(&a.1))
                        .then(b.2.cmp(&a.2))
                });

                let mut vec_constraints = Vec::with_capacity(maximal.len());
                for (ri, rj, rk) in maximal {
                    let mut satisfying: HashSet<String> = HashSet::new();
                    for (w, cnt) in all_words.iter().zip(word_counts.iter()) {
                        if cnt[i] >= ri && cnt[j] >= rj && cnt[k] >= rk {
                            satisfying.insert(w.clone());
                        }
                    }
                    vec_constraints.push(((ri, rj, rk), satisfying));
                }
                if !vec_constraints.is_empty() {
                    r3_constraints.insert((i,j,k), vec_constraints);
                }
            }
        } else {
            println!("[base-constraint-report] base_stats.r3v appears empty");
        }

        println!("=== R3 (base_stats triplet constraints - maximal only) ===");
        let mut trip_keys: Vec<_> = r3_constraints.keys().cloned().collect();
        trip_keys.sort_by_key(|&(i,j,k)| i * 10000 + j * 100 + k);
        for (i, j, k) in trip_keys {
            let a = index_char(i);
            let b = index_char(j);
            let c = index_char(k);
            println!("Triplet '{}{}{}' constraints (format: (ri,rj,rk) => sum => #words):", a, b, c);
            let entries = r3_constraints.get(&(i,j,k)).unwrap();
            for ((ri, rj, rk), words_set) in entries {
                let sum = ri + rj + rk;
                let mut ws: Vec<_> = words_set.iter().cloned().collect::<Vec<String>>();
                ws.sort();
                let joined = ws.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("  ({},{},{}) => sum={} => {} word(s): {}", ri, rj, rk, sum, ws.len(), joined);
            }
        }

        // --- Count per-word appearances across R1/R2/R3 constraints (as before) AND appearances "alone" in a constraint ---
        let mut counts_map: HashMap<String, (usize, usize, usize)> = HashMap::new(); // r1,r2,r3 totals
        let mut alone_map: HashMap<String, (usize, usize, usize)> = HashMap::new();  // r1_alone, r2_alone, r3_alone

        // R1 totals and alone
        for i in 0..n {
            let set = &best_single[i];
            for w in set.iter() {
                let e = counts_map.entry(w.clone()).or_insert((0usize,0usize,0usize));
                e.0 += 1;
            }
            if set.len() == 1 {
                // get the single element
                let only = set.iter().next().unwrap();
                let a = alone_map.entry(only.clone()).or_insert((0usize,0usize,0usize));
                a.0 += 1;
            }
        }

        // R2 totals and alone
        for (_pair, entries) in r2_constraints.iter() {
            for (_req, words_set) in entries.iter() {
                for w in words_set.iter() {
                    let e = counts_map.entry(w.clone()).or_insert((0usize,0usize,0usize));
                    e.1 += 1;
                }
                if words_set.len() == 1 {
                    let only = words_set.iter().next().unwrap();
                    let a = alone_map.entry(only.clone()).or_insert((0usize,0usize,0usize));
                    a.1 += 1;
                }
            }
        }

        // R3 totals and alone
        for (_trip, entries) in r3_constraints.iter() {
            for (_req, words_set) in entries.iter() {
                for w in words_set.iter() {
                    let e = counts_map.entry(w.clone()).or_insert((0usize,0usize,0usize));
                    e.2 += 1;
                }
                if words_set.len() == 1 {
                    let only = words_set.iter().next().unwrap();
                    let a = alone_map.entry(only.clone()).or_insert((0usize,0usize,0usize));
                    a.2 += 1;
                }
            }
        }

        // Build accum with both "alone" columns and totals
        // tuple: (alone_total, alone_r1, alone_r2, alone_r3, total, r1, r2, r3, word)
        let mut accum: Vec<(usize, usize, usize, usize, usize, usize, usize, usize, String)> = Vec::new();
        for (w, (r1c, r2c, r3c)) in counts_map.into_iter() {
            let total = r1c + r2c + r3c;
            let (ar1, ar2, ar3) = alone_map.remove(&w).unwrap_or((0usize,0usize,0usize));
            let alone_total = ar1 + ar2 + ar3;
            accum.push((alone_total, ar1, ar2, ar3, total, r1c, r2c, r3c, w));
        }

        // Sort by:
        // 1) alone_total desc
        // 2) total desc
        // 3) r3 desc
        // 4) r2 desc
        // 5) r1 desc
        // 6) word asc
        accum.sort_by(|a, b| {
            b.0.cmp(&a.0) // alone_total
                .then(b.4.cmp(&a.4)) // total
                .then(b.7.cmp(&a.7)) // r3
                .then(b.6.cmp(&a.6)) // r2
                .then(b.5.cmp(&a.5)) // r1
                .then(a.8.cmp(&b.8)) // word (asc)
        });

        // Print header to show new columns
        println!("=== Ranked words by 'alone' then total appearances ===");
        println!("alone_tot  aR1 aR2 aR3   total   R1   R2   R3   word");
        for (alone_total, ar1, ar2, ar3, total, r1c, r2c, r3c, w) in accum {
            println!(
                "{:>9}  {:>3} {:>3} {:>3}  {:>6}  {:>3}  {:>3}  {:>3}  {}",
                alone_total, ar1, ar2, ar3, total, r1c, r2c, r3c, w
            );
        }
    }
}


    base_constraints_reported = true;
}




        // --- NEW: one-time full-word seeding BEFORE merging forced
        if !seeded_fullword_constraints && MIN_CHAR_LENGTH > 0 {
            let (p0, t0, h0) = forced.counts();
            let long_iter = all_words
                .iter()
                .filter(|w| w.len() >= MIN_CHAR_LENGTH)
                .map(|s| s.as_str());
            forced.add_many(long_iter); // projects to pairs, triples, and k≥4
            let (p1, t1, h1) = forced.counts();
            println!(
                "[Seed] Full-word constraints len≥{}: pairs=+{} triples=+{} higher=+{}",
                MIN_CHAR_LENGTH,
                p1.saturating_sub(p0),
                t1.saturating_sub(t0),
                h1.saturating_sub(h0)
            );
            seeded_fullword_constraints = true;
        }

        // --- NEW: one-time KGE subset seeding for long words (seeded to curseed)
        if !seeded_kge_subsets && MIN_CHAR_LENGTH > 0 {
            let (p0, t0, h0) = forced.counts();

            let longs: Vec<&String> = all_words.iter().filter(|w| w.len() >= MIN_CHAR_LENGTH).collect();
            let total = longs.len();
            println!(
                "[KGE] Seeding k-subsets (k=4..{}), on {} long words…",
                crate::types::KGE_MAXK, total
            );

            let mut done = 0usize;
            let start = std::time::Instant::now();
            for w in longs {
                add_certificate_all_subsets_kge4(&mut forced, w, curseed as u64);
                done += 1;
                if done % 50 == 0 {
                    let dt = start.elapsed().as_secs_f64();
                    let rate = done as f64 / dt.max(1e-6);
                    println!("[KGE] {}/{} ({:.1}%)  ~{:.1} words/s", done, total, 100.0*done as f64/total.max(1) as f64, rate);
                }
            }
            let (p1, t1, h1) = forced.counts();
            println!(
                "[KGE] Added high-k keys: +{} (pairs/triples unchanged: +{}, +{})",
                h1.saturating_sub(h0),
                p1.saturating_sub(p0),
                t1.saturating_sub(t0)
            );
            seeded_kge_subsets = true;
        }

        // Merge forced constraints (injected post-compute, cannot be pruned)
        let mut stats = base_stats.clone();
        merge_forced_hall_after_cap(&mut stats, &forced);

        let r1sum: i32 = stats.r1.iter().sum();
        println!(
            "Capacity check (metric set = base + forced): sum(R1)={} vs faces={}.",
            r1sum, TOTAL_SLOTS
        );

        // Diagnostics: how many unique forced keys currently active
        let forced_pairs = forced.r2v.len();
        let forced_triples = forced.r3v.len();
        let forced_high = forced.rhv.len();
 
        
        println!(
            "\n=== Building cubes (greedy + repairs, sweeps={}, seed={}, mode={:?}, forced_keys: pairs={}, triples={}, higher={}) ===",
            sweeps, curseed, cur_mode, forced_pairs, forced_triples, forced_high
        );

        let built = if let Some(ws) = warm_start.take() {
            println!("Re-anneal warm-start from previous best…");
            assign_letters_reanneal_from_mode(&stats, sweeps, curseed, true, cur_mode, &ws)
        } else {
            assign_letters_once_mode(&all_words, &stats, sweeps, curseed, true, cur_mode)
        };
        print_cubes_human(&built);
        print_cubes_code(&built);

        // Build index & quick deficit scan for higher-k using the same stats (with forced merged)
        let idx = build_index_from_masks(&built.letter_masks);

        // Higher-k deficits summary
        let mut dks: BTreeMap<usize, i32> = BTreeMap::new();
        for k in 4..=max_k {
            let mut tot = 0i32;
            for (key, vecs) in stats.rhv.iter() {
                if key.len() != k { continue; }
                let need = active_union_vecs_pub(vecs, key, &built.deg);
                // *** UNION coverage ***
                let have = union_coverage_for_tuple(&idx, key);
                if need > have { tot += need - have; }
            }
            dks.insert(k, tot);
        }

        let singles: i32 = (0..n_letters()).map(|i| (stats.r1[i] - built.deg[i]).max(0)).sum();

        // R2/R3 deficits
        let mut pairs = 0i32;
        for ((a, b), vecs) in stats.r2v.iter() {
            // convert once for the generic vecs-based helper
            let as_vecs: Vec<Vec<u8>> = vecs.iter().map(|v| vec![v[0], v[1]]).collect();
            let need = active_union_vecs_pub(&as_vecs, &[*a, *b], &built.deg);
            let have = built.union2[*a][*b];
            if need > have { pairs += need - have; }
        }
        let mut triples = 0i32;
        for ((a, b, c), vecs) in stats.r3v.iter() {
            let as_vecs: Vec<Vec<u8>> = vecs.iter().map(|v| vec![v[0], v[1], v[2]]).collect();
            let need = active_union_vecs_pub(&as_vecs, &[*a, *b, *c], &built.deg);
            let (x1, x2, x3) = ord3(*a, *b, *c);
            let have = built.union3[x1][x2][x3];
            if need > have { triples += need - have; }
        }
        let total_def: i32 = singles + pairs + triples + dks.values().sum::<i32>();

        if total_def == 0 {
            println!("All constraints R1..R{} satisfied ✅ (with forced constraints)", max_k);
            println!("Starting exhaustive verification…");
            // VERIFY ALWAYS on the base corpus
            let (ok, fails) = verify_all(&all_words, &idx, 500, true, UTIL_TICKER_INTERVAL);
            if ok {
                println!("Exhaustive verify PASSED ✅");
                break;
            } else {
                println!("Exhaustive verify FAILED ❌  (#fails {})", fails.len());
                println!("Showing up to 2000 misses:");
                for w in fails.iter().take(2000) { println!("  miss: {}", w); }

                // Add forced constraints for these failing base words (dedupe naturally at merge step)
                let mut newly_added_total = 0usize;
                for w in fails.iter() {
                    if let Some((cert, _deficit)) = hall_certificate_for_word(w, &built.letter_masks) {
                        if !cert.is_empty() {
                            let (_p, _t, h) = add_certificate_to_forced(&mut forced, &cert);
                            newly_added_total += h;
                        }
                    }
                }
                // Also add KGE subsets (k=4..KGE_MAXK, per-k topK) for each failing word (seeded to curseed)
                let (p0, t0, h0) = forced.counts();
                println!("[KGE] Seeding k-subsets (k=4..{}), for {} failing words…",
                         KGE_MAXK, fails.len());
                for w in &fails {
                    add_certificate_all_subsets_kge4(&mut forced, w, curseed as u64);
                }
                let (p1, t1, h1) = forced.counts();
                println!(
                    "[KGE] Added high-k keys from fails: +{} (pairs/triples unchanged: +{}, +{})",
                    h1.saturating_sub(h0), p1.saturating_sub(p0), t1.saturating_sub(t0)
                );

                // Group misses by Hall certificate with coverage summary
                {
                    let idx2 = build_index_from_masks(&built.letter_masks);

                    // key -> (componentwise max multiplicities vector aligned to key, word_count)
                    let mut groups: BTreeMap<Vec<usize>, (Vec<u8>, usize)> = BTreeMap::new();

                    for w in &fails {
                        if let Some((cert, _def)) = hall_certificate_for_word(w, &built.letter_masks) {
                            // Build multiplicity counts from the certificate itself (preserve multiplicity!)
                            let n = n_letters();
                            let mut cnt = vec![0u8; n];
                            for ch in cert.chars() {
                                if let Some(i) = char_index(ch) {
                                    cnt[i] = cnt[i].saturating_add(1);
                                }
                            }
                            // Sorted support (distinct letters)
                            let mut key: Vec<usize> = (0..n).filter(|&i| cnt[i] > 0).collect();
                            if key.is_empty() { continue; }
                            key.sort_unstable();

                            // Per-key multiplicity vector aligned to key
                            let v: Vec<u8> = key.iter().map(|&i| cnt[i]).collect();

                            let entry = groups.entry(key).or_insert((vec![0u8; v.len()], 0usize));
                            if entry.0.len() != v.len() {
                                entry.0 = vec![0u8; v.len()];
                            }
                            for j in 0..v.len() {
                                if v[j] > entry.0[j] { entry.0[j] = v[j]; }
                            }
                            entry.1 += 1;
                        }
                    }

                    // (key, need, have, gap, count)
                    let mut rows: Vec<(Vec<usize>, i32, i32, i32, usize)> = Vec::new();
                    for (key, (vmax, count)) in groups.iter() {
                        // Hall: need = |S| = sum of multiplicities; have = |N(S)| = union coverage over key
                        let need: i32 = vmax.iter().map(|&x| x as i32).sum();
                        let have: i32 = union_coverage_for_tuple(&idx2, key);
                        let gap = (need - have).max(0);
                        rows.push((key.clone(), need, have, gap, *count));
                    }
                    rows.sort_by(|a,b| b.3.cmp(&a.3).then_with(|| b.4.cmp(&a.4)));
                    println!("Grouped misses by certificate (top 12):");
                    for (key, need, have, gap, count) in rows.into_iter().take(12) {
                        let key_str: String = key.iter().map(|&i| index_char(i)).collect();
                        println!("  key={}  need={}  have={}  gap={}  ({} words)", key_str, need, have, gap, count);
                    }
                }

                println!("[Forced] Added ~{} new high-k Hall keys (from Hall certs); +{} from KGE.",
                         newly_added_total,
                         forced.rhv.len());
 

                // Warm-start from the last best layout next round
                warm_start = Some(built.clone());

                continue;
            }
        } else {
            // print summary
            let higher_summary: Vec<String> = dks.iter().filter(|(_, v)| **v > 0).map(|(k, v)| format!("R{}={}", k, v)).collect();
            println!(
                "Repair summary: R1={}  R2={}  R3={}{}",
                singles,
                pairs,
                triples,
                if higher_summary.is_empty() { "".into() } else { format!("  {}", higher_summary.join("  ")) }
            );

            // If there are deficits and we already have forced constraints, prefer re-anneal over trimming.
            if forced_high + forced_pairs + forced_triples > 0 {
                println!("[Forced] Constraints active → re-annealing (bump seed).");
                curseed = curseed.wrapping_add(1);
                continue;
            }

            // === Original interactive path (unchanged) when no forced constraints yet ===
            let cand: Vec<String> = constraint_blockers_parallel(
                &all_words,
                &idx,
                &stats,
                max_k,
                200,           // same cap as before
                &built.deg,    // singles quick check
            );

            let (action, removed, new_sweeps, new_mode) =
                interactive_trim_or_repair(&mut all_words, &cand, sweeps, cur_mode);
            sweeps = new_sweeps;
            cur_mode = new_mode;
            match action.as_str() {
                "remove" => { discarded.extend(removed); stats_dirty = true; println!("Recomputing with removed words…"); continue; }
                "sweeps" => { println!("Rebuilding with longer repairs (sweeps={})…", sweeps); continue; }
                "seed_bump" => { curseed += 1; println!("Rebuilding from scratch with seed={} …", curseed); continue; }
                "seed_set" => {
                    if !removed.is_empty() { curseed = removed[0].parse::<i64>().unwrap_or(curseed); }
                    println!("Rebuilding from scratch with seed={} …", curseed);
                    continue;
                }
                "mode_set" => { println!("Mode set to {:?}. Rebuilding…", cur_mode); continue; }
                _ => { println!("No change requested. Stopping interactive loop with constraints unsatisfied."); break; }
            }
        }
    }

    if !discarded.is_empty() {
        println!("=== Final list of discarded words ({}) ===", discarded.len());
        for w in discarded { println!("  - {}", w); }
    }

    Ok(())
}
