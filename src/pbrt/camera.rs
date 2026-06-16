use super::{
	film::Film,
	math::{lerp, sample_uniform_disk_concentric},
	medium::Medium,
	options::{OPTIONS, RenderingCoordinateSystem},
	ray::{Ray, RayDifferential},
	spectrum::{SampledSpectrum, SampledWavelengths},
	transform::{AnimatedTransform, Transform},
	vecmath::{Bounds2f, Vector2f, Vector3f},
};
use crate::Float;
use std::ops;

pub trait Camera {
	fn get_film(&self) -> &dyn Film;
	fn sample_time(&self, u: Float) -> Float;
	// TODO:
	// fn init_metadata(&self, metadata: &mut ImageMetadata);
	fn get_camera_transform(&self) -> &CameraTransform;

	fn generate_ray(
		&self,
		sample: CameraSample,
		lambda: &mut SampledWavelengths,
	) -> Option<CameraRay>;

	fn generate_ray_differential(
		&self,
		sample: CameraSample,
		lambda: &mut SampledWavelengths,
	) -> Option<CameraRayDifferential>;
}

#[derive(Debug, Clone, Copy)]
pub struct CameraSample {
	pub p_film: Vector2f,
	pub p_lens: Vector2f,
	pub time: Float,
	pub filter_weight: Float,
}

#[derive(Debug)]
pub struct CameraRay {
	pub ray: Ray,
	pub weight: SampledSpectrum,
}

impl CameraRay {
	pub fn new(ray: Ray) -> Self {
		Self { ray, weight: SampledSpectrum::new_with_const(1.) }
	}
}

#[derive(Debug)]
pub struct CameraRayDifferential {
	pub ray: RayDifferential,
	pub weight: SampledSpectrum,
}

impl CameraRayDifferential {
	pub fn new(ray: RayDifferential) -> Self {
		Self { ray, weight: SampledSpectrum::new_with_const(1.) }
	}
}

#[derive(Debug)]
pub struct CameraTransform {
	render_from_camera: AnimatedTransform,
	world_from_render: Transform,
}

impl CameraTransform {
	pub fn new(world_from_camera: &AnimatedTransform) -> Self {
		let world_from_render = unsafe {
			match OPTIONS.rendering_space {
				RenderingCoordinateSystem::Camera => {
					todo!()
				}
				RenderingCoordinateSystem::CameraWorld => {
					todo!()
				}
				RenderingCoordinateSystem::World => Transform::default(),
			}
		};

		let render_from_world = world_from_render.inv().unwrap();
		let (a, b) = (
			&render_from_world * &world_from_camera.start_transform,
			&render_from_world * &world_from_camera.end_transform,
		);
		let render_from_camera =
			AnimatedTransform::new(a, world_from_camera.start_time, b, world_from_camera.end_time);

		Self { render_from_camera, world_from_render }
	}
}

#[derive(Debug)]
pub struct CameraBase<'a> {
	camera_transform: CameraTransform,
	shutter_open: Float,
	shutter_close: Float,
	film: &'a dyn Film,
	medium: Medium,
	min_pos_differential_x: Vector3f,
	min_pos_differential_y: Vector3f,
	min_dir_differential_x: Vector3f,
	min_dir_differential_y: Vector3f,
}

#[derive(Debug)]
pub struct CameraBaseParameters<'a> {
	pub camera_transform: CameraTransform,
	pub shutter_open: Float,
	pub shutter_close: Float,
	pub film: &'a dyn Film,
	pub medium: Medium,
}

impl<'a> CameraBase<'a> {
	pub fn new(p: CameraBaseParameters<'a>) -> Self {
		Self {
			camera_transform: p.camera_transform,
			shutter_open: p.shutter_open,
			shutter_close: p.shutter_close,
			film: p.film,
			medium: p.medium,
			min_pos_differential_x: Vector3f::default(),
			min_pos_differential_y: Vector3f::default(),
			min_dir_differential_x: Vector3f::default(),
			min_dir_differential_y: Vector3f::default(),
		}
	}

