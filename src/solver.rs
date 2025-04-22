use crate::config::{Precision, L, N_RES, N_THREADS, N_TRIES, N_UNK};
#[allow(unused_imports)]
use crate::dists::Distribution;
use crate::float::Float;
use crate::io;
use rand::rngs::ThreadRng;
use std::cmp::min;
use std::collections::VecDeque;
use std::mem;
use std::path::Path;
use std::thread;
use std::time::Duration;

const V_HGH: Precision = Precision::ONE;
const V_LOW: Precision = Precision::NEG_ONE;
const ZERO: Precision = Precision::ZERO;

static mut MATRIX: [[Precision; N_UNK + 1]; N_UNK] = [[ZERO; N_UNK + 1]; N_UNK];

#[allow(dead_code)]
pub enum ExportMode {
    NoExport,
    ExportArrays,
    ExportIsoSurface,
    ExportArraysAndIsoSurface,
}

#[inline]
fn fidx(i: usize, j: usize) -> usize {
    (i) + (j) * L
}

fn fill_resistances<D: Distribution<Precision>>(
    dist: &D,
    rng: &mut ThreadRng,
    resist: &mut Vec<Precision>,
) {
    for x in resist.iter_mut() {
        *x = dist.sample(rng);
    }
}

unsafe fn empty_matrix() {
    for i in 0..N_UNK {
        for j in 0..N_UNK + 1 {
            MATRIX[i][j] = ZERO;
        }
    }
}

unsafe fn swap_rows(row1: usize, row2: usize) {
    for col in 0..N_UNK + 1 {
        mem::swap(&mut MATRIX[row1][col], &mut MATRIX[row2][col]);
    }
}

unsafe fn build_system(resist: &Vec<Precision>) {
    let mut r_iter = resist.iter();
    empty_matrix();

    // Vertical resistances
    for j in 0..(L - 2) {
        for i in 0..(L - 1) {
            let r = r_iter.next().unwrap();
            let hgh = fidx(i, j);
            let low = fidx(i + 1, j);
            MATRIX[hgh][hgh] += -r;
            MATRIX[hgh][low] += r;
            MATRIX[low][hgh] += r;
            MATRIX[low][low] += -r;
        }
    }

    // Internal horizontal resistances
    for j in 0..(L - 3) {
        for i in 0..L {
            let r = r_iter.next().unwrap();
            let hgh = fidx(i, j);
            let low = fidx(i, j + 1);
            MATRIX[hgh][hgh] += -r;
            MATRIX[hgh][low] += r;
            MATRIX[low][hgh] += r;
            MATRIX[low][low] += -r;
        }
    }

    // High horizontal resistances
    for i in 0..L {
        let r = r_iter.next().unwrap();
        let idx = fidx(i, 0);
        MATRIX[idx][idx] += -r;
        MATRIX[idx][N_UNK] += -V_HGH * r;
    }

    // Low horizontal resistances
    for i in 0..L {
        let r = r_iter.next().unwrap();
        let idx = fidx(i, L - 3);
        MATRIX[idx][idx] += -r;
        MATRIX[idx][N_UNK] += -V_LOW * r;
    }
}

unsafe fn gauss_elimination(x: &mut Vec<Precision>) -> std::result::Result<(), String> {
    let rows = N_UNK;
    let cols = N_UNK + 1;

    for j in 0..(cols - 1) {
        if MATRIX[j][j] == ZERO {
            let pivot_row = (j + 1..rows)
                .find(|&i| MATRIX[i][j] != ZERO)
                .ok_or_else(|| format!("No pivot found in column {} below the diagonal", j))?;

            swap_rows(j, pivot_row);
        }

        for chunk_start in (j + 1..rows).step_by(N_THREADS) {
            let n_to_spawn = min(N_THREADS, rows - chunk_start);
            let threads: Vec<_> = (0..n_to_spawn)
                .map(|t| {
                    thread::spawn(move || {
                        let i = chunk_start + t;
                        if MATRIX[i][j] == ZERO {
                            return;
                        }
                        let factor = MATRIX[i][j] / MATRIX[j][j];
                        for k in j..cols {
                            MATRIX[i][k] = MATRIX[i][k] - factor * MATRIX[j][k];
                        }
                    })
                })
                .collect();

            for handle in threads {
                handle.join().unwrap();
            }
        }
    }

    // Backpropagation to solve for x
    for i in (0..rows).rev() {
        let dot_product = (i + 1..rows)
            .map(|j| MATRIX[i][j] * x[j])
            .fold(ZERO, |a, b| a + b);
        x[i] = (MATRIX[i][cols - 1] - dot_product) / MATRIX[i][i];
    }

    Ok(())
}

enum Norm {
    Two,
    Inf,
}

#[allow(static_mut_refs)]
unsafe fn compute_error(x: &Vec<Precision>, norm: Norm) -> Precision {
    let rows = MATRIX.len();
    let mut residual: Vec<Precision> = vec![ZERO; rows];

    for i in 0..rows {
        let ax_i: Precision = (0..rows)
            .map(|j| MATRIX[i][j] * x[j])
            .fold(ZERO, |a, b| a + b);
        let b_i = MATRIX[i][rows];
        residual[i] = ax_i - b_i;
    }

    match norm {
        Norm::Two => residual
            .iter()
            .map(|val| val * val)
            .fold(ZERO, |a, b| a + b)
            .sqrt(),
        Norm::Inf => residual
            .into_iter()
            .max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())
            .unwrap()
            .abs(),
    }
}

