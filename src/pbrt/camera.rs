use super::{
	film::Film,
	math::{lerp, sample_uniform_disk_concentric},
	options::{OPTIONS, RenderingCoordinateSystem},
	ray::{Ray, RayDifferential},
	spectrum::{SampledSpectrum, SampledWavelengths},
	transform::{AnimatedTransform, Transform},
	vecmath::{Bounds2f, Vector2f, Vector3f},
};
use std::ops::{Deref, DerefMut};

pub trait Camera {
	fn get_film(&self) -> &dyn Film;
	fn sample_time(&self, u: f32) -> f32;
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
	pub time: f32,
	pub filter_weight: f32,
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
	#[allow(dead_code)]
	render_from_camera: AnimatedTransform,
	#[allow(dead_code)]
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
			world_from_camera.end_transform.as_ref().map(|t| &render_from_world * t),
		);
		let render_from_camera =
			AnimatedTransform::new(a, world_from_camera.start_time, b, world_from_camera.end_time);

		Self { render_from_camera, world_from_render }
	}
}

#[derive(Debug)]
pub struct CameraBase {
	camera_transform: CameraTransform,
	shutter_open: f32,
	shutter_close: f32,
	film: Box<dyn Film>,
	min_pos_differential_x: Vector3f,
	min_pos_differential_y: Vector3f,
	#[allow(dead_code)]
	min_dir_differential_x: Vector3f,
	#[allow(dead_code)]
	min_dir_differential_y: Vector3f,
}

#[derive(Debug)]
pub struct CameraBaseParameters {
	pub camera_transform: CameraTransform,
	pub shutter_open: f32,
	pub shutter_close: f32,
	pub film: Box<dyn Film>,
}

impl CameraBase {
	pub fn new(p: CameraBaseParameters) -> Self {
		Self {
			camera_transform: p.camera_transform,
			shutter_open: p.shutter_open,
			shutter_close: p.shutter_close,
			film: p.film,
			min_pos_differential_x: Vector3f::default(),
			min_pos_differential_y: Vector3f::default(),
			min_dir_differential_x: Vector3f::default(),
			min_dir_differential_y: Vector3f::default(),
		}
	}

	#[allow(dead_code)]
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

impl Camera for CameraBase {
	fn get_film(&self) -> &dyn Film {
		self.film.as_ref()
	}

	fn get_camera_transform(&self) -> &CameraTransform {
		&self.camera_transform
	}

	fn sample_time(&self, t: f32) -> f32 {
		lerp(self.shutter_open, self.shutter_close, t)
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
pub struct ProjectiveCamera {
	base: CameraBase,
	#[allow(dead_code)]
	screen_from_camera: Transform,
	camera_from_raster: Transform,
	#[allow(dead_code)]
	raster_from_screen: Transform,
	#[allow(dead_code)]
	screen_from_raster: Transform,
	lens_radius: f32,
	focal_distance: f32,
}

impl ProjectiveCamera {
	pub fn new(
		base_parameters: CameraBaseParameters,
		screen_from_camera: &Transform,
		screen_window: Bounds2f,
		len_radius: f32,
		focal_distance: f32,
	) -> Self {
		let base = CameraBase::new(base_parameters);
		let ndc_from_screen =
			Transform::scale(
				1. / (screen_window.max.x - screen_window.min.x),
				1. / (screen_window.max.y - screen_window.min.y),
				1.,
			) * Transform::translate(Vector3f::new(-screen_window.min.x, -screen_window.min.y, 0.));
		let raster_from_ndc = Transform::scale(
			base.film.full_resolution().x as f32,
			-base.film.full_resolution().y as f32,
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

impl Deref for ProjectiveCamera {
	type Target = CameraBase;
	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl DerefMut for ProjectiveCamera {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}

#[derive(Debug)]
pub struct OrthographicCamera {
	proj: ProjectiveCamera,
	dx_camera: Vector3f,
	dy_camera: Vector3f,
}

impl OrthographicCamera {
	pub fn new(
		base_parameters: CameraBaseParameters,
		screen_window: Bounds2f,
		len_radius: f32,
		focal_distance: f32,
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

impl Camera for OrthographicCamera {
	fn get_film(&self) -> &dyn Film {
		self.proj.get_film()
	}

	fn get_camera_transform(&self) -> &CameraTransform {
		self.proj.get_camera_transform()
	}

	fn sample_time(&self, u: f32) -> f32 {
		self.proj.sample_time(u)
	}

	fn generate_ray(
		&self,
		sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRay> {
		let p_film = Vector3f::new(sample.p_film.x, sample.p_film.y, 0.);
		let p_camera = self.camera_from_raster.map_point(p_film);
		let mut ray = Ray::new(p_camera, Vector3f::new(0., 0., 1.), self.sample_time(sample.time));
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

impl Deref for OrthographicCamera {
	type Target = ProjectiveCamera;
	fn deref(&self) -> &Self::Target {
		&self.proj
	}
}

impl DerefMut for OrthographicCamera {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.proj
	}
}

#[derive(Debug)]
pub struct PerspectiveCamera {
	proj: ProjectiveCamera,
	dx_camera: Vector3f,
	dy_camera: Vector3f,
	// cos_total_width: f32,
}

impl PerspectiveCamera {
	pub fn new(
		base_parameters: CameraBaseParameters,
		fov: f32,
		screen_window: Bounds2f,
		len_radius: f32,
		focal_distance: f32,
	) -> Self {
		let proj = ProjectiveCamera::new(
			base_parameters,
			&Transform::perspective(fov, 1e-2, 1000.),
			screen_window,
			len_radius,
			focal_distance,
		);
		let _dx_camera = proj.camera_from_raster.map_point(Vector3f::new(1., 0., 0.))
			- proj.camera_from_raster.map_point(Vector3f::default());
		let _dy_camera = proj.camera_from_raster.map_point(Vector3f::new(0., 1., 0.))
			- proj.camera_from_raster.map_point(Vector3f::default());

		todo!("compute minimum differentials")
	}
}

impl Camera for PerspectiveCamera {
	fn get_film(&self) -> &dyn Film {
		self.proj.get_film()
	}

	fn get_camera_transform(&self) -> &CameraTransform {
		self.proj.get_camera_transform()
	}

	fn sample_time(&self, u: f32) -> f32 {
		self.proj.sample_time(u)
	}

	fn generate_ray(
		&self,
		sample: CameraSample,
		_lambda: &mut SampledWavelengths,
	) -> Option<CameraRay> {
		let p_film = Vector3f::new(sample.p_film.x, sample.p_film.y, 0.);
		let p_camera = self.camera_from_raster.map_point(p_film);
		let mut ray =
			Ray::new(Vector3f::default(), p_camera.normalized(), self.sample_time(sample.time));
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

impl Deref for PerspectiveCamera {
	type Target = ProjectiveCamera;
	fn deref(&self) -> &Self::Target {
		&self.proj
	}
}

impl DerefMut for PerspectiveCamera {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.proj
	}
}
