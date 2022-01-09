use image::{RgbImage, Rgb};

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> GenericResult<()> {
    let mut img = RgbImage::new(255, 255);
    for x in 0..img.width() {
        for y in 0..img.height() {
            let r = f64::from(x) * 255.0 / f64::from(img.width());
            let g = f64::from(img.height() - y) * 255.0 / f64::from(img.height());
            let b = 0.25f64 * 255.0f64;
            img.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }
    img.save("foo.png")?;
    Ok(())
}
