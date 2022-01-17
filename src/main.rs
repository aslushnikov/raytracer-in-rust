use image::{RgbImage};
use indicatif::ProgressBar;
use rand::prelude::*;
use rayon::prelude::*;

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
const SAMPLES_PER_PIXEL: u32 = 10;
const MAX_RAY_REFLECTION: usize = 10;

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

fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let v = random_unit_vector();

    if vec3::dot(&v, normal) > 0.0 {
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

fn ray_color<T: Hittable>(ray: &Ray, world: &Vec<T>, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match hit_list(ray, world.iter(), 0.001, 100.0) {
        None => {
            let unit_direction = vec3::unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        },
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + random_in_hemisphere(&hit_record.normal);
            let child_ray = Ray {
                origin: hit_record.p,
                direction: target - hit_record.p,
            };
            return 0.5 * ray_color::<T>(&child_ray, world, depth - 1);
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
    let progress = ProgressBar::new((img.height() * img.width()).into());

    let result = (0..(img.height() * img.width())).into_par_iter().map(|idx| {
        let y = idx / img.width();
        let x = idx % img.width();
        progress.inc(1);
        let mut color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f64 + rand::random::<f64>()) / img.width() as f64;
            let v = (y as f64 + rand::random::<f64>()) / img.height() as f64;
            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, MAX_RAY_REFLECTION);
        }
        (x, img.height() - y - 1, (color / SAMPLES_PER_PIXEL as f64).into())
    }).collect::<Vec<(u32, u32, image::Rgb<u8>)>>();
    for (x, y, color) in result {
        img.put_pixel(x, y, color);
    };
    img.save("foo.png")?;
    Ok(())
}

