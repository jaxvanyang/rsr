//! Wavelengths are specified in nanometers (nm).

use super::{
	Float,
	math::{fast_exp, lerp},
};
use std::ops;

pub const LAMBDA_MIN: Float = 360.;
pub const LAMBDA_MAX: Float = 830.;
pub const SPECTRUM_SAMPLES: usize = 4;

pub trait Spectrum {
	/// Evaluates the spectrum at the given wavelength.
	fn eval(&self, lambda: Float) -> Float;

	fn max_value(&self) -> Float {
		0.0
	}

	fn sample(&self, swl: &SampledWavelengths) -> SampledSpectrum {
		SampledSpectrum::new(
			&swl.lambdas
				.iter()
				.map(|lambda| self.eval(*lambda))
				.collect::<Vec<_>>(),
		)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantSpectrum {
	c: Float,
}

impl ConstantSpectrum {
	pub fn new(c: Float) -> Self {
		Self { c }
	}
}

impl Spectrum for ConstantSpectrum {
	fn eval(&self, _: Float) -> Float {
		self.c
	}

	fn sample(&self, _: &SampledWavelengths) -> SampledSpectrum {
		SampledSpectrum::new_with_const(self.c)
	}
}

#[derive(Debug, Clone)]
pub struct DenselySampledSpectrum {
	lambda_min: usize,
	lambda_max: usize,
	values: Vec<Float>,
}

impl DenselySampledSpectrum {
	pub fn new(spec: &dyn Spectrum, lambda_min: usize, lambda_max: usize) -> Self {
		let values = (lambda_min..=lambda_max)
			.map(|lambda| spec.eval(lambda as Float))
			.collect::<Vec<_>>();

		Self {
			lambda_min,
			lambda_max,
			values,
		}
	}
}

impl Spectrum for DenselySampledSpectrum {
	fn eval(&self, lambda: Float) -> Float {
		let lambda = lambda.ceil() as usize;
		if lambda < self.lambda_min || lambda > self.lambda_max {
			0.
		} else {
			self.values[lambda - self.lambda_min]
		}
	}

	fn max_value(&self) -> Float {
		self.values.iter().fold(0., |ret, v| ret.max(*v))
	}
}

#[derive(Debug, Clone)]
pub struct PiecewiseLinearSpectrum {
	lambdas: Vec<Float>,
	values: Vec<Float>,
}

impl PiecewiseLinearSpectrum {
	pub fn new(lambdas: &[Float], values: &[Float]) -> Self {
		assert!(lambdas.is_sorted());
		assert!(lambdas.len() == values.len());

		Self {
			lambdas: lambdas.to_vec(),
			values: values.to_vec(),
		}
	}
}

impl Spectrum for PiecewiseLinearSpectrum {
	fn eval(&self, lambda: Float) -> Float {
		if self.lambdas.is_empty()
			|| lambda < *self.lambdas.first().unwrap()
			|| lambda > *self.lambdas.last().unwrap()
		{
			return 0.;
		}

		if self.lambdas.len() == 1 {
			return if lambda == self.lambdas[0] {
				self.values[0]
			} else {
				0.
			};
		}

		let mut i = 0;
		while i < self.lambdas.len() - 1 && self.lambdas[i] <= lambda {
			i += 1;
		}
		let t = (lambda - self.lambdas[i - 1]) / (self.lambdas[i] - self.lambdas[i - 1]);

		lerp(t, self.values[i - 1], self.values[i])
	}

	fn max_value(&self) -> Float {
		self.values.iter().fold(0., |ret, v| ret.max(*v))
	}
}

/// Returns the blackbody spectrum value at the given wavelength (nm) and temperature (K).
#[allow(clippy::excessive_precision)]
pub fn blackbody(lambda: Float, t: Float) -> Float {
	if t <= 0. {
		return 0.;
	}

	let c = 299792458.;
	let h = 6.62606957e-34;
	let kb = 1.3806488e-23;
	let l = lambda * 1e-9;

	#[cfg(feature = "use_f64")]
	return (2. * h * c * c) / l.powi(5) * (fast_exp((h * c / (l * kb * t)) as f32) as Float - 1.);
	#[cfg(not(feature = "use_f64"))]
	return (2. * h * c * c) / l.powi(5) * (fast_exp(h * c / (l * kb * t)) - 1.);
}

#[derive(Debug, Clone, Copy)]
pub struct BlackbodySpectrum {
	t: Float,
	norm_factor: Float,
}

impl BlackbodySpectrum {
	/// Creates a new blackbody spectrum with the given temperature in Kelvin.
	#[allow(clippy::excessive_precision)]
	pub fn new(t: Float) -> Self {
		let lambda_max = 2.8977721e-3 / t;
		let norm_factor = 1. / blackbody(lambda_max * 1e9, t);

		Self { t, norm_factor }
	}
}

impl Spectrum for BlackbodySpectrum {
	fn eval(&self, lambda: Float) -> Float {
		blackbody(lambda, self.t) * self.norm_factor
	}

	fn max_value(&self) -> Float {
		1.
	}
}

#[derive(Debug, Clone)]
pub struct SampledSpectrum {
	values: [Float; SPECTRUM_SAMPLES],
}

impl SampledSpectrum {
	pub fn new(v: &[Float]) -> Self {
		Self {
			values: v[..SPECTRUM_SAMPLES].try_into().unwrap(),
		}
	}

	pub fn new_with_const(c: Float) -> Self {
		Self {
			values: [c; SPECTRUM_SAMPLES],
		}
	}

	pub fn is_zero(&self) -> bool {
		self.values.iter().all(|v| *v == 0.)
	}
}

impl ops::Index<usize> for SampledSpectrum {
	type Output = Float;

	fn index(&self, index: usize) -> &Self::Output {
		&self.values[index]
	}
}

impl ops::IndexMut<usize> for SampledSpectrum {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.values[index]
	}
}

// TODO: implement standard arithmetic operations for SampledSpectrum

#[derive(Debug, Clone)]
pub struct SampledWavelengths {
	lambdas: [Float; SPECTRUM_SAMPLES],
	pdf: [Float; SPECTRUM_SAMPLES],
}

impl SampledWavelengths {
	pub fn uniform(u: Float, lambda_min: Float, lambda_max: Float) -> Self {
		let mut lambdas = [0.; SPECTRUM_SAMPLES];
		lambdas[0] = lerp(u, lambda_min, lambda_max);
		let width = lambda_max - lambda_min;
		let delta = width / SPECTRUM_SAMPLES as Float;
		for i in 1..SPECTRUM_SAMPLES {
			lambdas[i] = lambdas[i - 1] + delta;
			if lambdas[i] > lambda_max {
				lambdas[i] -= width;
			}
		}

		Self {
			lambdas,
			pdf: [1. / width; SPECTRUM_SAMPLES],
		}
	}

