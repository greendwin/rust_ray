use super::camera::Camera;
use crate::math::*;
use crate::utils::*;

pub trait RenderTarget {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize, color: Vec3);
}

pub enum RenderProgress {
    InProgress(usize), // percents
    Done,
}

use RenderProgress::*;

pub struct Renderer<'a, Scene, Target> {
    samples_per_pixel: usize,
    max_depth: usize,

    camera: &'a Camera,
    scene: &'a Scene,
    target: &'a mut Target,

    // iteration data
    cur_y: usize,
    cur_samples: usize,
    accum_colors: Vec<Vec3>,
}

impl<'a, Scene, Target> Renderer<'a, Scene, Target>
where
    Scene: HitRay,
    Target: RenderTarget,
{
    pub fn new(
        samples_per_pixel: usize,
        max_depth: usize,
        camera: &'a Camera,
        scene: &'a Scene,
        target: &'a mut Target,
    ) -> Self {
        let num_pixels = target.width() * target.height();

        Self {
            samples_per_pixel,
            max_depth,
            camera,
            scene,
            target,

            // iteration
            cur_y: 0,
            cur_samples: 0,
            accum_colors: vec![Vec3::zero(); num_pixels],
        }
    }

    pub fn target_mut(&mut self) -> &mut Target {
        &mut self.target
    }

    pub fn next(&mut self) -> RenderProgress {
        if self.cur_samples == self.samples_per_pixel {
            return Done;
        }

        render_next(
            self.target.height() - self.cur_y - 1, // iterate lines top-down for better preview
            self.cur_samples,
            &mut self.accum_colors,
            self.max_depth,
            self.camera,
            self.scene,
            self.target,
        );

        self.cur_y += 1;

        if self.cur_y == self.target.height() {
            self.cur_y = 0;
            self.cur_samples += 1;
        }

        let cur_iters = self.cur_samples * self.target.height() + self.cur_y;
        let total_iters = self.samples_per_pixel * self.target.height();
        let ratio = cur_iters as f64 / total_iters as f64;
        RenderProgress::InProgress((ratio * 100.0).round() as usize)
    }
}

fn render_next(
    y: usize,
    cur_samples: usize,
    accum_colors: &mut [Vec3],
    max_depth: usize,
    camera: &Camera,
    scene: &impl HitRay,
    target: &mut impl RenderTarget,
) {
    let u_last = (target.width() - 1) as f64;
    let v_last = (target.height() - 1) as f64;

    for x in 0..target.width() {
        let accum_color = &mut accum_colors[y * target.width() + x];

        let x = x as f64;
        let y = y as f64;
        let u = inv_lerp(x + random(), 0.0, u_last);
        let v = inv_lerp(y + random(), 0.0, v_last);
        let ray = camera.get_ray(u, v);

        *accum_color += ray_color(&ray, scene, max_depth as i32);

        let color = (*accum_color / (cur_samples + 1) as f64).sqrt();
        target.set_pixel(x as usize, y as usize, color);
    }
}

fn ray_color(ray: &Ray, scene: &impl HitRay, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some((hit, mat)) = scene.hit(ray, 0.001, f64::MAX) {
        if let Some((next_ray, color)) = mat.scatter(&ray, &hit) {
            return color * ray_color(&next_ray, scene, depth - 1);
        }

        return Vec3::zero();
    }

    let norm_dir = ray.dir.norm();
    let t = 0.5 * (norm_dir.y + 1.0);
    lerp(t, Vec3::new(1, 1, 1), Vec3::new(0.5, 0.7, 1.0))
}
