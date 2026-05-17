use image::{GenericImageView, Pixel};

pub fn comparer_images(path1: &str, path2: &str) -> Result<f64, Box<dyn std::error::Error>> {

    let img1 = image::open(path1)?;
    let img2 = image::open(path2)?;
    let img2 = img2.resize_exact(
        img1.width(),
        img1.height(),
        image::imageops::FilterType::Nearest,
    );
    let mut difference_totale: f64 = 0.0;
    let total_pixels = (img1.width() * img1.height() * 3) as f64;
    for y in 0..img1.height() {
        for x in 0..img1.width() {
            let p1 = img1.get_pixel(x, y).to_rgb();
            let p2 = img2.get_pixel(x, y).to_rgb();

            for i in 0..3 {
                difference_totale += (p1[i] as f64 - p2[i] as f64).abs();
            }
        }
    }
    let difference_max = 255.0 * total_pixels;
    let similarite = 100.0 - ((difference_totale / difference_max) * 100.0);
    Ok(similarite)
}