	pub fn pdf(&self) -> SampledSpectrum {
		SampledSpectrum::new(&self.pdf)
	}

	pub fn terminate_secondary(&mut self) {
		if self.secondary_terminated() {
			return;
		}

		for i in 1..SPECTRUM_SAMPLES {
			self.pdf[i] = 0.;
		}
	}

	pub fn secondary_terminated(&self) -> bool {
		self.pdf[1..].iter().all(|v| *v == 0.)
	}
}

impl ops::Index<usize> for SampledWavelengths {
	type Output = Float;

	fn index(&self, index: usize) -> &Self::Output {
		&self.lambdas[index]
	}
}

impl ops::IndexMut<usize> for SampledWavelengths {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.lambdas[index]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_densely_sampled_spectrum() {
		let spec = DenselySampledSpectrum::new(&ConstantSpectrum::new(1.), 360, 830);
		assert_eq!(spec.eval(360.), 1.);
		assert_eq!(spec.eval(830.), 1.);
		assert_eq!(spec.eval(359.), 0.);
		assert_eq!(spec.eval(831.), 0.);
		assert_eq!(spec.max_value(), 1.);
	}

	#[test]
	fn test_piecewise_linear_spectrum() {
		let spec = PiecewiseLinearSpectrum::new(&[360., 830.], &[0., 1.]);
		assert_eq!(spec.eval(360.), 0.);
		assert_eq!(spec.eval(830.), 1.);
		assert_eq!(spec.eval(595.), 0.5);
		assert_eq!(spec.eval(359.), 0.);
		assert_eq!(spec.eval(831.), 0.);
		assert_eq!(spec.max_value(), 1.);
	}
}
