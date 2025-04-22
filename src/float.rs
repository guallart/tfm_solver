use crate::math::{erff256, erff32, erff64, exp256, gammaf256, gammaf32, gammaf64, powf256};
use f256::f256;
use rand::{rngs::ThreadRng, Rng};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::{f32, f64};

pub trait Float:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Copy
    + Display
{
    const ONE: Self;
    const NEG_ONE: Self;
    const ZERO: Self;
    const TWO: Self;
    const PI: Self;

    fn new<F: Into<f64>>(x: F) -> Self;
    fn random(rng: &mut ThreadRng) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn powf(self, exponent: Self) -> Self;
    fn gamma(self) -> Self;
    fn abs(self) -> Self;
    fn erf(self) -> Self;
    #[allow(unused)]
    fn into_f64(self) -> f64;
}

impl Float for f32 {
    const ONE: Self = 1.0f32;
    const NEG_ONE: Self = -1.0f32;
    const ZERO: Self = 0.0f32;
    const TWO: Self = 2.0f32;
    const PI: Self = f32::consts::PI;

    fn new<T>(x: T) -> Self
    where
        T: Into<f64>,
    {
        x.into() as f32
    }

    fn random(rng: &mut ThreadRng) -> Self {
        rng.random::<f32>()
    }

    fn sqrt(self) -> f32 {
        f32::sqrt(self)
    }

    fn exp(self) -> f32 {
        f32::exp(self)
    }

    fn ln(self) -> f32 {
        f32::ln(self)
    }

    fn powf(self, exponent: f32) -> f32 {
        f32::powf(self, exponent)
    }

    fn gamma(self) -> f32 {
        gammaf32(self)
    }

    fn abs(self) -> f32 {
        f32::abs(self)
    }

    fn erf(self) -> f32 {
        erff32(self)
    }

    fn into_f64(self) -> f64 {
        self.into()
    }
}

impl Float for f64 {
    const ONE: Self = 1.0f64;
    const NEG_ONE: Self = -1.0f64;
    const ZERO: Self = 0.0f64;
    const TWO: Self = 2.0f64;
    const PI: Self = f64::consts::PI;

    fn new<T>(x: T) -> Self
    where
        T: Into<f64>,
    {
        x.into()
    }

    fn random(rng: &mut ThreadRng) -> Self {
        rng.random::<f64>()
    }

    fn sqrt(self) -> f64 {
        f64::sqrt(self)
    }

    fn exp(self) -> f64 {
        f64::exp(self)
    }

    fn ln(self) -> f64 {
        f64::ln(self)
    }

    fn powf(self, exponent: f64) -> f64 {
        f64::powf(self, exponent)
    }

    fn gamma(self) -> f64 {
        gammaf64(self)
    }

    fn abs(self) -> f64 {
        f64::abs(self)
    }

    fn erf(self) -> f64 {
        erff64(self)
    }

    fn into_f64(self) -> f64 {
        self
    }
}

impl Float for f256 {
    const ONE: Self = f256::ONE;
    const NEG_ONE: Self = f256::NEG_ONE;
    const ZERO: Self = f256::ZERO;
    const TWO: Self = f256::TWO;
    // not exactly pi to all decimals, only the first 64 bits, but good enough
    const PI: Self = f256::from_bits((85070776964233020888359549780463976448, 0));

    fn new<T>(x: T) -> Self
    where
        T: Into<f64>,
    {
        f256::from(x.into())
    }

    fn random(rng: &mut ThreadRng) -> Self {
        f256::from(rng.random::<f64>())
    }

    fn sqrt(self) -> f256 {
        f256::sqrt(self)
    }

    fn exp(self) -> f256 {
        exp256(self)
    }

    fn ln(self) -> f256 {
        f256::ln(&self)
    }

    fn powf(self, exponent: f256) -> f256 {
        powf256(self, exponent)
    }

    fn gamma(self) -> f256 {
        gammaf256(self)
    }

    fn abs(self) -> f256 {
        f256::abs(&self)
    }

    fn erf(self) -> f256 {
        erff256(self)
    }

    fn into_f64(self) -> f64 {
        self.to_string().parse::<f64>().unwrap()
    }
}

#[allow(unused)]
pub trait PrecisionStr {
    fn precision_str() -> String;
}

impl PrecisionStr for f32 {
    fn precision_str() -> String {
        "f32".to_string()
    }
}

impl PrecisionStr for f64 {
    fn precision_str() -> String {
        "f64".to_string()
    }
}

impl PrecisionStr for f256 {
    fn precision_str() -> String {
        "f256".to_string()
    }
}
