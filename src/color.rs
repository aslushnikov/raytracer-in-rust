use super::vec3::Vec3;
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

