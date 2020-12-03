use super::*;
use crate::math::*;

#[derive(Debug, Copy, Clone, From)]
pub enum SomeMaterial {
    Diff(DiffuseMat),
    Metal(MetalMat),
    Di(DielectricMat),
    Glow(GlowMat),
}

use SomeMaterial::*;

impl Material for SomeMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> ScatterResult {
        match self {
            Diff(mat) => mat.scatter(ray_in, hit),
            Di(mat) => mat.scatter(ray_in, hit),
            Metal(mat) => mat.scatter(ray_in, hit),
            Glow(mat) => mat.scatter(ray_in, hit),
        }
    }
}