	fn generate_ray_differential(
		camera: &dyn Camera,
		sample: CameraSample,
		lambda: &mut SampledWavelengths,
	) -> Option<CameraRayDifferential> {
		let cr = camera.generate_ray(sample, lambda)?;
		let mut rd = RayDifferential::from(cr.ray);
		let mut rx: Option<CameraRay> = None;
		let mut ry: Option<CameraRay> = None;

		for eps in [0.05, -0.05] {
			let mut sshift = sample;
			sshift.p_film.x += eps;
			rx = camera.generate_ray(sshift, lambda);
			if let Some(rx) = &rx {
				// Q: Why divide by eps?
				rd.rx_origin = rd.o + (rx.ray.o - rd.o) / eps;
				rd.rx_direction = rd.d + (rx.ray.d - rd.d) / eps;
				break;
			}
		}
		for eps in [0.05, -0.05] {
			let mut sshift = sample;
			sshift.p_film.y += eps;
			ry = camera.generate_ray(sshift, lambda);
			if let Some(ry) = &ry {
				rd.ry_origin = rd.ray.o + (ry.ray.o - rd.o) / eps;
				rd.ry_direction = rd.ray.d + (ry.ray.d - rd.d) / eps;
				break;
			}
		}

		rd.has_differentials = rx.is_some() && ry.is_some();

		Some(CameraRayDifferential { ray: rd, weight: cr.weight })
	}

	fn render_from_camera_ray(&self, _ray: &Ray) -> Ray {
		// self.camera_transform.render_from_camera(ray)
		todo!()
	}

	fn render_from_camera_ray_differential(&self, _ray: &RayDifferential) -> RayDifferential {
		// self.camera_transform.render_from_camera(ray)
		todo!()
	}

	// TODO: other protected methods
}

impl<'a> Camera for CameraBase<'a> {
	fn get_film(&self) -> &dyn Film {
		self.film as &dyn Film
	}

	fn get_camera_transform(&self) -> &CameraTransform {
		&self.camera_transform
	}

	fn sample_time(&self, u: Float) -> Float {
		lerp(u, self.shutter_open, self.shutter_close)
	}

