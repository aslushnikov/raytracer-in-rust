use image::{RgbImage};
use indicatif::ProgressBar;
use rayon::prelude::*;

mod base;
mod vec3;
mod shapes;
mod camera;

use self::vec3::*;
use self::base::*;
use self::shapes::*;
use self::camera::*;

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;
const SAMPLES_PER_PIXEL: u32 = 10;
const MAX_RAY_REFLECTION: usize = 10;

pub fn hit_list<'a, G: 'a + Geometry, M: 'a + Material>(ray: &Ray, objects: impl Iterator<Item = &'a Object<G, M>>, t_min: f64, t_max: f64) -> Option<(&'a Object<G, M>, HitRecord)> {
    let mut hit_record = None;
    let mut t_max = t_max;
    for object in objects {
        if let Some(result) = object.geometry.hit(ray, t_min, t_max) {
            t_max = result.t;
            hit_record = Some((object, result));
        }
    }
    hit_record
}

fn ray_color<G: Geometry, M: Material>(ray: &Ray, world: &Vec<Object<G, M>>, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match hit_list(ray, world.iter(), 0.001, 100.0) {
        None => {
            let unit_direction = vec3::unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        },
        Some((object, hit_record)) => {
            if let Some((child_ray, attenuation)) = object.material.scatter(&ray, &hit_record) {
                return vec3::hadamard(&attenuation, &ray_color::<G, M>(&child_ray, world, depth - 1));
            }
            return Color::new(0.0, 0.0, 0.0);
        }
    }
}

fn main() -> GenericResult<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // World
    let material_ground = MetalMaterial { albedo: Color::new(0.8, 0.8, 0.0) };
    let material_center = MetalMaterial { albedo: Color::new(0.7, 0.3, 0.3) };
    let material_left = MetalMaterial { albedo: Color::new(0.8, 0.8, 0.8) };
    let material_right = MetalMaterial { albedo: Color::new(0.8, 0.6, 0.2) };

    let world = vec![
        Object {
            geometry: Sphere {
                center: Point3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            },
            material: material_ground,
        },
        Object {
            geometry: Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            },
            material: material_center,
        },
        Object {
            geometry: Sphere {
                center: Point3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
            },
            material: material_left,
        },
        Object {
            geometry: Sphere {
                center: Point3::new(1.0, 0.0, -1.0),
                radius: 0.5,
            },
            material: material_right,
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

