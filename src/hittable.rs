use crate::vec3::{self,*};
use crate::ray::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point3, t: f64, normal: Vec3) -> HitRecord {
        let front_face = vec3::dot(ray.direction, normal) < 0.0;
        let normal = if !front_face { normal * -1.0 } else { normal };
        HitRecord {
            p,
            t,
            normal,
            front_face
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