	fn generate_ray(
		&self,
		_sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRay> {
		unimplemented!()
	}

	fn generate_ray_differential(
		&self,
		_sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRayDifferential> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct ProjectiveCamera<'a> {
	base: CameraBase<'a>,
	screen_from_camera: Transform,
	camera_from_raster: Transform,
	raster_from_screen: Transform,
	screen_from_raster: Transform,
	lens_radius: Float,
	focal_distance: Float,
}

impl<'a> ProjectiveCamera<'a> {
	pub fn new(
		base_parameters: CameraBaseParameters<'a>,
		screen_from_camera: &Transform,
		screen_window: Bounds2f,
		len_radius: Float,
		focal_distance: Float,
	) -> Self {
		let base = CameraBase::new(base_parameters);
		let ndc_from_screen =
			Transform::scale(
				1. / (screen_window.max.x - screen_window.min.x),
				1. / (screen_window.max.y - screen_window.min.y),
				1.,
			) * Transform::translate(Vector3f::new(-screen_window.min.x, -screen_window.min.y, 0.));
		let raster_from_ndc = Transform::scale(
			base.film.full_resolution().x as Float,
			-base.film.full_resolution().y as Float,
			1.,
		);
		let raster_from_screen = raster_from_ndc * ndc_from_screen;
		let screen_from_raster = raster_from_screen.inv().unwrap();
		let camera_from_raster = screen_from_camera.inv().unwrap() * &screen_from_raster;

		Self {
			base,
			screen_from_camera: screen_from_camera.clone(),
			camera_from_raster,
			raster_from_screen,
			screen_from_raster,
			lens_radius: len_radius,
			focal_distance,
		}
	}
}

impl<'a> ops::Deref for ProjectiveCamera<'a> {
	type Target = CameraBase<'a>;
	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'a> ops::DerefMut for ProjectiveCamera<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}

#[derive(Debug)]
pub struct OrthographicCamera<'a> {
	proj: ProjectiveCamera<'a>,
	dx_camera: Vector3f,
	dy_camera: Vector3f,
}

impl<'a> OrthographicCamera<'a> {
	pub fn new(
		base_parameters: CameraBaseParameters<'a>,
		screen_window: Bounds2f,
		len_radius: Float,
		focal_distance: Float,
	) -> Self {
		let mut proj = ProjectiveCamera::new(
			base_parameters,
			&Transform::orthographic(0., 1.),
			screen_window,
			len_radius,
			focal_distance,
		);
		let dx_camera = proj.camera_from_raster.map_vector(Vector3f::new(1., 0., 0.));
		let dy_camera = proj.camera_from_raster.map_vector(Vector3f::new(0., 1., 0.));
		proj.min_pos_differential_x = dx_camera;
		proj.min_pos_differential_y = dy_camera;

		Self { proj, dx_camera, dy_camera }
	}
}

impl<'a> Camera for OrthographicCamera<'a> {
	fn get_film(&self) -> &dyn Film {
		self.proj.get_film()
	}

	fn get_camera_transform(&self) -> &CameraTransform {
		self.proj.get_camera_transform()
	}

	fn sample_time(&self, u: Float) -> Float {
		self.proj.sample_time(u)
	}

	fn generate_ray(
		&self,
		sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRay> {
		let p_film = Vector3f::new(sample.p_film.x, sample.p_film.y, 0.);
		let p_camera = self.camera_from_raster.map_point(p_film);
		let mut ray = Ray::new(
			p_camera,
			Vector3f::new(0., 0., 1.),
			self.sample_time(sample.time),
			self.medium.clone(),
		);
		// modify ray for depth of field
		if self.lens_radius > 0. {
			let p_lens = self.lens_radius * sample_uniform_disk_concentric(sample.p_lens);
			let ft = self.focal_distance / ray.d.z;
			let p_focus = ray.eval(ft);
			ray.o = Vector3f::new(p_lens.x, p_lens.y, 0.);
			ray.d = (p_focus - ray.o).normalized();
		}

		Some(CameraRay::new(self.render_from_camera_ray(&ray)))
	}

	fn generate_ray_differential(
		&self,
		sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRayDifferential> {
		let p_film = Vector3f::new(sample.p_film.x, sample.p_film.y, 0.);
		let p_camera = self.camera_from_raster.map_point(p_film);
		let mut ray = RayDifferential::new(
			p_camera,
			Vector3f::new(0., 0., 1.),
			self.sample_time(sample.time),
			self.base.medium.clone(),
		);
		if self.lens_radius > 0. {
			// modify ray for depth of field
			let p_lens = self.lens_radius * sample_uniform_disk_concentric(sample.p_lens);
			let ft = self.focal_distance / ray.d.z;
			let p_focus = ray.eval(ft);
			ray.o = Vector3f::new(p_lens.x, p_lens.y, 0.);
			ray.d = (p_focus - ray.o).normalized();

			// compute ray differentials
			let p_focus = p_camera + self.dx_camera + (ft * Vector3f::new(0., 0., 1.));
			ray.rx_origin = Vector3f::new(p_lens.x, p_lens.y, 0.);
			ray.rx_direction = (p_focus - ray.rx_origin).normalized();
			let p_focus = p_camera + self.dy_camera + (ft * Vector3f::new(0., 0., 1.));
			ray.ry_origin = Vector3f::new(p_lens.x, p_lens.y, 0.);
			ray.ry_direction = (p_focus - ray.rx_origin).normalized();
		} else {
			ray.rx_origin = ray.o + self.dx_camera;
			ray.ry_origin = ray.o + self.dy_camera;
			ray.rx_direction = ray.d;
			ray.ry_direction = ray.d;
		}

		Some(CameraRayDifferential::new(self.render_from_camera_ray_differential(&ray)))
	}
}

impl<'a> ops::Deref for OrthographicCamera<'a> {
	type Target = ProjectiveCamera<'a>;
	fn deref(&self) -> &Self::Target {
		&self.proj
	}
}

impl<'a> ops::DerefMut for OrthographicCamera<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.proj
	}
}

#[derive(Debug)]
pub struct PerspectiveCamera<'a> {
	proj: ProjectiveCamera<'a>,
	dx_camera: Vector3f,
	dy_camera: Vector3f,
	cos_total_width: Float,
}

impl<'a> PerspectiveCamera<'a> {
	pub fn new(
		base_parameters: CameraBaseParameters<'a>,
		fov: Float,
		screen_window: Bounds2f,
		len_radius: Float,
		focal_distance: Float,
	) -> Self {
		let proj = ProjectiveCamera::new(
			base_parameters,
			&Transform::perspective(fov, 1e-2, 1000.),
			screen_window,
			len_radius,
			focal_distance,
		);
		let dx_camera = proj.camera_from_raster.map_point(Vector3f::new(1., 0., 0.))
			- proj.camera_from_raster.map_point(Vector3f::default());
		let dy_camera = proj.camera_from_raster.map_point(Vector3f::new(0., 1., 0.))
			- proj.camera_from_raster.map_point(Vector3f::default());

		// compute cos_total_width
		let radius = proj.film.get_filter().radius();
		let p_corner = Vector3f::new(-radius.x, -radius.y, 0.);
		let w_corner_camera = proj.camera_from_raster.map_point(p_corner).normalized();
		let cos_total_width = w_corner_camera.z;

		todo!("compute minimum differentials")
	}
}

impl<'a> Camera for PerspectiveCamera<'a> {
	fn get_film(&self) -> &dyn Film {
		self.proj.get_film()
	}

