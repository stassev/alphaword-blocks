// AlphaWord Blocks — letter-to-block assignment
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

// src/assign.rs
use crate::types::*;
use crate::stats::{Stats, active_union2_pub, active_union3_pub, active_union_vecs_pub};
use rand::seq::SliceRandom; // Vec::choose(...)
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;

// ───────── new: rayon for parallel high-k and R4+ totals ─────────
use rayon::prelude::*;

/* ───────────────────────────── utilities ───────────────────────────── */

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic hash for a high-k key using a seed; yields a pseudo-random weight.
#[inline]
fn key_weight(seed: u64, key: &Vec<usize>) -> u64 {
    let mut h = splitmix64(seed ^ 0xA24B_AED4_963E_E407);
    for &v in key.iter() {
        h = splitmix64(h ^ (v as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    }
    h
}

#[inline]
fn letter_jitter(seed: u64, l: usize) -> u64 {
    splitmix64(seed ^ (l as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

/* ───────────────────────────── Tunables (edit here) ───────────────────────────── */

#[derive(Copy, Clone)]
struct RepairParams {
    /* mode knobs */
    highk_steps_per_sweep: usize, // how many high-k pushes each sweep (base)
    allow_elastic_singles: bool,  // allow temporary R1 dip by 1 when swapping
    dmg2_penalty: f64,            // penalty per risky pair removed in a swap
    dmg3_penalty: f64,            // penalty per risky triple removed in a swap
    base_free: f64,               // bias for free adds
    base_swap: f64,               // bias for swaps

    /* global knobs */
    highk_scan_budget: usize,     // how many high-k keys to scan per step
    kicks_early: usize,           // anneal proposals per kick when R1..R3 > 0
    kicks_late: usize,            // anneal proposals per kick when R1..R3 == 0
    kick_every_base: usize,       // kick cadence while R1..R3 > 0
    kick_every_late: usize,       // kick cadence while R1..R3 == 0
    plateau_inactive: usize,      // stop if no meaningful improvement for this many sweeps
    r4_stall_for_kick: usize,     // only kick if R4+ hasn't improved for this long
    t0: f64,                      // anneal temp (start of schedule)
    t_end: f64,                   // anneal temp (end of schedule)
    kick_accept_low: f64,         // adapt temp scale up if acceptance below this
    kick_accept_high: f64,        // adapt temp scale down if acceptance above this
    kick_scale_min: f64,          // clamp temp scale
    kick_scale_max: f64,
    kick_up_factor: f64,
    kick_down_factor: f64,
    singles_remove_penalty: f64,
}

fn params_for_mode(mode: RepairMode) -> RepairParams {
    match mode {
        RepairMode::HighFirst => RepairParams {
            highk_steps_per_sweep: 3,
            allow_elastic_singles: false,
            dmg2_penalty: 2.25,
            dmg3_penalty: 2.25 * 0.25,
            base_free: 28.0,
            base_swap: 18.0,

            highk_scan_budget: 10_000_000,
            kicks_early: 8,
            kicks_late: 4,
            kick_every_base: 50,
            kick_every_late: 100,
            plateau_inactive: 1_000_000,
            r4_stall_for_kick: 80,
            t0: 1.20,
            t_end: 0.05,
            kick_accept_low: 0.20,
            kick_accept_high: 0.70,
            kick_scale_min: 0.50,
            kick_scale_max: 1.50,
            kick_up_factor: 1.15,
            kick_down_factor: 0.9,
            singles_remove_penalty: 6.0,
        },
        RepairMode::Elastic => RepairParams {
            highk_steps_per_sweep: 8,
            allow_elastic_singles: true,
            dmg2_penalty: 0.5,
            dmg3_penalty: 0.5 * 0.25,
            base_free: 22.0,
            base_swap: 14.0,

            highk_scan_budget: 10_000_000,
            kicks_early: 8,
            kicks_late: 12,
            kick_every_base: 50,
            kick_every_late: 25,
            plateau_inactive: 1_000_000,
            r4_stall_for_kick: 30,
            t0: 1.20,
            t_end: 0.05,
            kick_accept_low: 0.20,
            kick_accept_high: 0.70,
            kick_scale_min: 0.50,
            kick_scale_max: 1.50,
            kick_up_factor: 1.15,
            kick_down_factor: 0.9,
            singles_remove_penalty: 6.0,
        },
        RepairMode::Balanced => RepairParams {
            highk_steps_per_sweep: 1,
            allow_elastic_singles: false,
            dmg2_penalty: 3.0,
            dmg3_penalty: 3.0 * 0.25,
            base_free: 20.0,
            base_swap: 12.0,

            highk_scan_budget: 10_000_000,
            kicks_early: 8,
            kicks_late: 4,
            kick_every_base: 50,
            kick_every_late: 100,
            plateau_inactive: 1_000_000,
            r4_stall_for_kick: 80,
            t0: 1.20,
            t_end: 0.05,
            kick_accept_low: 0.20,
            kick_accept_high: 0.70,
            kick_scale_min: 0.50,
            kick_scale_max: 1.50,
            kick_up_factor: 1.15,
            kick_down_factor: 0.9,
            singles_remove_penalty: 6.0,
        },
    }
}

pub fn fingerprint_masks(m: &[u64]) -> u64 {
    // small, stable, dependency-free fingerprint
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (i, &v) in m.iter().enumerate() {
        h = h.wrapping_mul(1099511628211);
        h ^= v.wrapping_mul(1469598103934665603u64 ^ ((i as u64 + 1) * 1099511628211));
    }
    h
}

/* ───────────────────────────── Debug cache check ───────────────────────────── */

#[allow(dead_code)]
fn assert_caches_consistent(b: &Built) {
    let n = n_letters();
    let idx = CubeIndex {
        letter_masks: b.letter_masks.clone(),
        cube_lists: vec![Vec::new(); n],
    };

    // union2: check only a < t, and enforce symmetry
    for a in 0..n {
        for t in (a + 1)..n {
            let have = coverage_for_tuple(&idx, &[a, t]); // OR coverage
            assert_eq!(b.union2[a][t], have, "union2 mismatch at ({},{})", a, t);
            assert_eq!(b.union2[t][a], have, "union2 symmetry mismatch at ({},{})", t, a);
        }
    }

    // union3 untouched (keys are a<b<c and you already compare to OR coverage)
    for a in 0..n {
        for b2 in (a + 1)..n {
            for c in (b2 + 1)..n {
                let have = coverage_for_tuple(&idx, &[a, b2, c]);
                assert_eq!(b.union3[a][b2][c], have, "union3 mismatch at ({},{},{})", a, b2, c);
            }
        }
    }
}

/* ───────────────────────────── Degree target ───────────────────────────── */

#[inline]
fn choose_target_degrees(r1: &[i32], f: &[i64], total_slots: usize) -> Vec<i32> {
    let n = n_letters();
    assert_eq!(r1.len(), n);
    assert_eq!(f.len(), n);
    let n_cubes = N_CUBES as i32;

    let mut w = vec![0f64; n];
    let mut wsum = 0.0;
    for i in 0..n {
        w[i] = (f[i] as f64 + 1e-6).sqrt();
        wsum += w[i];
    }
    let mut base = vec![0i32; n];
    for i in 0..n {
        base[i] = r1[i].max(((total_slots as f64) * (w[i] / wsum)).round() as i32).min(n_cubes);
    }
    let mut x = base.clone();
    let mut s: i32 = x.iter().sum();

    if s < total_slots as i32 {
        let mut deficit = vec![0i32; n];
        for i in 0..n {
            deficit[i] = (r1[i] - x[i]).max(0);
        }
        while s < total_slots as i32 {
            let mut i_best = 0usize;
            let mut best = f64::NEG_INFINITY;
            for i in 0..n {
                if x[i] < n_cubes {
                    let sc = 2.0 * (deficit[i] as f64) + w[i] / (x[i] as f64 + 1.0);
                    if sc > best {
                        best = sc;
                        i_best = i;
                    }
                }
            }
            x[i_best] += 1;
            s += 1;
            if deficit[i_best] > 0 {
                deficit[i_best] -= 1;
            }
        }
    } else if s > total_slots as i32 {
        while s > total_slots as i32 {
            let mut i_best = None::<usize>;
            let mut best = f64::INFINITY;
            for i in 0..n {
                if x[i] > r1[i] {
                    let sc = w[i] / (x[i] as f64);
                    if sc < best {
                        best = sc;
                        i_best = Some(i);
                    }
                }
            }
            if let Some(i_best) = i_best {
                if x[i_best] == r1[i_best] {
                    break;
                }
                x[i_best] -= 1;
                s -= 1;
            } else {
                break;
            }
        }
    }
    println!(
        "Target degrees (first few): {:?} ... total={}",
        (0..n.min(8))
            .map(|i| (index_char(i), x[i]))
            .collect::<Vec<_>>(),
        x.iter().sum::<i32>()
    );
    x
}

/* ───────────────────────────── Build state ───────────────────────────── */

#[derive(Clone)]
pub struct Built {
    pub cubes_present: Vec<Vec<bool>>,   // [N_CUBES][n]
    pub deg: Vec<i32>,                   // [n]
    pub letter_masks: Vec<u64>,          // [n]
    pub union2: Vec<Vec<i32>>,           // [n][n]
    pub union3: Vec<Vec<Vec<i32>>>,      // [n][n][n]
    pub cap: [usize; N_CUBES],           // capacity per cube, still fixed
}

impl Built {
    fn new() -> Self {
        let n = n_letters();
        Self {
            cubes_present: vec![vec![false; n]; N_CUBES],
            deg: vec![0; n],
            letter_masks: vec![0u64; n],
            union2: vec![vec![0; n]; n],
            union3: vec![vec![vec![0; n]; n]; n],
            cap: [FACES_PER_CUBE; N_CUBES],
        }
    }
}

#[inline]
fn add_letter(b: &mut Built, cube_i: usize, l: usize) {
    let n = n_letters();
    if b.cubes_present[cube_i][l] {
        return;
    }
    b.cubes_present[cube_i][l] = true;
    b.deg[l] += 1;
    b.letter_masks[l] |= 1u64 << cube_i;

    for t in 0..n {
        if t == l {
            continue;
        }
        if !b.cubes_present[cube_i][t] {
            b.union2[l][t] += 1;
            b.union2[t][l] += 1;
        }
    }
    for bb in 0..n {
        if bb == l || b.cubes_present[cube_i][bb] {
            continue;
        }
        for cc in (bb + 1)..n {
            if cc == l || b.cubes_present[cube_i][cc] {
                continue;
            }
            let (a1, a2, a3) = ord3(l, bb, cc);
            b.union3[a1][a2][a3] += 1;
        }
    }
}

#[inline]
fn remove_letter(b: &mut Built, cube_i: usize, r: usize) {
    let n = n_letters();
    if !b.cubes_present[cube_i][r] {
        return;
    }
    for t in 0..n {
        if t == r {
            continue;
        }
        if !b.cubes_present[cube_i][t] {
            b.union2[r][t] -= 1;
            b.union2[t][r] -= 1;
            debug_assert!(b.union2[r][t] >= 0);
            debug_assert!(b.union2[t][r] >= 0);
        }
    }
    for bb in 0..n {
        if bb == r || b.cubes_present[cube_i][bb] {
            continue;
        }
        for cc in (bb + 1)..n {
            if cc == r || b.cubes_present[cube_i][cc] {
                continue;
            }
            let (a1, a2, a3) = ord3(r, bb, cc);
            b.union3[a1][a2][a3] -= 1;
            debug_assert!(b.union3[a1][a2][a3] >= 0);
        }
    }
    b.cubes_present[cube_i][r] = false;
    b.deg[r] -= 1;
    b.letter_masks[r] &= !(1u64 << cube_i);
}

/* ───────────────────── Scoring & diagnostics helpers ───────────────────── */

#[inline]
fn score_letter_on_cube(i: usize, l: usize, x: &[i32], b: &Built, s: &Stats) -> f64 {
    let n = n_letters();
    if b.cap[i] == 0 || b.cubes_present[i][l] {
        return -1.0e12;
    }
    let deg = &b.deg;

    // Singles deficit
    let g1 = (s.r1[l] - deg[l]).max(0) as f64;

    // Pair deficits this would push (only if cube currently lacks t)
    let mut g2_sum = 0.0f64;
    for t in 0..n {
        if t == l || b.cubes_present[i][t] {
            continue;
        }
        let need = active_union2_pub(&s.r2v, l, t, deg);
        let have = b.union2[l][t];
        let def = (need - have).max(0) as f64;
        if def > 0.0 {
            g2_sum += def;
        }
    }

    // Triple deficits this would push (only if cube currently lacks bb,cc)
    let mut g3_sum = 0.0f64;
    for bb in 0..n {
        if bb == l || b.cubes_present[i][bb] {
            continue;
        }
        for cc in (bb + 1)..n {
            if cc == l || b.cubes_present[i][cc] {
                continue;
            }
            let need = active_union3_pub(&s.r3v, l, bb, cc, deg);
            let (a1, a2, a3) = ord3(l, bb, cc);
            let have = b.union3[a1][a2][a3];
            let def = (need - have).max(0) as f64;
            if def > 0.0 {
                g3_sum += def;
            }
        }
    }

    // Pressure against overshooting target
    let over = (deg[l] - x[l]) as f64;
    let bias = if over >= 0.0 { -2.0 * (over + 1.0) } else { 2.2 * (-over) };

    8.0 * g1 + 0.75 * g2_sum + 0.10 * g3_sum + bias
}

/* ───────────────────── Deficit measurement (R1/R2/R3) ───────────────────── */

#[inline]
fn deficits_r1_r2_r3(b: &Built, s: &Stats) -> (i32, i32, i32) {
    let n = n_letters();

    // R1 singles
    let r1: i32 = (0..n).map(|i| (s.r1[i] - b.deg[i]).max(0)).sum();

    // R2
    let mut r2 = 0i32;
    for ((a, b2), _vecs) in s.r2v.iter() {
        let need = active_union2_pub(&s.r2v, *a, *b2, &b.deg);
        let have = b.union2[*a][*b2];
        if need > have {
            r2 += need - have;
        }
    }

    // R3
    let mut r3 = 0i32;
    for ((a, b2, c), _vecs) in s.r3v.iter() {
        let need = active_union3_pub(&s.r3v, *a, *b2, *c, &b.deg);
        let (x1, x2, x3) = (*a, *b2, *c);
        let have = b.union3[x1][x2][x3];
        if need > have {
            r3 += need - have;
        }
    }

    (r1, r2, r3)
}

/* ───────────────────── Seeded high-k termination check ───────────────────── */

#[inline]
fn any_highk_deficit_exists_seeded(b: &Built, s: &Stats, seed: i64) -> bool {
    let n = n_letters();
    let idx = CubeIndex {
        letter_masks: b.letter_masks.clone(),
        cube_lists: vec![Vec::<usize>::new(); n],
    };
    let mut keys: Vec<&Vec<usize>> = s.rhv.keys().collect();
    keys.sort_unstable_by_key(|k| key_weight(seed as u64, *k));
    for key in keys {
        if key.len() < 4 {
            continue;
        }
        if let Some(vecs) = s.rhv.get(key) {
            let need = active_union_vecs_pub(vecs, key, &b.deg);
            if need == 0 {
                continue;
            }
            let have = crate::types::coverage_for_tuple(&idx, key);
            if have < need {
                return true;
            }
        }
    }
    false
}

/* ───────────────── shared: singles + capacity R2/R3 (with tie-break tweaks) ───────────────── */

fn singles_and_capacity_topups_shared(
    b: &mut Built,
    s: &Stats,
    rp: &RepairParams,
    x: &[i32],
    r2_keys_sorted: &[(usize, usize)],
    r3_keys_sorted: &[(usize, usize, usize)],
) {
    let n = n_letters();
    let elastic_slack: i32 = 1;

    // --- Singles fixups (graded swap to raise deg[l] up to r1[l]) ---
    for l in 0..n {
        let mut need_single = s.r1[l] - b.deg[l];
        while need_single > 0 {
            let mut best_i = None::<usize>;
            let mut best_r = 0usize;
            let mut best_gain = f64::NEG_INFINITY;

            for i in 0..N_CUBES {
                if b.cubes_present[i][l] {
                    continue;
                }
                for r in 0..n {
                    if !b.cubes_present[i][r] {
                        continue;
                    }

                    // removal guard: strict vs elastic
                    let ok_remove = if !rp.allow_elastic_singles {
                        b.deg[r] > s.r1[r]
                    } else {
                        b.deg[r] > s.r1[r] - elastic_slack
                    };
                    if !ok_remove {
                        continue;
                    }

                    // removal damages
                    let mut dmg2 = 0.0f64;
                    let mut dmg3 = 0.0f64;
                    for t in 0..n {
                        if t == r || b.cubes_present[i][t] {
                            continue;
                        }
                        let need = active_union2_pub(&s.r2v, r, t, &b.deg);
                        if b.union2[r][t] <= need {
                            dmg2 += 1.0;
                        }
                    }
                    for bb in 0..n {
                        if bb == r || b.cubes_present[i][bb] {
                            continue;
                        }
                        for cc in (bb + 1)..n {
                            if cc == r || b.cubes_present[i][cc] {
                                continue;
                            }
                            let need = active_union3_pub(&s.r3v, r, bb, cc, &b.deg);
                            let (a1, a2, a3) = ord3(r, bb, cc);
                            if b.union3[a1][a2][a3] <= need {
                                dmg3 += 1.0;
                            }
                        }
                    }

                    // addition gains (graded)
                    let mut add2 = 0.0f64;
                    let mut add3 = 0.0f64;
                    for t in 0..n {
                        if t == l || b.cubes_present[i][t] {
                            continue;
                        }
                        let need = active_union2_pub(&s.r2v, l, t, &b.deg);
                        let have = b.union2[l][t];
                        if have < need {
                            add2 += (need - have) as f64;
                        }
                    }
                    for bb in 0..n {
                        if bb == l || b.cubes_present[i][bb] {
                            continue;
                        }
                        for cc in (bb + 1)..n {
                            if cc == l || b.cubes_present[i][cc] {
                                continue;
                            }
                            let need = active_union3_pub(&s.r3v, l, bb, cc, &b.deg);
                            let (a1, a2, a3) = ord3(l, bb, cc);
                            let have = b.union3[a1][a2][a3];
                            if have < need {
                                add3 += (need - have) as f64;
                            }
                        }
                    }

                    let sc = 10.0 * (need_single as f64)
                        + 0.75 * add2
                        + 0.10 * add3
                        - (rp.dmg2_penalty * dmg2 + 3.0 * rp.dmg3_penalty * dmg3);
                    if sc > best_gain {
                        best_gain = sc;
                        best_i = Some(i);
                        best_r = r;
                    }
                }
            }

            if let Some(i) = best_i {
                remove_letter(b, i, best_r);
                add_letter(b, i, l);
                need_single -= 1;
            } else {
                break;
            }
        }
    }

    // --- R2 capacity-only top-ups (with micro tie-break) ---
    for &(a_i, b_i) in r2_keys_sorted {
        loop {
            let need = active_union2_pub(&s.r2v, a_i, b_i, &b.deg);
            if b.union2[a_i][b_i] >= need {
                break;
            }
            let mut placed_here = false;
            for i in 0..N_CUBES {
                if b.cap[i] > 0 && !(b.cubes_present[i][a_i] || b.cubes_present[i][b_i]) {
                    let da = s.r1[a_i] - b.deg[a_i];
                    let db = s.r1[b_i] - b.deg[b_i];
                    let target = if da > db {
                        a_i
                    } else if db > da {
                        b_i
                    } else {
                        let oa = b.deg[a_i] - x[a_i];
                        let ob = b.deg[b_i] - x[b_i];
                        if oa < ob {
                            a_i
                        } else if ob < oa {
                            b_i
                        } else {
                            a_i.min(b_i)
                        }
                    };
                    add_letter(b, i, target);
                    b.cap[i] -= 1;
                    placed_here = true;
                    break;
                }
            }
            if !placed_here {
                break;
            }
        }
    }

    // --- R3 capacity-only top-ups (with micro tie-break) ---
    for &(a_i, b_i, c_i) in r3_keys_sorted {
        let (x1, x2, x3) = (a_i, b_i, c_i);
        loop {
            let need = active_union3_pub(&s.r3v, x1, x2, x3, &b.deg);
            if b.union3[x1][x2][x3] >= need {
                break;
            }
            let mut placed_here = false;
            for i in 0..N_CUBES {
                if b.cap[i] > 0 && !(b.cubes_present[i][x1] || b.cubes_present[i][x2] || b.cubes_present[i][x3]) {
                    let mut cand = [
                        (x1, s.r1[x1] - b.deg[x1]),
                        (x2, s.r1[x2] - b.deg[x2]),
                        (x3, s.r1[x3] - b.deg[x3]),
                    ];
                    cand.sort_unstable_by(|&(li, di), &(lj, dj)| {
                        dj.cmp(&di)
                            .then_with(|| {
                                let oi = b.deg[li] - x[li];
                                let oj = b.deg[lj] - x[lj];
                                oi.cmp(&oj)
                            })
                            .then_with(|| li.cmp(&lj))
                    });
                    add_letter(b, i, cand[0].0);
                    b.cap[i] -= 1;
                    placed_here = true;
                    break;
                }
            }
            if !placed_here {
                break;
            }
        }
    }
}

/* ─────────────── Annealing jerk (temperature “noise”) ─────────────── */

#[inline]
fn anneal_accept(sc: f64, temp: f64, rng: &mut ChaCha20Rng) -> bool {
    if sc >= 0.0 {
        return true;
    }
    let p = (sc / temp.max(1e-6)).exp();
    rng.gen::<f64>() < p
}

fn temp_jerk(b: &mut Built, s: &Stats, rp: &RepairParams, rng: &mut ChaCha20Rng, temp: f64, x: &[i32]) -> bool {
    let n = n_letters();
    let i = rng.gen_range(0..N_CUBES);
    let elastic_slack: i32 = 1;

    if b.cap[i] > 0 {
        let mut best_l = None::<usize>;
        let mut best_sc = f64::NEG_INFINITY;
        for l in 0..n {
            if b.cubes_present[i][l] {
                continue;
            }
            let sc = rp.base_free + score_letter_on_cube(i, l, x, b, s);
            if sc > best_sc {
                best_sc = sc;
                best_l = Some(l);
            }
        }
        if let Some(l) = best_l {
            if anneal_accept(best_sc, temp, rng) {
                add_letter(b, i, l);
                b.cap[i] -= 1;
                return true;
            }
        }
        return false;
    }

    let mut choices_r = Vec::new();
    for r in 0..n {
        if !b.cubes_present[i][r] {
            continue;
        }
        if rp.allow_elastic_singles {
            if b.deg[r] <= s.r1[r] - elastic_slack {
                continue;
            }
        } else if b.deg[r] <= s.r1[r] {
            continue;
        }
        choices_r.push(r);
    }
    if choices_r.is_empty() {
        return false;
    }

    let r = *choices_r.choose(rng).unwrap();

    let mut best_pair = None::<(usize, f64)>;
    for l in 0..n {
        if b.cubes_present[i][l] {
            continue;
        }

        let mut dmg2 = 0.0f64;
        let mut dmg3 = 0.0f64;
        for t in 0..n {
            if t == r || b.cubes_present[i][t] {
                continue;
            }
            let need = active_union2_pub(&s.r2v, r, t, &b.deg);
            if b.union2[r][t] <= need {
                dmg2 += 1.0;
            }
        }
        for bb in 0..n {
            if bb == r || b.cubes_present[i][bb] {
                continue;
            }
            for cc in (bb + 1)..n {
                if cc == r || b.cubes_present[i][cc] {
                    continue;
                }
                let need = active_union3_pub(&s.r3v, r, bb, cc, &b.deg);
                let (a1, a2, a3) = ord3(r, bb, cc);
                if b.union3[a1][a2][a3] <= need {
                    dmg3 += 1.0;
                }
            }
        }

        let mut add2 = 0.0f64;
        let mut add3 = 0.0f64;
        for t in 0..n {
            if t == l || b.cubes_present[i][t] {
                continue;
            }
            let need = active_union2_pub(&s.r2v, l, t, &b.deg);
            let have = b.union2[l][t];
            if have < need {
                add2 += (need - have) as f64;
            }
        }
        for bb in 0..n {
            if bb == l || b.cubes_present[i][bb] {
                continue;
            }
            for cc in (bb + 1)..n {
                if cc == l || b.cubes_present[i][cc] {
                    continue;
                }
                let need = active_union3_pub(&s.r3v, l, bb, cc, &b.deg);
                let (a1, a2, a3) = ord3(l, bb, cc);
                let have = b.union3[a1][a2][a3];
                if have < need {
                    add3 += (need - have) as f64;
                }
            }
        }

        let singles_bias = if b.deg[r] <= s.r1[r] { -rp.singles_remove_penalty } else { 0.0 };
        let sc = rp.base_swap + singles_bias + 0.75 * add2 + 0.10 * add3
            - (rp.dmg2_penalty * dmg2 + 3.0 * rp.dmg3_penalty * dmg3);
        if best_pair.map_or(true, |(_, bs)| sc > bs) {
            best_pair = Some((l, sc));
        }
    }

    if let Some((l, sc)) = best_pair {
        if anneal_accept(sc, temp, rng) {
            remove_letter(b, i, r);
            add_letter(b, i, l);
            return true;
        }
    }
    false
}

/* ────────────────────── High-k step (fixed semantics) ───────────────────── */

fn highk_step(
    b: &mut Built,
    s: &Stats,
    rp: &RepairParams,
    rhv_keys_sorted: &[&Vec<usize>],
) -> bool {
    let n = n_letters();
    let tmp_idx = CubeIndex {
        letter_masks: b.letter_masks.clone(),
        cube_lists: vec![Vec::<usize>::new(); n],
    };

    let scan_len = rp.highk_scan_budget.min(rhv_keys_sorted.len());
    let (best_gap, best_idx) = (0..scan_len)
        .into_par_iter()
        .map(|i| {
            let key = rhv_keys_sorted[i];
            if key.len() < 4 {
                return (0i32, usize::MAX);
            }
            if let Some(vecs) = s.rhv.get(key) {
                let need = crate::stats::active_union_vecs_pub(vecs, key, &b.deg);
                if need == 0 {
                    return (0i32, usize::MAX);
                }
                let have = crate::types::coverage_for_tuple(&tmp_idx, key);
                let gap = (need - have).max(0);
                if gap > 0 {
                    (gap, i)
                } else {
                    (0, usize::MAX)
                }
            } else {
                (0i32, usize::MAX)
            }
        })
        .reduce(|| (0i32, usize::MAX), |a, b| {
            if a.0 > b.0 {
                a
            } else if b.0 > a.0 {
                b
            } else if a.1 < b.1 {
                a
            } else {
                b
            }
        });

    if best_gap <= 0 || best_idx == usize::MAX {
        return false;
    }

    let key = rhv_keys_sorted[best_idx].clone();

    // choose letter to add among `key`: least current degree
    let mut best_l = key[0];
    let mut bestd = i32::MAX;
    for &l in key.iter() {
        if b.deg[l] < bestd {
            bestd = b.deg[l];
            best_l = l;
        }
    }

    let mut chosen_i = None::<usize>;
    let mut chosen_r = None::<usize>;
    let mut best_score = f64::NEG_INFINITY;
    let elastic_slack: i32 = 1;

    for i in 0..N_CUBES {
        if key.iter().any(|&kk| b.cubes_present[i][kk]) {
            continue;
        }

        if b.cap[i] > 0 {
            // free placement: graded gains
            let mut add2 = 0.0f64;
            let mut add3 = 0.0f64;
            for t in 0..n {
                if t == best_l || b.cubes_present[i][t] {
                    continue;
                }
                let need = active_union2_pub(&s.r2v, best_l, t, &b.deg);
                let have = b.union2[best_l][t];
                if have < need {
                    add2 += (need - have) as f64;
                }
            }
            for bb in 0..n {
                if bb == best_l || b.cubes_present[i][bb] {
                    continue;
                }
                for cc in (bb + 1)..n {
                    if cc == best_l || b.cubes_present[i][cc] {
                        continue;
                    }
                    let need = active_union3_pub(&s.r3v, best_l, bb, cc, &b.deg);
                    let (a1, a2, a3) = ord3(best_l, bb, cc);
                    let have = b.union3[a1][a2][a3];
                    if have < need {
                        add3 += (need - have) as f64;
                    }
                }
            }
            let sc = rp.base_free + 0.75 * add2 + 0.10 * add3;
            if sc > best_score {
                best_score = sc;
                chosen_i = Some(i);
                chosen_r = None;
            }
        } else {
            // swap
            for r in 0..n {
                if !b.cubes_present[i][r] {
                    continue;
                }

                if !rp.allow_elastic_singles {
                    if b.deg[r] <= s.r1[r] {
                        continue;
                    }
                } else if b.deg[r] <= s.r1[r] - elastic_slack {
                    continue;
                }

                let mut dmg2 = 0.0f64;
                let mut dmg3 = 0.0f64;
                for t in 0..n {
                    if t == r || b.cubes_present[i][t] {
                        continue;
                    }
                    let need = active_union2_pub(&s.r2v, r, t, &b.deg);
                    if b.union2[r][t] <= need {
                        dmg2 += 1.0;
                    }
                }
                for bb in 0..n {
                    if bb == r || b.cubes_present[i][bb] {
                        continue;
                    }
                    for cc in (bb + 1)..n {
                        if cc == r || b.cubes_present[i][cc] {
                            continue;
                        }
                        let need = active_union3_pub(&s.r3v, r, bb, cc, &b.deg);
                        let (a1, a2, a3) = ord3(r, bb, cc);
                        if b.union3[a1][a2][a3] <= need {
                            dmg3 += 1.0;
                        }
                    }
                }

                let mut add2 = 0.0f64;
                let mut add3 = 0.0f64;
                for t in 0..n {
                    if t == best_l || b.cubes_present[i][t] {
                        continue;
                    }
                    let need = active_union2_pub(&s.r2v, best_l, t, &b.deg);
                    let have = b.union2[best_l][t];
                    if have < need {
                        add2 += (need - have) as f64;
                    }
                }
                for bb in 0..n {
                    if bb == best_l || b.cubes_present[i][bb] {
                        continue;
                    }
                    for cc in (bb + 1)..n {
                        if cc == best_l || b.cubes_present[i][cc] {
                            continue;
                        }
                        let need = active_union3_pub(&s.r3v, best_l, bb, cc, &b.deg);
                        let (a1, a2, a3) = ord3(best_l, bb, cc);
                        let have = b.union3[a1][a2][a3];
                        if have < need {
                            add3 += (need - have) as f64;
                        }
                    }
                }

                let sc = rp.base_swap + 0.75 * add2 + 0.10 * add3
                    - (rp.dmg2_penalty * dmg2 + 3.0 * rp.dmg3_penalty * dmg3);
                if sc > best_score {
                    best_score = sc;
                    chosen_i = Some(i);
                    chosen_r = Some(r);
                }
            }
        }
    }

    if let Some(i) = chosen_i {
        if let Some(r) = chosen_r {
            remove_letter(b, i, r);
        }
        add_letter(b, i, best_l);
        if chosen_r.is_none() {
            b.cap[i] -= 1;
        }
        return true;
    }
    false
}

/* ───────────────────────────── Core helpers ───────────────────────────── */

#[inline]
fn count_highk_keys(s: &Stats) -> usize {
    s.rhv.keys().filter(|k| k.len() >= 4).count()
}

#[inline]
fn r4plus_deficit_total(b: &Built, s: &Stats) -> i32 {
    let n = n_letters();
    let idx = CubeIndex {
        letter_masks: b.letter_masks.clone(),
        cube_lists: vec![Vec::<usize>::new(); n],
    };

    let entries: Vec<(&Vec<usize>, &Vec<Vec<u8>>)> = s.rhv.iter().collect();

    entries
        .par_iter()
        .map(|(key, vecs)| {
            if key.len() < 4 {
                return 0i32;
            }
            let need = active_union_vecs_pub(vecs, key, &b.deg);
            if need == 0 {
                return 0i32;
            }
            let have = crate::types::coverage_for_tuple(&idx, key);
            (need - have).max(0)
        })
        .sum()
}

fn greedy_initial_fill(b: &mut Built, s: &Stats, x: &[i32], seed: i64, verbose: bool) {
    let n = n_letters();
    let mut rng = ChaCha20Rng::seed_from_u64(seed as u64);

    let total_to_place = TOTAL_SLOTS;
    let mut placed = 0usize;
    println!("Greedy placement: placing {} faces…", total_to_place);
    let print_every = N_CUBES.max(1);
    while placed < total_to_place {
        let mut i = (0..N_CUBES).max_by_key(|&i| b.cap[i]).unwrap();
        if b.cap[i] == 0 {
            let idxs: Vec<usize> = (0..N_CUBES).filter(|&j| b.cap[j] > 0).collect();
            if idxs.is_empty() {
                break;
            }
            i = idxs[rng.gen_range(0..idxs.len())];
        }
        let mut best_l = None::<usize>;
        let mut best_sc = f64::NEG_INFINITY;
        let mut best_jit = 0u64;
        for l in 0..n {
            if b.cubes_present[i][l] {
                continue;
            }
            let sc = score_letter_on_cube(i, l, x, b, s);
            let jit = letter_jitter(seed as u64, l);
            if sc > best_sc + 1e-12 || ((sc - best_sc).abs() <= 1e-12 && jit > best_jit) {
                best_sc = sc;
                best_l = Some(l);
                best_jit = jit;
            }
        }
        let chosen = if let Some(l) = best_l {
            l
        } else {
            let cand: Vec<usize> = (0..n).filter(|&l| !b.cubes_present[i][l]).collect();
            if cand.is_empty() {
                break;
            }
            cand[rng.gen_range(0..cand.len())]
        };
        add_letter(b, i, chosen);
        b.cap[i] -= 1;
        placed += 1;
        if placed % print_every == 0 && verbose {
            let pct = 100.0 * (placed as f64) / (total_to_place as f64);
            println!("Greedy: placed {}/{} ({:.1}%)", placed, total_to_place, pct);
        }
    }
    println!("Greedy placement: done.");
}

#[inline]
fn pair_weight(seed: u64, a: usize, b: usize) -> u64 {
    let mut h = splitmix64(seed ^ 0xD2B7_4407_B1CE_6E93);
    h = splitmix64(h ^ (a as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    h = splitmix64(h ^ (b as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9));
    h
}

#[inline]
fn triple_weight(seed: u64, a: usize, b: usize, c: usize) -> u64 {
    let mut h = splitmix64(seed ^ 0x94D0_49BB_1331_11EB);
    h = splitmix64(h ^ (a as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    h = splitmix64(h ^ (b as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9));
    h = splitmix64(h ^ (c as u64).wrapping_mul(0x94D0_49BB_1331_11EB));
    h
}

fn run_anneal(
    mut b: Built,
    s: &Stats,
    sweeps: usize,
    seed: i64,
    verbose: bool,
    rp_mode: RepairParams,
) -> Built {
    let r2_keys_sorted: Vec<(usize, usize)> = {
        let mut v: Vec<_> = s.r2v.keys().cloned().collect();
        v.sort_by(|&(a1, b1), &(a2, b2)| {
            pair_weight(seed as u64, a1, b1)
                .cmp(&pair_weight(seed as u64, a2, b2))
                .then_with(|| a1.cmp(&a2))
                .then_with(|| b1.cmp(&b2))
        });
        v
    };
    let r3_keys_sorted: Vec<(usize, usize, usize)> = {
        let mut v: Vec<_> = s.r3v.keys().cloned().collect();
        v.sort_by(|&(a1, b1, c1), &(a2, b2, c2)| {
            triple_weight(seed as u64, a1, b1, c1)
                .cmp(&triple_weight(seed as u64, a2, b2, c2))
                .then_with(|| a1.cmp(&a2))
                .then_with(|| b1.cmp(&b2))
                .then_with(|| c1.cmp(&c2))
        });
        v
    };
    let mut rhv_keys_sorted: Vec<&Vec<usize>> = s.rhv.keys().collect();
    rhv_keys_sorted.sort_by(|ka, kb| {
        key_weight(seed as u64, ka)
            .cmp(&key_weight(seed as u64, kb))
            .then_with(|| ka.cmp(kb))
    });

    if verbose {
        assert_caches_consistent(&b);
        println!("cache ok before anneal");
        let fp = fingerprint_masks(&b.letter_masks);
        println!("fingerprint={:016x}", fp);

        let r1_sum: i32 = s.r1.iter().sum();
        let r2c = s.r2v.len();
        let r3c = s.r3v.len();
        let r4c = count_highk_keys(s);
        if s.rhv_forced_count > 0 {
            println!(
                "Stats view for build: r1_sum={} r2_keys={} r3_keys={} r4+_keys={} (forced added: {})",
                r1_sum, r2c, r3c, r4c, s.rhv_forced_count
            );
        } else {
            println!(
                "Stats view for build: r1_sum={} r2_keys={} r3_keys={} r4+_keys={}",
                r1_sum, r2c, r3c, r4c
            );
        }
        if r4c == 0 {
            println!("NOTE: no k≥4 keys visible in Stats.rhv; if you *expected* Hall certs, they are not merged into Stats.");
        }
    }

    let x = choose_target_degrees(&s.r1, &s.f, TOTAL_SLOTS);

    let (mut r1, mut r2, mut r3) = deficits_r1_r2_r3(&b, s);
    let mut best_r4plus: i32 = r4plus_deficit_total(&b, s);
    let mut best_pair = (r1 + r2 + r3, best_r4plus);
    let mut best_b = b.clone();

    let mut rng = ChaCha20Rng::seed_from_u64(seed as u64);
    let mut kick_temp_scale = 1.0_f64;
    let mut accepted_kicks = 0usize;
    let mut last_r4_improve_sweep = 0usize;
    let mut last_activity_sweep = 0usize;

    if verbose {
        println!("Start: R1={} R2={} R3={}  R4+={}", r1, r2, r3, best_r4plus);
        println!("Repair sweeps: up to {}…", sweeps);
    }

    for sidx in 1..=sweeps {
        if verbose && (sidx % 200 == 0) {
            assert_caches_consistent(&b);
            println!("cache ok @ sweep {}", sidx);
        }

        let r123 = r1 + r2 + r3;
        let mut rp_now = rp_mode;
        if r123 == 0 {
            rp_now.allow_elastic_singles = true;
            rp_now.highk_steps_per_sweep = 8;
            rp_now.highk_scan_budget = 10_000_000;
            rp_now.dmg2_penalty *= 0.25;
            rp_now.dmg3_penalty *= 0.25;
            rp_now.kicks_late = 12;
            rp_now.kick_every_late = 25;
        }

        let hk_boost = if r123 <= 3 { 8 } else if r123 <= 8 { 2 } else { 1 };
        let hk_steps_base = rp_now.highk_steps_per_sweep + if r123 == 0 { 1 } else { 0 };
        let run_highk_first = rp_now.allow_elastic_singles;

        let do_highk = |b_: &mut Built| {
            for _ in 0..(hk_steps_base * hk_boost) {
                highk_step(b_, s, &rp_now, &rhv_keys_sorted);
            }
        };
        let do_singles_and_capacity_topups = |b_: &mut Built| {
            singles_and_capacity_topups_shared(b_, s, &rp_now, &x, &r2_keys_sorted, &r3_keys_sorted);
        };

        if run_highk_first {
            do_highk(&mut b);
        }
        do_singles_and_capacity_topups(&mut b);
        if !run_highk_first {
            do_highk(&mut b);
        }

        let (r1n, r2n, r3n) = deficits_r1_r2_r3(&b, s);
        let score_total = r1n + r2n + r3n;
        let r4plus_now = r4plus_deficit_total(&b, s);

        let current_pair = (score_total, r4plus_now);
        if current_pair < best_pair {
            best_pair = current_pair;
            best_b = b.clone();
            if verbose {
                println!(
                    "Best so far @ {}: R1..R3={}  R4+={}  fp={:016x}",
                    sidx,
                    score_total,
                    r4plus_now,
                    fingerprint_masks(&best_b.letter_masks)
                );
            }
        }

        let improved_r123 = (r1 + r2 + r3) > (r1n + r2n + r3n);
        let improved_r4 = r4plus_now < best_r4plus;
        if improved_r4 {
            best_r4plus = r4plus_now;
            last_r4_improve_sweep = sidx;
        }
        if improved_r123 || improved_r4 {
            last_activity_sweep = sidx;
        }

        let kick_every_now = if r1 + r2 + r3 == 0 {
            rp_now.kick_every_late
        } else {
            rp_now.kick_every_base
        };
        if sidx % kick_every_now == 0 && sidx.saturating_sub(last_r4_improve_sweep) >= rp_now.r4_stall_for_kick {
            let t_base = rp_now.t0 + (rp_now.t_end - rp_now.t0) * (sidx as f64 / sweeps.max(1) as f64);
            let prev_scale = kick_temp_scale;
            let t_in = t_base * prev_scale;
            let kicks_per_now = if r1 + r2 + r3 == 0 { rp_now.kicks_late } else { rp_now.kicks_early };
            let mut accepted = 0usize;
            for _ in 0..kicks_per_now {
                if temp_jerk(&mut b, s, &rp_now, &mut rng, t_in, &x) {
                    accepted += 1;
                }
            }
            accepted_kicks += accepted;

            let acc_ratio = accepted as f64 / (kicks_per_now as f64);
            if acc_ratio < rp_now.kick_accept_low {
                kick_temp_scale *= rp_now.kick_up_factor;
            } else if acc_ratio > rp_now.kick_accept_high {
                kick_temp_scale *= rp_now.kick_down_factor;
            } else {
                kick_temp_scale = 0.9 * kick_temp_scale + 0.1 * 1.0;
            }
            if kick_temp_scale < rp_now.kick_scale_min {
                kick_temp_scale = rp_now.kick_scale_min;
            }
            if kick_temp_scale > rp_now.kick_scale_max {
                kick_temp_scale = rp_now.kick_scale_max;
            }

            if verbose {
                let t_next = t_base * kick_temp_scale;
                println!(
                    "Kick@{}: T_in={:.3} → T_next={:.3} (scale x{:.2}→x{:.2}) accepted {}/{}  acc={:.2}",
                    sidx, t_in, t_next, prev_scale, kick_temp_scale, accepted, kicks_per_now, acc_ratio
                );
            }
        }

        if verbose && (sidx % 10 == 0) {
            println!(
                "Sweep {:5}: R1={} (Δ{})  R2={} (Δ{})  R3={} (Δ{})  R4+={}",
                sidx,
                r1n,
                r1n - r1,
                r2n,
                r2n - r2,
                r3n,
                r3n - r3,
                r4plus_now
            );
        }

        if r1n == 0 && r2n == 0 && r3n == 0 {
            if !any_highk_deficit_exists_seeded(&b, s, seed) {
                println!(
                    "All constraints R1..R3 satisfied; no higher-k (k≥4) deficits remain ✅ at sweep {}.",
                    sidx
                );
                break;
            }
        }

        if sidx.saturating_sub(last_activity_sweep) > rp_now.plateau_inactive {
            println!(
                "Plateau detected: no improvement (R1..R3 or R4+) in {} sweeps (R1..R3 now: {}+{}+{}, best R4+: {}). Stopping.",
                rp_now.plateau_inactive, r1n, r2n, r3n, best_r4plus
            );
            break;
        }

        r1 = r1n;
        r2 = r2n;
        r3 = r3n;
    }

    if verbose {
        println!(
            "Repair summary (R1..R3): singles={} | accepted_kicks={}",
            (0..n_letters()).map(|i| (s.r1[i] - best_b.deg[i]).max(0)).sum::<i32>(),
            accepted_kicks
        );
        assert_caches_consistent(&best_b);
        println!("cache ok on best_b");
        let fp = fingerprint_masks(&best_b.letter_masks);
        println!("fingerprint={:016x}", fp);
    }

    best_b
}

/* ───────────────────── Public entry points (unified behavior) ───────────────────── */

pub fn assign_letters_once_mode(
    _words: &[String],
    stats: &Stats,
    sweeps: usize,
    seed: i64,
    verbose: bool,
    mode: RepairMode,
) -> Built {
    let rp_mode = params_for_mode(mode);

    if verbose {
        let r4c = count_highk_keys(stats);
        if stats.rhv_forced_count > 0 {
            println!(
                "High-k visibility: {} k≥4 keys in Stats.rhv (forced added: {})",
                r4c, stats.rhv_forced_count
            );
        } else {
            println!("High-k visibility: {} k≥4 keys in Stats.rhv", r4c);
        }
        if r4c == 0 {
            println!("WARNING: No high-k keys visible. If Hall certs were added, they were NOT merged into Stats for this build.");
        }
    }

    let mut b = Built::new();

    let x = choose_target_degrees(&stats.r1, &stats.f, TOTAL_SLOTS);
    greedy_initial_fill(&mut b, stats, &x, seed, verbose);

    run_anneal(b, stats, sweeps, seed, verbose, rp_mode)
}

pub fn assign_letters_reanneal_from_mode(
    stats: &Stats,
    sweeps: usize,
    seed: i64,
    verbose: bool,
    mode: RepairMode,
    start: &Built,
) -> Built {
    let rp_mode = params_for_mode(mode);

    if verbose {
        let r4c = count_highk_keys(stats);
        if stats.rhv_forced_count > 0 {
            println!(
                "High-k visibility (re-anneal): {} k≥4 keys in Stats.rhv (forced added: {})",
                r4c, stats.rhv_forced_count
            );
        } else {
            println!("High-k visibility (re-anneal): {} k≥4 keys in Stats.rhv", r4c);
        }
        if r4c == 0 {
            println!("WARNING: No high-k keys visible for re-anneal. If Hall certs were added, they were NOT merged into Stats.");
        }
    }

    let b = start.clone();

    run_anneal(b, stats, sweeps, seed, verbose, rp_mode)
}

/* ───────────────────── Printing utils (runtime alphabet) ───────────────────── */

pub fn print_cubes_human(b: &Built) {
    let n = n_letters();
    println!("=== Cubes (human) ===");
    for i in 0..N_CUBES {
        let mut face: Vec<char> = Vec::new();
        for l in 0..n {
            if b.cubes_present[i][l] {
                face.push(index_char(l));
            }
        }
        face.sort_unstable();
        let mut s = String::new();
        for (k, ch) in face.iter().enumerate() {
            if k > 0 {
                s.push(' ');
            }
            s.push(*ch);
        }
        println!("Cube {:02}: {}", i + 1, s);
    }
}

pub fn cubes_as_code_string(b: &Built) -> String {
    let n = n_letters();
    let mut parts = Vec::new();
    for i in 0..N_CUBES {
        let mut face: Vec<char> = Vec::new();
        for l in 0..n {
            if b.cubes_present[i][l] {
                face.push(index_char(l));
            }
        }
        face.sort_unstable();
        let s = face
            .iter()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<_>>()
            .join(",");
        parts.push(format!("Set{{Char}}([{}])", s));
    }
    format!("[{}]", parts.join(", "))
}

pub fn print_cubes_code(b: &Built) {
    let n = n_letters();

    // --- Julia ---
    println!("=== Cubes (copy-pasteable Julia) ===");
    println!("const CUBES :: Vector{{Set{{Char}}}} = {}", cubes_as_code_string(b));

    // Collect sorted faces once for reuse
    let mut faces: Vec<String> = Vec::with_capacity(N_CUBES);
    for i in 0..N_CUBES {
        let mut letters: Vec<char> = Vec::new();
        for l in 0..n {
            if b.cubes_present[i][l] {
                letters.push(index_char(l));
            }
        }
        letters.sort_unstable();
        faces.push(letters.iter().collect::<String>());
    }

    // --- JavaScript: const CUBES = [['a','b','c','d','e','f'], ...];
    println!("=== Cubes (JavaScript) ===");
    let mut js = String::from("const CUBES = [");
    for (k, s) in faces.iter().enumerate() {
        if k > 0 {
            js.push_str(", ");
        }
        js.push('[');
        for (j, ch) in s.chars().enumerate() {
            if j > 0 {
                js.push_str(", ");
            }
            js.push('\'');
            js.push(ch);
            js.push('\'');
        }
        js.push(']');
    }
    js.push_str("];");
    println!("{}", js);

    // --- Python: cubes = [ set(\"abcdef\"), set(\"...\"), ... ]
    println!("=== Cubes (Python) ===");
    let mut py = String::from("cubes = [");
    for (k, s) in faces.iter().enumerate() {
        if k > 0 {
            py.push_str(", ");
        }
        py.push_str("set(\"");
        py.push_str(s);
        py.push_str("\")");
    }
    py.push(']');
    println!("{}", py);
}
