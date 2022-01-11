use image::{RgbImage};
use indicatif::ProgressBar;

mod vec3;
mod ray;
mod color;
mod hittable;
mod shapes;
mod camera;

use self::vec3::*;
use self::color::*;
use self::ray::*;
use self::hittable::*;
use self::shapes::*;
use self::camera::*;

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn hit_list<'a, T: 'a + Hittable>(ray: &Ray, hittables: impl Iterator<Item = &'a T>, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

fn ray_color<T: Hittable>(ray: &Ray, world: &Vec<T>) -> Color {
    match hit_list(ray, world.iter(), 0.0, 100.0) {
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

    // World
    let world = vec![
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    // Camera
    let camera = Camera::new(aspect_ratio);

    // Render
    dbg!(image_width);
    dbg!(image_height);

    let mut img = RgbImage::new(image_width, image_height);
    let progress = ProgressBar::new(img.height().into());
    for y in 0..img.height() {
        progress.inc(1);
        for x in 0..img.width() {
            let ray = camera.get_ray(x as f64 / img.width() as f64, y as f64 / img.height() as f64);
            img.put_pixel(x, img.height() - y - 1, ray_color(&ray, &world).into());
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