	fn get_camera_transform(&self) -> &CameraTransform {
		self.proj.get_camera_transform()
	}

	fn sample_time(&self, u: Float) -> Float {
		self.proj.sample_time(u)
	}

	fn generate_ray(
		&self,
		sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRay> {
		let p_film = Vector3f::new(sample.p_film.x, sample.p_film.y, 0.);
		let p_camera = self.camera_from_raster.map_point(p_film);
		let mut ray = Ray::new(
			Vector3f::default(),
			p_camera.normalized(),
			self.sample_time(sample.time),
			self.medium.clone(),
		);
		// modify ray for depth of field
		if self.lens_radius > 0. {
			let p_lens = self.lens_radius * sample_uniform_disk_concentric(sample.p_lens);
			let ft = self.focal_distance / ray.d.z;
			let p_focus = ray.eval(ft);
			ray.o = Vector3f::new(p_lens.x, p_lens.y, 0.);
			ray.d = (p_focus - ray.o).normalized();
		}

		Some(CameraRay::new(self.render_from_camera_ray(&ray)))
	}

	fn generate_ray_differential(
		&self,
		sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRayDifferential> {
		let p_film = Vector3f::new(sample.p_film.x, sample.p_film.y, 0.);
		let p_camera = self.camera_from_raster.map_point(p_film);
		let mut ray = RayDifferential::new(
			Vector3f::default(),
			p_camera.normalized(),
			self.sample_time(sample.time),
			self.medium.clone(),
		);
		if self.lens_radius > 0. {
			// modify ray for depth of field
			let p_lens = self.lens_radius * sample_uniform_disk_concentric(sample.p_lens);
			let ft = self.focal_distance / ray.d.z;
			let p_focus = ray.eval(ft);
			ray.o = Vector3f::new(p_lens.x, p_lens.y, 0.);
			ray.d = (p_focus - ray.o).normalized();

			// compute ray differentials
			let dx = (p_camera + self.dx_camera).normalized();
			let ft = self.focal_distance / dx.z;
			let p_focus = ft * dx;
			ray.rx_origin = ray.o;
			ray.rx_direction = (p_focus - ray.rx_origin).normalized();
			let dy = (p_camera + self.dy_camera).normalized();
			let ft = self.focal_distance / dy.z;
			let p_focus = ft * dy;
			ray.ry_origin = ray.o;
			ray.ry_direction = (p_focus - ray.ry_origin).normalized();
		}

		Some(CameraRayDifferential::new(self.render_from_camera_ray_differential(&ray)))
	}
}

impl<'a> ops::Deref for PerspectiveCamera<'a> {
	type Target = ProjectiveCamera<'a>;
	fn deref(&self) -> &Self::Target {
		&self.proj
	}
}

impl<'a> ops::DerefMut for PerspectiveCamera<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.proj
	}
}
