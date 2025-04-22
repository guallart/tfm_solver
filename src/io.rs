use crate::config::{Precision, L, N_UNK};
use crate::dists::Distribution;
use chrono::{Datelike, Timelike, Utc};
use itertools::Itertools;
use std::fs::{metadata, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;
// use std::io::BufReader;
// use regex::Regex;
// use std::collections::HashMap;

fn datetime() -> String {
    let now = Utc::now();
    format!(
        "{:?} {:02}-{:02}-{:04} {:02}:{:02}:{:02}",
        now.weekday(),
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second(),
    )
}

#[allow(dead_code)]
pub fn save_matrix(
    matrix: &'static [[Precision; N_UNK + 1]; N_UNK],
    file_path: &Path,
    header: &Vec<String>,
) -> std::io::Result<()> {
    let file = File::create(Path::new(file_path))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "#{}", datetime())?;
    for line in header {
        writeln!(writer, "#{line}")?;
    }

    for row in matrix.iter() {
        let row_string = row
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(writer, "{}", row_string)?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn save_array(
    file_path: &Path,
    arr: &Vec<Precision>,
    header: &Vec<String>,
) -> std::io::Result<()> {
    let file = File::create(Path::new(file_path))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "#{}", datetime())?;
    for line in header {
        writeln!(writer, "#{line}")?;
    }

    let content = arr
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    writeln!(writer, "{content}")?;

    Ok(())
}

pub fn export_arrays<D: Distribution<Precision>>(
    dist: &D,
    outdir: &Path,
    resist: &Vec<Precision>,
    x: &Vec<Precision>,
    err2: Precision,
    errinf: Precision,
    iter: usize,
) -> std::io::Result<()> {
    let header = vec![
        "Solution x of the system of equations".to_string(),
        format!("L={L}"),
        format!("dist={}", dist),
        format!("error2={err2:.5e}"),
        format!("error_inf={errinf:.5e}"),
    ];

    let x_path = outdir.join(format!("L{}_{}_{:04}.x", L, dist, iter));
    save_array(&x_path, &x, &header)?;

    let header = vec![
        "Resistances of the system of equations".to_string(),
        format!("L={L}"),
        format!("dist={}", dist),
    ];
    let resist_path = outdir.join(format!("L{}_{}_{:04}.r", L, dist, iter));
    save_array(&resist_path, &resist, &header)?;

    Ok(())
}

pub fn export_surface(out_file: &Path, surface: &Vec<(usize, usize)>) -> std::io::Result<()> {
    let file_exists = metadata(out_file).is_ok();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(out_file)?;
    let mut writer = BufWriter::new(file);

    if !file_exists {
        writeln!(writer, "#{}", datetime())?;
    }
    let line = surface
        .iter()
        .map(|(r, c)| format!("{} {}", r, c))
        .join(" ");

    writeln!(writer, "{}", line)?;

    Ok(())
}

#[allow(dead_code)]
pub fn plot_matrix(file_path: &str) {
    Command::new("py")
        .args(["utils/plot_matrix.py", file_path])
        .output()
        .expect("Failed at plotting matrix");
}

// #[allow(dead_code)]
// pub fn load_vec(
//     file_path: &Path,
//     arr: &mut Vec<f64>,
// ) -> std::io::Result<(HashMap<String, String>)> {
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(file);
//     let mut info = HashMap::new();
//     let re = Regex::new(r"#(.+)=(.+)").unwrap();

//     let mut line = "".to_string();
//     for maybe_line in reader.lines() {
//         let line = maybe_line?;
//         if line.starts_with("#") {
//             if let Some(caps) = re.captures(&line) {
//                 let key = caps.get(1).unwrap().to_string();
//                 let value = caps.get(2).unwrap().to_string();
//                 info.insert(key, value);
//             }
//         } else {
//         }
//     }

//     Ok(info)
// }
