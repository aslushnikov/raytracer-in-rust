use super::vec3::{Vec3, Point3, dot};
use image::Rgb;

pub type Color = Vec3;

impl std::convert::From<Color> for Rgb<u8> {
    fn from(value: Color) -> Rgb<u8> {
        Rgb([
            (value.x.sqrt().clamp(0.0, 1.0) * 255.0) as u8,
            (value.y.sqrt().clamp(0.0, 1.0) * 255.0) as u8,
            (value.z.sqrt().clamp(0.0, 1.0) * 255.0) as u8,
        ])
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub trait Geometry {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait Material {
    fn scatter(&self, hit_record: &HitRecord) -> Ray;
}

pub struct Object<G: Geometry, M: Material> {
    pub geometry: G,
    pub material: M,
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point3, t: f64, normal: Vec3) -> HitRecord {
        let front_face = dot(&ray.direction, &normal) < 0.0;
        let normal = if !front_face { normal * -1.0 } else { normal };
        HitRecord {
            p,
            t,
            normal,
            front_face
        }
    }
}


