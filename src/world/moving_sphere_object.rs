use crate::math::*;

#[derive(Debug, Clone, new)]
pub struct MovingSphereObject<Mat> {
    pub pos0: Vec3,
    pub pos1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Mat,
}

impl<Mat> MovingSphereObject<Mat> {
    #[inline]
    pub fn center_at(&self, time: f64) -> Vec3 {
        time.inv_lerp(self.time0, self.time1)
            .lerp(self.pos0, self.pos1)
    }

    #[inline]
    pub fn sphere_at(&self, time: f64) -> Sphere {
        Sphere::new(self.center_at(time), self.radius)
    }
}

impl<Mat> HitRay<Mat> for MovingSphereObject<Mat>
where
    Mat: Clone,
{
    #[inline]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Mat)> {
        self.sphere_at(ray.time)
            .hit(&ray, t_min, t_max)
            .map(|hit| (hit, self.material.clone()))
    }
}
