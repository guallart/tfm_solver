#[macro_use]
mod benchmarking;
mod config;
mod dists;
mod float;
mod io;
mod math;
mod solver;

use clap::Parser;
use dists::{Distribution, InverseDist, LogNormalDist, ValidDists, WeibullDist};
use solver::{compute_n_tries, ExportMode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Distribution
    #[arg(short, long)]
    dist: String,

    /// Parameter value
    #[arg(short, long)]
    param: String,

    /// Export mode
    #[arg(short, long, default_value = "ExportIsoSurface")]
    export: String,

    /// Isosurface value
    #[arg(short, long, default_value_t = 0.0)]
    surfval: f64,

    /// Output directory
    #[arg(short, long, default_value_t = String::from("."))]
    outdir: String,
}

fn main() {
    let args = Args::parse();
    let param = args.param.parse::<f64>().unwrap();

    let dist = match args.dist.as_str() {
        "inverse" => ValidDists::InverseDist(InverseDist::new(param)),
        "weibull" => ValidDists::WeibullDist(WeibullDist::new(param)),
        "lognormal" => ValidDists::LogNormalDist(LogNormalDist::new(param)),
        _ => panic!("Distribution {} not supported.", args.dist),
    };

    let export_mode = match args.export.to_lowercase().as_str() {
        "noexport" => ExportMode::NoExport,
        "exportarrays" => ExportMode::ExportArrays,
        "exportisosurface" => ExportMode::ExportIsoSurface,
        "exportarraysandisosurface" => ExportMode::ExportArraysAndIsoSurface,
        _ => panic!("Export mode {} not supported.", args.export),
    };

    match dist {
        ValidDists::InverseDist(inner) => {
            compute_n_tries(inner, &args.outdir, export_mode, args.surfval)
        }
        ValidDists::WeibullDist(inner) => {
            compute_n_tries(inner, &args.outdir, export_mode, args.surfval)
        }
        ValidDists::LogNormalDist(inner) => {
            compute_n_tries(inner, &args.outdir, export_mode, args.surfval)
        }
    }
}
