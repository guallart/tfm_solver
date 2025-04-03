use f256::f256;
use std::f64::consts::PI;

use crate::types::Float;

const EXP_ITERS: u64 = 50;

pub fn factorial256(n: u64) -> f256 {
    (1..=n).fold(f256::ONE, |a, b| a * f256::from(b))
}

pub fn powi256(x: f256, n: u64) -> f256 {
    (0..n).fold(f256::ONE, |a, _b| a * x)
}

pub fn exp256(x: f256) -> f256 {
    f256::ONE
        + (1..=EXP_ITERS)
            .map(|n| powi256(x, n) / factorial256(n))
            .fold(f256::ZERO, |a, b| a + b)
}

pub fn powf256(b: f256, x: f256) -> f256 {
    exp256(x * b.ln())
}

#[allow(unused)]
pub fn intof64(x: f256) -> f64 {
    x.to_string().parse::<f64>().unwrap()
}

const G: f64 = 7.0;
const N: usize = 9;
const P: [f64; N] = [
    0.99999999999980993,
    676.5203681218851,
    -1259.1392167224028,
    771.32342877765313,
    -176.61502916214059,
    12.507343278686905,
    -0.13857109526572012,
    9.9843695780195716e-6,
    1.5056327351493116e-7,
];

pub fn gammaf64(z: f64) -> f64 {
    if z < 0.5 {
        PI / ((z * PI).sin() * gammaf64(1.0 - z))
    } else {
        let z = z - 1.0;
        let mut x = P[0];
        for i in 1..N {
            x += P[i] / (z + i as f64);
        }
        let t = z + G + 0.5;
        (2.0 * PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * x
    }
}

pub fn gammaf256(z: f256) -> f256 {
    let pi256: f256 = f256::from(PI);
    let half: f256 = f256::from(0.5);
    let g: f256 = f256::from(7.0);
    let p: [f256; N] = [
        f256::from(0.99999999999980993),
        f256::from(676.5203681218851),
        f256::from(-1259.1392167224028),
        f256::from(771.32342877765313),
        f256::from(-176.61502916214059),
        f256::from(12.507343278686905),
        f256::from(-0.13857109526572012),
        f256::from(9.9843695780195716e-6),
        f256::from(1.5056327351493116e-7),
    ];

    if z < half {
        pi256 / ((z * pi256).sin() * gammaf256(f256::ONE - z))
    } else {
        let z = z - f256::ONE;
        let mut x = p[0];
        for i in 1..N {
            x += p[i] / (z + f256::from(i as f64));
        }
        let t = z + g + half;
        (f256::TWO * pi256).sqrt() * powf256(t, z + half) * exp256(-t) * x
    }
}

#[allow(unused)]
pub fn linspace(start: f64, end: f64, num: usize) -> Vec<f64> {
    if num < 2 {
        return vec![start];
    }

    let step = (end - start) / (num - 1) as f64;
    (0..num).map(|i| start + i as f64 * step).collect()
}

pub fn erff64(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let t = 1.0 / (1.0 + p * x);
    let temp = (-x * x).exp();

    let poly = t * (a1 + t * (a2 + t * (a3 + t * (a4 + a5 * t))));

    sign * (1.0 - poly * temp)
}

pub fn erff256(x: f256) -> f256 {
    let sign = if x < f256::ZERO {
        f256::NEG_ONE
    } else {
        f256::ONE
    };
    let x = x.abs();

    let a1 = f256::from(0.254829592);
    let a2 = f256::from(-0.284496736);
    let a3 = f256::from(1.421413741);
    let a4 = f256::from(-1.453152027);
    let a5 = f256::from(1.061405429);
    let p = f256::from(0.3275911);

    let t = f256::ONE / (f256::ONE + p * x);
    let temp = (-x * x).exp();

    let poly = t * (a1 + t * (a2 + t * (a3 + t * (a4 + a5 * t))));

    sign * (f256::ONE - poly * temp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const EPS: f64 = 1e-5;

    #[test]
    fn test_exp() {
        let x: f64 = 3.0;
        let actual = intof64(exp256(f256::from(x)));
        let expected = x.exp();
        assert_relative_eq!(actual, expected, epsilon = EPS);

        let x: f64 = 1.1;
        let actual = intof64(exp256(f256::from(x)));
        let expected = x.exp();
        assert_relative_eq!(actual, expected, epsilon = EPS);

        let x: f64 = 0.5;
        let actual = intof64(exp256(f256::from(x)));
        let expected = x.exp();
        assert_relative_eq!(actual, expected, epsilon = EPS);

        let x: f64 = 0.001;
        let actual = intof64(exp256(f256::from(x)));
        let expected = x.exp();
        assert_relative_eq!(actual, expected, epsilon = EPS);
    }

    #[test]
    fn test_gamma() {
        let x = 5.0;
        let expected = 24.0;
        let actualf64 = gammaf64(x);
        let actualf256 = intof64(gammaf256(f256::from(x)));
        assert_relative_eq!(actualf64, expected, epsilon = EPS);
        assert_relative_eq!(actualf256, expected, epsilon = EPS);

        let x = 0.5;
        let expected = PI.sqrt();
        let actualf64 = gammaf64(x);
        let actualf256 = intof64(gammaf256(f256::from(x)));
        assert_relative_eq!(actualf64, expected, epsilon = EPS);
        assert_relative_eq!(actualf256, expected, epsilon = EPS);
    }
}
