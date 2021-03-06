use super::vec3::{self,*};
use super::base::*;
use rand::prelude::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = vec3::dot(oc, ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::new(ray, p, root, normal))
    }
}

pub struct LambertianMaterial {
    pub albedo: Color,
}

impl Material for LambertianMaterial {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let direction = hit_record.normal + random_in_hemisphere(hit_record.normal);
        Some((Ray {
            origin: hit_record.p,
            direction: direction,
        }, self.albedo))
    }
}

pub struct MetalMaterial {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        //TODO probably no need to do unit_vector anywhere here.
        let direction = reflect(unit_vector(ray.direction), hit_record.normal) + random_in_unit_sphere() * self.fuzz;
        if vec3::dot(direction, hit_record.normal) <= 0.0 {
            return None
        }
        Some((Ray {
            origin: hit_record.p,
            direction: direction,
        }, self.albedo))
    }
}

pub struct DielectricMaterial {
    pub ir: f64, // index of refraction
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit_record.front_face { 1.0/self.ir } else { self.ir };
        let unit_direction = unit_vector(ray.direction);

        let cos_theta = f64::min(vec3::dot(-1.0 * unit_direction, hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some((Ray {
            origin: hit_record.p,
            direction,
        }, Color::new(1.0, 1.0, 1.0)))
    }
}

fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let v = random_unit_vector();

    if vec3::dot(v, normal) > 0.0 {
        v
    } else {
        v * -1.0
    }
}

fn random_unit_vector() -> Vec3 {
    vec3::unit_vector(random_in_unit_sphere())
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let x = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if x.len_squared() < 1.0 {
            return x;
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * vec3::dot(v, n) * n
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(vec3::dot(-1.0f64 * uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0+ ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

