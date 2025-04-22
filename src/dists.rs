use crate::float::Float;
use rand::rngs::ThreadRng;
use std::convert::Into;
use std::fmt;

pub trait Distribution<T>: fmt::Display
where
    T: Float,
{
    fn new<F: Into<f64>>(param: F) -> Self;
    fn sample(&self, rng: &mut ThreadRng) -> T;

    #[allow(unused)]
    fn mean(&self) -> T;
}

#[allow(unused)]
pub struct InverseDist<T>
where
    T: Float,
{
    param: T,
}

impl<T: Float> fmt::Display for InverseDist<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inverse(a={:.3})", self.param)
    }
}

impl<T: Float> Distribution<T> for InverseDist<T> {
    fn new<F: Into<f64>>(param: F) -> InverseDist<T> {
        InverseDist {
            param: T::new(param.into()),
        }
    }

    fn sample(&self, rng: &mut ThreadRng) -> T {
        (self.param * T::random(rng)).exp()
    }

    #[allow(unused)]
    fn mean(&self) -> T {
        (self.param.exp() - T::ONE) / self.param
    }
}

#[allow(unused)]
pub struct WeibullDist<T>
where
    T: Float,
{
    param: T,
}

impl<T: Float> fmt::Display for WeibullDist<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Weibull(k={:.3})", self.param)
    }
}

impl<T: Float> Distribution<T> for WeibullDist<T> {
    fn new<F: Into<f64>>(param: F) -> WeibullDist<T> {
        WeibullDist {
            param: T::new(param.into()),
        }
    }

    fn sample(&self, rng: &mut ThreadRng) -> T {
        (T::NEG_ONE * (T::ONE - T::random(rng)).ln()).powf(T::ONE / self.param)
    }

    #[allow(unused)]
    fn mean(&self) -> T {
        (T::ONE + T::ONE / self.param).gamma()
    }
}

#[allow(unused)]
pub struct LogNormalDist<T>
where
    T: Float,
{
    param: T,
}

impl<T: Float> fmt::Display for LogNormalDist<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LogNormal(s={:.3})", self.param)
    }
}

impl<T: Float> Distribution<T> for LogNormalDist<T> {
    fn new<F: Into<f64>>(param: F) -> LogNormalDist<T> {
        LogNormalDist {
            param: T::new(param.into()),
        }
    }

    fn sample(&self, rng: &mut ThreadRng) -> T {
        let w = T::random(rng);
        let log = w.ln();
        T::ONE / (w * (T::TWO * T::PI * self.param * self.param).sqrt())
            * (T::NEG_ONE * log * log / (T::TWO * self.param * self.param)).exp()
    }

    #[allow(unused)]
    fn mean(&self) -> T {
        let s = self.param;
        (s * (s * s / T::TWO)).exp() / s.abs()
            - (s * (s * s / T::TWO).exp() * (s / T::TWO.sqrt()).erf()) / s.abs()
    }
}

pub enum ValidDists<T: Float> {
    InverseDist(InverseDist<T>),
    WeibullDist(WeibullDist<T>),
    LogNormalDist(LogNormalDist<T>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::float::{Float, PrecisionStr};
    use approx::assert_relative_eq;
    use f256::f256;
    use plotters::prelude::*;
    use rand;
    use std::path::Path;

    const EPS: f64 = 5e-2;

    fn empty_plots_directory() {
        let dir_path = "dist_plots";
        if std::path::Path::new(dir_path).exists() {
            std::fs::remove_dir_all(dir_path).unwrap();
        }
        std::fs::create_dir(dir_path).unwrap();
    }

    fn histogram(
        data: &Vec<f64>,
        file_name: &str,
        expected_mean: f64,
        actual_mean: f64,
        title: &str,
    ) {
        let file_name = Path::new("dist_plots").join(file_name);
        let drawing_area = BitMapBackend::new(file_name.as_path(), (800, 600)).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();

        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .margin(5)
            .set_left_and_bottom_label_area_size(40);

        let x_min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let x_max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bins = 50;
        let bin_width = (x_max - x_min) / bins as f64;
        let mut histogram_data = vec![0; bins];
        for &value in data {
            let bin = ((value - x_min) / bin_width).floor() as usize;
            if bin < bins {
                histogram_data[bin] += 1;
            }
        }

        let y_max = *histogram_data.iter().max().unwrap() as i32;

        let mut chart_context = chart_builder
            .caption(title, ("sans-serif", 20).into_font())
            .build_cartesian_2d(x_min..x_max, 0i32..y_max)
            .unwrap();
        chart_context.configure_mesh().draw().unwrap();
        chart_context
            .draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
                let x0 = x_min + i as f64 * bin_width;
                let x1 = x_min + (i + 1) as f64 * bin_width;
                Rectangle::new([(x0, 0), (x1, count as i32)], BLUE.filled())
            }))
            .unwrap();

        let vertical_dashed_line = |x: f64, color: &RGBColor| {
            let color = color.clone();
            (0..y_max)
                .step_by(10)
                .map(move |y| PathElement::new(vec![(x, y), (x, y + 5)], color.stroke_width(2)))
        };

        chart_context
            .draw_series(vertical_dashed_line(expected_mean, &RED))
            .unwrap()
            .label("Expected Mean")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

        chart_context
            .draw_series(vertical_dashed_line(actual_mean, &GREEN))
            .unwrap()
            .label("Actual Mean")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN.stroke_width(2)));

        chart_context
            .configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .draw()
            .unwrap();
    }

    fn test_dist<D, T>(mut rng: &mut ThreadRng, params: &Vec<f64>, n_tries: usize)
    where
        D: Distribution<T>,
        T: Float + PrecisionStr + 'static,
    {
        for p in params.into_iter() {
            let dist = D::new(*p);
            println!("Testing {}", dist);
            let data: Vec<f64> = (0..n_tries)
                .map(|_| dist.sample(&mut rng).into_f64())
                .collect();

            let precision = T::precision_str();
            let file_name = format!("{}_{}.png", dist, precision);
            let expected = dist.mean().into_f64();
            let actual = data.iter().sum::<f64>() / n_tries as f64;
            histogram(&data, &file_name, expected, actual, &format!("{}", dist));
            assert_relative_eq!(actual / expected, 1.0, epsilon = EPS);
        }
    }

    #[test]
    fn test_means() {
        let mut rng = rand::rng();
        let n_tries: usize = 10000;

        empty_plots_directory();

        let params = vec![3.0, 5.0, 10.0];
        test_dist::<InverseDist<f64>, f64>(&mut rng, &params, n_tries);
        test_dist::<InverseDist<f256>, f256>(&mut rng, &params, n_tries);

        let params = vec![5.0, 3.0, 1.0];
        test_dist::<WeibullDist<f64>, f64>(&mut rng, &params, n_tries);
        test_dist::<WeibullDist<f256>, f256>(&mut rng, &params, n_tries);

        let params = vec![1.0, 0.5, 0.25];
        test_dist::<LogNormalDist<f64>, f64>(&mut rng, &params, n_tries);
        test_dist::<LogNormalDist<f256>, f256>(&mut rng, &params, n_tries);
    }
}
