use crate::math::*;
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct GlowMat {
    pub color: Vec3,
}

impl GlowMat {
    pub fn new(color: impl Into<Vec3>) -> Self {
        Self {
            color: color.into(),
        }
    }
}

impl Material for GlowMat {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> ScatterResult {
        let norm_dir = ray_in.dir.norm();
        let scale = norm_dir.dot(hit.norm).abs().powi(2);

        if random() > scale {
            // pass through
            let next = hit.pt + norm_dir * 0.1;
            return ScatterResult::scatter(
                Ray::new(next, ray_in.dir, ray_in.time),
                Vec3::new(1, 1, 1),
                0.0,
            );
        }

        ScatterResult::glow(self.color * scale)
    }
}
