use super::{
	color::{RGB, XYZ},
	spectrum::{DenselySampledSpectrum, Spectrum},
	transform::SquareMatrix,
	vecmath::Vector2f,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RGBColorSpace<'a> {
	pub r: Vector2f,
	pub g: Vector2f,
	pub b: Vector2f,
	pub w: Vector2f,
	pub illuminant: DenselySampledSpectrum,
	pub rgb2spec: &'a RGBToSpectrumTable,
	pub rgb2xyz: SquareMatrix<3>,
	pub xyz2rgb: SquareMatrix<3>,
}

impl<'a> RGBColorSpace<'a> {
	pub fn new(
		r: Vector2f,
		g: Vector2f,
		b: Vector2f,
		illuminant: &dyn Spectrum,
		rgb2spec: &'a RGBToSpectrumTable,
	) -> Self {
		#[allow(non_snake_case)]
		let W = XYZ::from(illuminant);
		let w = W.xy();
		#[allow(non_snake_case)]
		let (R, G, B) = (XYZ::from_xy(r), XYZ::from_xy(g), XYZ::from_xy(b));
		let rgb = SquareMatrix::from([[R.x, G.x, B.x], [R.y, G.y, B.y], [R.z, G.z, B.z]]);
		#[allow(non_snake_case)]
		let C = rgb.inv().unwrap() * W;
		let rgb2xyz = rgb * SquareMatrix::diag([C.x, C.y, C.z]);
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
	pub fn convert_rgb_color_space(from: &Self, to: &Self) -> SquareMatrix<3> {
		if from == to {
			SquareMatrix::default()
		} else {
			&to.xyz2rgb * &from.rgb2xyz
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct RGBToSpectrumTable;