fn compute_isosurface(x: &Vec<Precision>, value: Precision) -> Vec<(usize, usize)> {
    let mut surface: Vec<(usize, usize)> = Vec::with_capacity(L);

    for row in 0..L {
        for col in 0..L - 2 {
            if x[fidx(row, col)] < value {
                continue;
            }

            let neighs: Vec<(usize, usize)> = [
                row.checked_sub(1).map(|r| (r, col)),    // Up
                (row < L - 1).then_some((row + 1, col)), // Down
                col.checked_sub(1).map(|c| (row, c)),    // Left
                (col < L - 3).then_some((row, col + 1)), // Right
            ]
            .into_iter()
            .flatten() // Remove None values
            .collect();

            'nloop: for neigh in neighs.iter() {
                if x[fidx(neigh.0, neigh.1)] < value {
                    surface.push(*neigh);
                    break 'nloop;
                }
            }
        }
    }

    surface
}

fn compute_eta_and_completion_time(
    times: &VecDeque<Duration>,
    remaining_iters: usize,
) -> (String, String) {
    let avg_secs: f64 = times.iter().map(|d| d.as_secs_f64()).sum::<f64>() / times.len() as f64;
    let remaining_secs = (avg_secs * remaining_iters as f64) as usize;
    let hours = remaining_secs / 3600;
    let minutes = (remaining_secs % 3600) / 60;
    let seconds = remaining_secs % 60;
    let eta = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    let now = chrono::Local::now();
    let estimated_end = now + chrono::Duration::seconds(remaining_secs as i64);

    let estimated_end_str = if now.date_naive() == estimated_end.date_naive() {
        format!("{}", estimated_end.format("%H:%M:%S"))
    } else {
        format!("{}", estimated_end.format("%d-%m-%Y %H:%M:%S"))
    };

    (eta, estimated_end_str)
}

pub fn compute_n_tries<D, P>(dist: D, outdir: P, export_mode: ExportMode, isosurface_value: f64)
where
    D: Distribution<Precision>,
    P: AsRef<Path>,
{
    let mut resist: Vec<Precision> = vec![ZERO; N_RES];
    let mut x: Vec<Precision> = vec![ZERO; N_UNK];
    let mut rng = rand::rng();
    let outdir: &Path = outdir.as_ref();
    let mut times: VecDeque<Duration> = VecDeque::with_capacity(100);
    let isosurface_value = Precision::new(isosurface_value);

    unsafe {
        for iter in 0..N_TRIES {
            let (duration_fill, _) = timeit!(fill_resistances(&dist, &mut rng, &mut resist));
            let (duration_build, _) = timeit!(build_system(&mut resist));
            let (duration_gauss, result) = timeit!(gauss_elimination(&mut x));

            let msg = match result.clone() {
                Ok(_) => "DONE".to_string(),
                Err(e) => format!("FAIL - Gaussian elimination failed. {:?}", e),
            };

            let err2 = compute_error(&x, Norm::Two);
            let errinf = compute_error(&x, Norm::Inf);

            if times.len() >= 100 {
                times.pop_front();
            }
            let duration = duration_fill + duration_build + duration_gauss;
            times.push_back(duration);

            let (eta, completion_time) =
                compute_eta_and_completion_time(&times, N_TRIES - iter - 1);

            println!(
                "{}/{}  L={:>3}  dist={}  time={:>5.3}s  |err|2={:<9.3e}  |err|inf={:<9.3e}  {}  ETA={}  completion={}",
                iter + 1,
                N_TRIES,
                L,
                dist,
                duration.as_secs_f64(),
                err2,
                errinf,
                msg,
                eta,
                completion_time
            );

            if let Err(_e) = result {
                continue;
            }

            match export_mode {
                ExportMode::NoExport => continue,
                ExportMode::ExportArrays => {
                    io::export_arrays(&dist, &outdir, &resist, &x, err2, errinf, iter)
                        .expect("Failed at saving results");
                }
                ExportMode::ExportIsoSurface => {
                    let surface = compute_isosurface(&x, isosurface_value);
                    let surf_file = outdir.join(format!("isosurfaces_L{}_{}.out", L, dist));
                    io::export_surface(&surf_file, &surface).expect("Failed at saving results");
                }
                ExportMode::ExportArraysAndIsoSurface => {
                    io::export_arrays(&dist, &outdir, &resist, &x, err2, errinf, iter)
                        .expect("Failed at saving results");

                    let surface = compute_isosurface(&x, isosurface_value);
                    let surf_file = outdir.join(format!("isosurfaces_L{}_{}.out", L, dist));
                    io::export_surface(&surf_file, &surface).expect("Failed at saving results");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Precision;
    use crate::math;

    #[test]
    fn test_compute_surface() {
        let mut x: Vec<Precision> = vec![Precision::ZERO; N_UNK];
        let values = math::linspace::<Precision>(V_LOW, V_HGH, L - 2);
        let isosurface_value = Precision::ZERO;

        for (j, v) in values.iter().enumerate() {
            for i in 0..L {
                let idx = fidx(i, j);
                x[idx] = *v;
            }
        }

        let surface = compute_isosurface(&x, isosurface_value);
        let mid_column = (L - 2) / 2 - 1;
        for (i, (row, col)) in surface.into_iter().enumerate() {
            assert_eq!(row, i);
            assert_eq!(col, mid_column);
        }
    }
}
