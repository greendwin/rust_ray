use crate::math::*;

use super::dielectric::DielectricMat;
use super::diffuse::DiffuseMat;
use super::metal::MetalMat;

#[derive(Debug, Clone)]
pub enum SomeMaterial {
    Diff(DiffuseMat),
    Metal(MetalMat),
    Di(DielectricMat),
}

use SomeMaterial::*;

impl From<DiffuseMat> for SomeMaterial {
    fn from(mat: DiffuseMat) -> Self {
        Diff(mat)
    }
}

impl From<DielectricMat> for SomeMaterial {
    fn from(mat: DielectricMat) -> Self {
        Di(mat)
    }
}

impl From<MetalMat> for SomeMaterial {
    fn from(mat: MetalMat) -> Self {
        Metal(mat)
    }
}

impl Material for SomeMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)> {
        match self {
            Diff(mat) => mat.scatter(ray_in, hit),
            Di(mat) => mat.scatter(ray_in, hit),
            Metal(mat) => mat.scatter(ray_in, hit),
        }
    }
}