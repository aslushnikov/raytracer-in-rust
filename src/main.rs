use image::{RgbImage, Rgb};
use indicatif::ProgressBar;

mod vec3;
mod ray;
mod color;

use self::vec3::*;
use self::color::*;
use self::ray::*;


type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn new(ray: &Ray, p: Point3, t: f64, normal: Vec3) -> HitRecord {
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

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = vec3::dot(oc, ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if (discriminant < 0.0) {
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

fn hit_list<T>(ray: &Ray, hittables: impl Iterator<Item = T>, t_min: f64, t_max: f64) -> Option<HitRecord>
    where T: Hittable {
    let mut hit_record = None;
    let mut t_max = t_max;
    for hittable in hittables {
        if let Some(result) = hittable.hit(ray, t_min, t_max) {
            t_max = result.t;
            hit_record = Some(result);
        }
    }
    hit_record
}

fn ray_color(ray: &Ray) -> Color {
    let s = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    match s.hit(ray, 0.0, 100.0) {
        None => {
            let unit_direction = vec3::unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        },
        Some(hit_record) => {
            return 0.5 * Color::new(hit_record.normal.x + 1.0, hit_record.normal.y + 1.0, hit_record.normal.z + 1.0);
        }
    }
}

fn main() -> GenericResult<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0- Vec3::new(0.0, 0.0, focal_length);

    // Render
    dbg!(image_width);
    dbg!(image_height);

    let mut img = RgbImage::new(image_width, image_height);
    let progress = ProgressBar::new(img.height().into());
    for y in 0..img.height() {
        progress.inc(1);
        for x in 0..img.width() {
            let ray = Ray {
                origin,
                direction: lower_left_corner + vertical * y as f64 / img.height() as f64 + horizontal * x as f64 / img.width() as f64,
            };
            img.put_pixel(x, img.height() - y - 1, ray_color(&ray).into());
        }
    }
    img.save("foo.jpg")?;
    Ok(())
}

#[test]
fn test_dot() {
    let a = Vec3::new(10f64, 10f64, 10f64);
    let b = Vec3::new(12f64, 12f64, 12f64);
    dbg!(a.len());
    dbg!(vec3::dot(a, b));
    dbg!(a + b);
}
