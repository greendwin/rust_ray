use super::{MovingSphereObject, SomeMaterial, SphereObject};
use crate::math::*;

#[derive(Debug, Clone, From, new)]
pub enum SomeObject {
    Sphere(SphereObject<SomeMaterial>),
    MovingSphere(MovingSphereObject<SomeMaterial>),
}

use SomeObject::*;

impl HitRay<SomeMaterial> for SomeObject {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, SomeMaterial)> {
        match self {
            Sphere(p) => p.hit(ray, t_min, t_max),
            MovingSphere(p) => p.hit(ray, t_min, t_max),
        }
    }
}

impl BoundBox for SomeObject {
    fn get_bounds(&self) -> AABB {
        match self {
            Sphere(p) => AABB::from(p.sphere),
            MovingSphere(p) => AABB::from_many(&[
                AABB::from(p.sphere_at(p.time0)),
                AABB::from(p.sphere_at(p.time1)),
            ]),
        }
    }
}
