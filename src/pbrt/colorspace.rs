use super::{
	color::{RGB, RGBSigmoidPolynomial, RGBToSpectrumTable, XYZ},
	spectrum::{DenselySampledSpectrum, Spectrum, get_named_spectrum},
	transform::SMatrix,
	vecmath::Vec2f,
};
use std::sync::LazyLock;

// TODO: sRGB, DCI_P3, Rec2020, ACES2065_1
#[allow(non_upper_case_globals)]
pub static sRGB: LazyLock<RGBColorSpace> = LazyLock::new(|| {
	RGBColorSpace::new(
		Vec2f::new(0.64, 0.33),
		Vec2f::new(0.3, 0.6),
		Vec2f::new(0.15, 0.06),
		get_named_spectrum("stdillum-D65").unwrap(),
		&RGBToSpectrumTable::sRGB,
	)
});
// P3-D65 (display)
pub static DCI_P3: LazyLock<RGBColorSpace> = LazyLock::new(|| {
	RGBColorSpace::new(
		Vec2f::new(0.68, 0.32),
		Vec2f::new(0.265, 0.690),
		Vec2f::new(0.15, 0.06),
		get_named_spectrum("stdillum-D65").unwrap(),
		&RGBToSpectrumTable::DCI_P3,
	)
});
// ITU-R Rec BT.2020
#[allow(non_upper_case_globals)]
pub static Rec2020: LazyLock<RGBColorSpace> = LazyLock::new(|| {
	RGBColorSpace::new(
		Vec2f::new(0.708, 0.292),
		Vec2f::new(0.170, 0.797),
		Vec2f::new(0.131, 0.046),
		get_named_spectrum("stdillum-D65").unwrap(),
		&RGBToSpectrumTable::Rec2020,
	)
});
pub static ACES2065_1: LazyLock<RGBColorSpace> = LazyLock::new(|| {
	RGBColorSpace::new(
		Vec2f::new(0.7347, 0.2653),
		Vec2f::new(0., 1.),
		Vec2f::new(0.0001, -0.077),
		get_named_spectrum("illum-acesD60").unwrap(),
		&RGBToSpectrumTable::ACES2065_1,
	)
});

#[derive(Debug, Clone, PartialEq)]
pub struct RGBColorSpace<'a> {
	pub r: Vec2f,
	pub g: Vec2f,
	pub b: Vec2f,
	pub w: Vec2f,
	pub illuminant: DenselySampledSpectrum,
	pub rgb2spec: &'a RGBToSpectrumTable<'a>,
	pub rgb2xyz: SMatrix<3>,
	pub xyz2rgb: SMatrix<3>,
}

impl<'a> RGBColorSpace<'a> {
	pub fn new(
		r: Vec2f,
		g: Vec2f,
		b: Vec2f,
		illuminant: &dyn Spectrum,
		rgb2spec: &'a RGBToSpectrumTable,
	) -> Self {
		#[allow(non_snake_case)]
		let W = XYZ::from(illuminant);
		let w = W.xy();
		#[allow(non_snake_case)]
		let (R, G, B) = (XYZ::from_xy(r), XYZ::from_xy(g), XYZ::from_xy(b));
		let rgb = SMatrix::from([[R.x, G.x, B.x], [R.y, G.y, B.y], [R.z, G.z, B.z]]);
		#[allow(non_snake_case)]
		let C = rgb.inv().unwrap() * W;
		let rgb2xyz = rgb * SMatrix::diag([C.x, C.y, C.z]);
		let xyz2rgb = rgb2xyz.inv().unwrap();

		Self {
			r,
			g,
			b,
			w,
			illuminant: DenselySampledSpectrum::new(illuminant),
			rgb2spec,
			rgb2xyz,
			xyz2rgb,
		}
	}

	pub fn to_rgb(&self, xyz: XYZ) -> RGB {
		self.xyz2rgb.mul(xyz)
	}

	pub fn to_xyz(&self, rgb: RGB) -> XYZ {
		self.rgb2xyz.mul(rgb)
	}

	/// Return a matrix convert color between two RGB color spaces.
	pub fn convert_rgb_color_space(from: &Self, to: &Self) -> SMatrix<3> {
		if from == to { SMatrix::default() } else { to.xyz2rgb * from.rgb2xyz }
	}

	pub fn to_rgb_coeffs(&self, rgb: RGB) -> RGBSigmoidPolynomial {
		self.rgb2spec.eval(rgb.clamp_zero())
	}
}
