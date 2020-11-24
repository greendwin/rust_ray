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
    fn hit(&self, ray: &Ray, hit: &Hit) -> HitResult {
        let norm_dir = ray.dir.norm();
        let scale = norm_dir.dot(hit.norm).abs().powi(2);

        if random() > scale {
            // pass through
            let next = hit.pt + norm_dir * 0.1;
            return HitResult::scatter(Ray::new(next, ray.dir), Vec3::new(1, 1, 1));
        }

        HitResult::glow(self.color * scale)
    }
}
