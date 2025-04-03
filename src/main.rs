#[macro_use]
mod benchmarking;
mod config;
mod io;
mod math;
mod solver;
mod types;

use config::{Dist, Precision};
use solver::{compute_n_tries, ExportMode};
use std::env;
use std::path::Path;
#[allow(unused_imports)]
use types::{Distribution, Float};

fn main() {
    let args: Vec<String> = env::args().collect();
    let param = args[1].parse::<Precision>().unwrap();
    // let param = Precision::new(3);
    let dist = Dist::new(param);
    let outdir = Path::new(".");
    compute_n_tries(dist, &outdir, ExportMode::ExportIsoSurface);
}
