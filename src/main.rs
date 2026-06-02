// AlphaWord Blocks — command-line entry point and argument parsing
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

mod alphabet;   // runtime alphabet
mod types;
mod util;
mod filters;
mod stats;
mod assign;
mod verify;
mod interactive;
mod hall;       // used by interactive.rs
mod verify_cmd; // verifier entry point
mod glyph_db;
mod glyph_config;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="alphaword-blocks", about="AlphaWord Blocks: design 26×6 letter blocks that spell a large corpus")]
pub struct Args {
    /// If provided, run the verifier on a config file and exit.
    #[arg(long)]
    verify: Option<String>,

    /// PRNG seed (for interactive/annealing mode)
    #[arg(long, default_value_t = 0)]
    seed: i64,

    /// Annealing sweeps (interactive mode)
    #[arg(long, default_value_t = 200_000_000)]
    sweeps: usize,

    /// Max K for stats (interactive mode)
    #[arg(long, default_value_t = 20)]
    max_k: usize,
}

fn main() -> anyhow::Result<()> {
    // 1) Load and build a plan from TOML
    let toml_src = std::fs::read_to_string("src/alphabet.toml")?;
    let plan = glyph_config::build_from_toml(&toml_src)?;
    glyph_config::print_preview(&plan);

    // 2) Build the runtime alphabet from plan.tokens
    alphabet::build_alphabet(&plan.tokens)?;

    // 3) Install rewrites:
    //    - combine plan-provided rewrites with a baseline pack that mirrors the old char map
    //    - auto-disable rules whose LHS token exists in the compiled alphabet
  //  let mut all_rewrites = plan.rewrites.clone();
 //   all_rewrites.extend(alphabet::default_rewrite_pairs());
//    alphabet::install_rewrites_filtered(&all_rewrites)?;
    alphabet::install_rewrites_filtered(&plan.rewrites)?;

    // 4) Pretty print the final compiled alphabet for human inspection
    alphabet::print_alphabet_summary();

    // 5) CLI dispatch
    let args = Args::parse();
    if let Some(conf) = args.verify.as_deref() {
        return verify_cmd::run_from_conf(conf);
    }
    interactive::interactive_main(args.seed, args.sweeps, args.max_k)?;
    Ok(())
}
